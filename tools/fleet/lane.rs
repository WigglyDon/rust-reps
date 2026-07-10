use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::{Command, Output, Stdio},
};

use crate::{context, AppResult, OutputMode};

const READ_ONLY_GIT_SUBCOMMANDS: &[&str] = &[
    "rev-parse",
    "status",
    "remote",
    "config",
    "rev-list",
    "merge-base",
    "diff",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Classification {
    Ready,
    FastForwardAvailable,
    LocalMergeRequired,
    LocalRebaseRequired,
    ContextStale,
    RemoteAuthRepairAvailable,
    RemoteAuthBlocked,
    DirtyUnknownWork,
    WrongWorktree,
    WrongBranch,
    DirectOverlap,
    RedLineConflict,
}

impl Classification {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "READY",
            Self::FastForwardAvailable => "FAST_FORWARD_AVAILABLE",
            Self::LocalMergeRequired => "LOCAL_MERGE_REQUIRED",
            Self::LocalRebaseRequired => "LOCAL_REBASE_REQUIRED",
            Self::ContextStale => "CONTEXT_STALE",
            Self::RemoteAuthRepairAvailable => "REMOTE_AUTH_REPAIR_AVAILABLE",
            Self::RemoteAuthBlocked => "REMOTE_AUTH_BLOCKED",
            Self::DirtyUnknownWork => "DIRTY_UNKNOWN_WORK",
            Self::WrongWorktree => "WRONG_WORKTREE",
            Self::WrongBranch => "WRONG_BRANCH",
            Self::DirectOverlap => "DIRECT_OVERLAP",
            Self::RedLineConflict => "RED_LINE_CONFLICT",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DoctorFacts {
    requested_worktree: String,
    repository_root: String,
    expected_branch: String,
    actual_branch: String,
    head: String,
    expected_base: String,
    dirty: bool,
    context_stale: bool,
    direct_overlap: bool,
    red_line: bool,
    ahead: Option<u64>,
    behind: Option<u64>,
    base_is_ancestor: bool,
    remote_configured: bool,
    upstream: Option<String>,
    credential_helper_configured: bool,
    forge_cli_authenticated: bool,
    context_sha: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DoctorReport {
    classification: Classification,
    facts: DoctorFacts,
}

pub(crate) fn command_doctor(
    worktree: &Path,
    expected_branch: &str,
    expected_base: &str,
    overlap_file: Option<&Path>,
    mode: OutputMode,
) -> AppResult<()> {
    let report = inspect(worktree, expected_branch, expected_base, overlap_file)?;
    print!("{}", format_report(&report, mode));
    Ok(())
}

fn inspect(
    worktree: &Path,
    expected_branch: &str,
    expected_base: &str,
    overlap_file: Option<&Path>,
) -> AppResult<DoctorReport> {
    let requested = worktree.display().to_string();
    let repository_root = git_text(worktree, &["rev-parse", "--show-toplevel"])
        .unwrap_or_else(|_| "UNAVAILABLE".to_owned());
    let actual_branch = git_text(worktree, &["branch", "--show-current"])
        .unwrap_or_else(|_| "UNAVAILABLE".to_owned());
    let head =
        git_text(worktree, &["rev-parse", "HEAD"]).unwrap_or_else(|_| "UNAVAILABLE".to_owned());
    let status = git_text(worktree, &["status", "--porcelain=v1"])
        .unwrap_or_else(|_| "UNAVAILABLE".to_owned());
    let dirty = !status.is_empty();

    let base_is_ancestor = git_status(
        worktree,
        &["merge-base", "--is-ancestor", expected_base, "HEAD"],
    ) == Some(0);

    let upstream = git_text(
        worktree,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    )
    .ok();
    let (ahead, behind) = upstream
        .as_deref()
        .and_then(|name| divergence(worktree, name).ok())
        .map(|(ahead, behind)| (Some(ahead), Some(behind)))
        .unwrap_or((None, None));
    let remote_configured = git_text(worktree, &["remote"])
        .map(|text| !text.is_empty())
        .unwrap_or(false);
    let credential_helper_configured =
        git_status(worktree, &["config", "--get-all", "credential.helper"]) == Some(0);
    let forge_cli_authenticated = Command::new("gh")
        .args(["auth", "status"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    let verification = context::verify_repo(worktree);
    let context_stale = !verification.passed();
    let context_sha = verification
        .context_sha
        .unwrap_or_else(|| "UNAVAILABLE".to_owned());
    let direct_overlap = match overlap_file {
        Some(path) => has_direct_overlap(worktree, expected_base, path)?,
        None => false,
    };
    let red_line = head == "UNAVAILABLE" || !base_is_ancestor;

    let facts = DoctorFacts {
        requested_worktree: requested,
        repository_root,
        expected_branch: expected_branch.to_owned(),
        actual_branch,
        head,
        expected_base: expected_base.to_owned(),
        dirty,
        context_stale,
        direct_overlap,
        red_line,
        ahead,
        behind,
        base_is_ancestor,
        remote_configured,
        upstream,
        credential_helper_configured,
        forge_cli_authenticated,
        context_sha,
    };
    Ok(DoctorReport {
        classification: classify(&facts),
        facts,
    })
}

fn classify(facts: &DoctorFacts) -> Classification {
    if facts.repository_root == "UNAVAILABLE"
        || Path::new(&facts.repository_root) != Path::new(&facts.requested_worktree)
    {
        return Classification::WrongWorktree;
    }
    if facts.actual_branch != facts.expected_branch {
        return Classification::WrongBranch;
    }
    if facts.dirty {
        return Classification::DirtyUnknownWork;
    }
    if facts.context_stale {
        return Classification::ContextStale;
    }
    if facts.direct_overlap {
        return Classification::DirectOverlap;
    }
    if facts.red_line {
        return Classification::RedLineConflict;
    }
    if let (Some(ahead), Some(behind)) = (facts.ahead, facts.behind) {
        if ahead == 0 && behind > 0 {
            return Classification::FastForwardAvailable;
        }
        if ahead > 0 && behind > 0 {
            return if facts.base_is_ancestor {
                Classification::LocalRebaseRequired
            } else {
                Classification::LocalMergeRequired
            };
        }
    }
    if facts.remote_configured && facts.upstream.is_none() {
        return if facts.forge_cli_authenticated {
            Classification::RemoteAuthRepairAvailable
        } else {
            Classification::RemoteAuthBlocked
        };
    }
    Classification::Ready
}

fn divergence(worktree: &Path, upstream: &str) -> AppResult<(u64, u64)> {
    let text = git_text(
        worktree,
        &[
            "rev-list",
            "--left-right",
            "--count",
            &format!("HEAD...{upstream}"),
        ],
    )?;
    let mut values = text.split_whitespace();
    let ahead = values
        .next()
        .ok_or_else(|| "missing ahead count".to_owned())?
        .parse::<u64>()
        .map_err(|error| format!("invalid ahead count: {error}"))?;
    let behind = values
        .next()
        .ok_or_else(|| "missing behind count".to_owned())?
        .parse::<u64>()
        .map_err(|error| format!("invalid behind count: {error}"))?;
    Ok((ahead, behind))
}

fn has_direct_overlap(worktree: &Path, base: &str, overlap_file: &Path) -> AppResult<bool> {
    let requested = fs::read_to_string(overlap_file)
        .map_err(|error| format!("read overlap file {}: {error}", overlap_file.display()))?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let changed = git_text(
        worktree,
        &["diff", "--name-only", &format!("{base}...HEAD")],
    )?;
    Ok(changed.lines().any(|path| requested.contains(path)))
}

fn git_text(worktree: &Path, args: &[&str]) -> AppResult<String> {
    let output = git_output(worktree, args)?;
    if !output.status.success() {
        return Err(format!("git {} failed", args.join(" ")));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|error| format!("git output was not UTF-8: {error}"))
}

fn git_status(worktree: &Path, args: &[&str]) -> Option<i32> {
    git_output(worktree, args).ok()?.status.code()
}

fn git_output(worktree: &Path, args: &[&str]) -> AppResult<Output> {
    let Some(subcommand) = args.first() else {
        return Err("empty git command".to_owned());
    };
    if !READ_ONLY_GIT_SUBCOMMANDS.contains(subcommand) && *subcommand != "branch" {
        return Err(format!(
            "non-read-only git subcommand rejected: {subcommand}"
        ));
    }
    Command::new("git")
        .arg("-C")
        .arg(worktree)
        .args(args)
        .output()
        .map_err(|error| format!("run git {}: {error}", args.join(" ")))
}

fn format_report(report: &DoctorReport, mode: OutputMode) -> String {
    let facts = &report.facts;
    match mode {
        OutputMode::Human => format!(
            "Lane doctor: {}\nWorktree: {}\nBranch: {}\nHEAD: {}\nContext-SHA: {}\nDirty: {}\nRemote: {}\n",
            report.classification.as_str(),
            facts.repository_root,
            facts.actual_branch,
            facts.head,
            facts.context_sha,
            yes_no(facts.dirty),
            if facts.remote_configured { "configured" } else { "UNAVAILABLE" }
        ),
        OutputMode::Machine => format!(
            "classification\t{}\nworktree\t{}\nbranch\t{}\nhead\t{}\ncontext_sha\t{}\ndirty\t{}\nahead\t{}\nbehind\t{}\nremote\t{}\ncredential_helper\t{}\nforge_auth\t{}\n",
            report.classification.as_str(),
            facts.repository_root,
            facts.actual_branch,
            facts.head,
            facts.context_sha,
            yes_no(facts.dirty),
            optional_number(facts.ahead),
            optional_number(facts.behind),
            if facts.remote_configured { "configured" } else { "UNAVAILABLE" },
            yes_no(facts.credential_helper_configured),
            yes_no(facts.forge_cli_authenticated),
        ),
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn optional_number(value: Option<u64>) -> String {
    value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "UNAVAILABLE".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts() -> DoctorFacts {
        DoctorFacts {
            requested_worktree: "/repo".to_owned(),
            repository_root: "/repo".to_owned(),
            expected_branch: "lane".to_owned(),
            actual_branch: "lane".to_owned(),
            head: "1111111111111111111111111111111111111111".to_owned(),
            expected_base: "0000000000000000000000000000000000000000".to_owned(),
            dirty: false,
            context_stale: false,
            direct_overlap: false,
            red_line: false,
            ahead: None,
            behind: None,
            base_is_ancestor: true,
            remote_configured: false,
            upstream: None,
            credential_helper_configured: false,
            forge_cli_authenticated: false,
            context_sha: "a".repeat(64),
        }
    }

    #[test]
    fn correct_clean_lane_is_ready() {
        assert_eq!(classify(&facts()), Classification::Ready);
    }

    #[test]
    fn wrong_branch_is_classified() {
        let mut value = facts();
        value.actual_branch = "other".to_owned();
        assert_eq!(classify(&value), Classification::WrongBranch);
    }

    #[test]
    fn dirty_unknown_work_is_classified() {
        let mut value = facts();
        value.dirty = true;
        assert_eq!(classify(&value), Classification::DirtyUnknownWork);
    }

    #[test]
    fn stale_context_is_classified() {
        let mut value = facts();
        value.context_stale = true;
        assert_eq!(classify(&value), Classification::ContextStale);
    }

    #[test]
    fn direct_overlap_is_classified() {
        let mut value = facts();
        value.direct_overlap = true;
        assert_eq!(classify(&value), Classification::DirectOverlap);
    }

    #[test]
    fn human_and_machine_output_are_deterministic() {
        let report = DoctorReport {
            classification: Classification::Ready,
            facts: facts(),
        };
        assert_eq!(
            format_report(&report, OutputMode::Human),
            format_report(&report, OutputMode::Human)
        );
        assert_eq!(
            format_report(&report, OutputMode::Machine),
            format_report(&report, OutputMode::Machine)
        );
    }

    #[test]
    fn git_allow_list_contains_no_destructive_subcommand() {
        for forbidden in [
            "reset", "clean", "stash", "push", "checkout", "switch", "rebase",
        ] {
            assert!(!READ_ONLY_GIT_SUBCOMMANDS.contains(&forbidden));
        }
    }

    #[test]
    fn every_required_classification_has_stable_machine_text() {
        let values = [
            Classification::Ready,
            Classification::FastForwardAvailable,
            Classification::LocalMergeRequired,
            Classification::LocalRebaseRequired,
            Classification::ContextStale,
            Classification::RemoteAuthRepairAvailable,
            Classification::RemoteAuthBlocked,
            Classification::DirtyUnknownWork,
            Classification::WrongWorktree,
            Classification::WrongBranch,
            Classification::DirectOverlap,
            Classification::RedLineConflict,
        ];
        let names = values.map(Classification::as_str);
        for (index, name) in names.iter().enumerate() {
            assert!(!name.is_empty());
            assert!(!names[..index].contains(name));
        }
    }
}
