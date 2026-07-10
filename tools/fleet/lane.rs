use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::{context, AppResult, OutputMode};

const READ_ONLY_GIT_SUBCOMMANDS: &[&str] = &[
    "branch",
    "config",
    "diff",
    "merge-base",
    "remote",
    "rev-list",
    "rev-parse",
    "status",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Classification {
    Ready,
    FastForwardAvailable,
    LocalMergeRequired,
    ContextStale,
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
            Self::ContextStale => "CONTEXT_STALE",
            Self::DirtyUnknownWork => "DIRTY_UNKNOWN_WORK",
            Self::WrongWorktree => "WRONG_WORKTREE",
            Self::WrongBranch => "WRONG_BRANCH",
            Self::DirectOverlap => "DIRECT_OVERLAP",
            Self::RedLineConflict => "RED_LINE_CONFLICT",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct WorktreeStatus {
    staged: Vec<String>,
    unstaged: Vec<String>,
    untracked: Vec<String>,
    conflicted: Vec<String>,
}

impl WorktreeStatus {
    fn dirty(&self) -> bool {
        !self.staged.is_empty()
            || !self.unstaged.is_empty()
            || !self.untracked.is_empty()
            || !self.conflicted.is_empty()
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
    status: WorktreeStatus,
    context_stale: bool,
    direct_overlap: bool,
    red_line: bool,
    ahead: Option<u64>,
    behind: Option<u64>,
    base_is_ancestor: bool,
    remote: String,
    upstream: Option<String>,
    credential_helper: String,
    context_sha: String,
    lane_id: String,
    lane_page: String,
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
    let requested_path = canonicalize(worktree, "requested worktree")?;
    let requested_worktree = requested_path.display().to_string();
    let repository_root = match git_text(worktree, &["rev-parse", "--show-toplevel"]) {
        Ok(root) => canonicalize(Path::new(&root), "discovered repository root")?
            .display()
            .to_string(),
        Err(_) => "UNAVAILABLE".to_owned(),
    };
    let actual_branch = git_text(worktree, &["branch", "--show-current"])
        .unwrap_or_else(|_| "UNAVAILABLE".to_owned());
    let head =
        git_text(worktree, &["rev-parse", "HEAD"]).unwrap_or_else(|_| "UNAVAILABLE".to_owned());
    let status = collect_status(worktree)?;

    let commit_expression = format!("{expected_base}^{{commit}}");
    let base_available =
        git_status(worktree, &["rev-parse", "--verify", &commit_expression]) == Some(0);
    let base_is_ancestor = base_available
        && git_status(
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
    let remote = observation_from_text(git_text(worktree, &["remote"]));
    let credential_helper = observation_from_config(git_output(
        worktree,
        &["config", "--get-all", "credential.helper"],
    ));

    let repository_path = if repository_root == "UNAVAILABLE" {
        requested_path.as_path()
    } else {
        Path::new(&repository_root)
    };
    let verification = context::verify_repo(repository_path);
    let context_stale = !verification.passed();
    let context_sha = verification
        .context_sha
        .unwrap_or_else(|| "UNAVAILABLE".to_owned());
    let direct_overlap = match overlap_file {
        Some(path) if base_is_ancestor => has_direct_overlap(worktree, expected_base, path)?,
        _ => false,
    };
    let red_line = head == "UNAVAILABLE" || !base_is_ancestor;
    let (lane_id, lane_page) =
        active_lane_memory(repository_path, requested_path.as_path(), expected_branch);

    let facts = DoctorFacts {
        requested_worktree,
        repository_root,
        expected_branch: expected_branch.to_owned(),
        actual_branch,
        head,
        expected_base: expected_base.to_owned(),
        status,
        context_stale,
        direct_overlap,
        red_line,
        ahead,
        behind,
        base_is_ancestor,
        remote,
        upstream,
        credential_helper,
        context_sha,
        lane_id,
        lane_page,
    };
    Ok(DoctorReport {
        classification: classify(&facts),
        facts,
    })
}

fn classify(facts: &DoctorFacts) -> Classification {
    if facts.repository_root == "UNAVAILABLE" || facts.repository_root != facts.requested_worktree {
        return Classification::WrongWorktree;
    }
    if facts.actual_branch != facts.expected_branch {
        return Classification::WrongBranch;
    }
    if facts.status.dirty() {
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
            return Classification::LocalMergeRequired;
        }
    }
    Classification::Ready
}

fn canonicalize(path: &Path, label: &str) -> AppResult<PathBuf> {
    fs::canonicalize(path)
        .map_err(|error| format!("canonicalize {label} {}: {error}", path.display()))
}

fn collect_status(worktree: &Path) -> AppResult<WorktreeStatus> {
    let text = git_stdout(
        worktree,
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )?;
    parse_status(&text)
}

fn parse_status(text: &str) -> AppResult<WorktreeStatus> {
    let mut staged = BTreeSet::new();
    let mut unstaged = BTreeSet::new();
    let mut untracked = BTreeSet::new();
    let mut conflicted = BTreeSet::new();

    for line in text.lines() {
        if line.len() < 3 || line.as_bytes()[2] != b' ' {
            return Err(format!("malformed git status record: {line}"));
        }
        let x = line.as_bytes()[0];
        let y = line.as_bytes()[1];
        let path = line[3..].to_owned();
        if x == b'?' && y == b'?' {
            untracked.insert(path);
        } else if is_conflicted(x, y) {
            conflicted.insert(path);
        } else {
            if x != b' ' {
                staged.insert(path.clone());
            }
            if y != b' ' {
                unstaged.insert(path);
            }
        }
    }

    Ok(WorktreeStatus {
        staged: staged.into_iter().collect(),
        unstaged: unstaged.into_iter().collect(),
        untracked: untracked.into_iter().collect(),
        conflicted: conflicted.into_iter().collect(),
    })
}

fn is_conflicted(x: u8, y: u8) -> bool {
    matches!(
        (x, y),
        (b'D', b'D')
            | (b'A', b'U')
            | (b'U', b'D')
            | (b'U', b'A')
            | (b'D', b'U')
            | (b'A', b'A')
            | (b'U', b'U')
    )
}

fn active_lane_memory(repo: &Path, worktree: &Path, branch: &str) -> (String, String) {
    let registry_path = repo.join("docs/lanes/lane-registry.md");
    let Ok(text) = fs::read_to_string(registry_path) else {
        return unavailable_lane();
    };
    let mut active = false;
    for line in text.lines() {
        if line == "## Active lanes" {
            active = true;
            continue;
        }
        if active && line.starts_with("## ") {
            break;
        }
        if !active || !line.starts_with('|') {
            continue;
        }
        let columns = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        if columns.len() != 5 || columns[0] == "Lane ID" || columns[0].starts_with("---") {
            continue;
        }
        let lane_id = trim_code(columns[0]);
        let Some(page_target) = markdown_link_target(columns[1]) else {
            continue;
        };
        let declared_worktree = PathBuf::from(trim_code(columns[2]));
        let declared_branch = trim_code(columns[3]);
        let Ok(declared_worktree) = fs::canonicalize(declared_worktree) else {
            continue;
        };
        if declared_worktree == worktree && declared_branch == branch {
            return (lane_id.to_owned(), format!("docs/lanes/{page_target}"));
        }
    }
    unavailable_lane()
}

fn unavailable_lane() -> (String, String) {
    ("UNAVAILABLE".to_owned(), "UNAVAILABLE".to_owned())
}

fn trim_code(value: &str) -> &str {
    value.trim().trim_matches('`')
}

fn markdown_link_target(value: &str) -> Option<&str> {
    value.split_once("](")?.1.strip_suffix(')')
}

fn observation_from_text(result: AppResult<String>) -> String {
    match result {
        Ok(text) if text.is_empty() => "not-configured".to_owned(),
        Ok(_) => "configured".to_owned(),
        Err(_) => "unknown".to_owned(),
    }
}

fn observation_from_config(result: AppResult<Output>) -> String {
    match result {
        Ok(output) if output.status.success() && !output.stdout.is_empty() => {
            "configured".to_owned()
        }
        Ok(output) if output.status.code() == Some(1) => "not-configured".to_owned(),
        Ok(_) | Err(_) => "unknown".to_owned(),
    }
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
    git_stdout(worktree, args).map(|text| text.trim().to_owned())
}

fn git_stdout(worktree: &Path, args: &[&str]) -> AppResult<String> {
    let output = git_output(worktree, args)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git {} failed with status {}: {}",
            args.join(" "),
            output
                .status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "signal".to_owned()),
            stderr.trim()
        ));
    }
    String::from_utf8(output.stdout).map_err(|error| format!("git output was not UTF-8: {error}"))
}

fn git_status(worktree: &Path, args: &[&str]) -> Option<i32> {
    git_output(worktree, args).ok()?.status.code()
}

fn git_output(worktree: &Path, args: &[&str]) -> AppResult<Output> {
    let Some(subcommand) = args.first() else {
        return Err("empty git command".to_owned());
    };
    if !READ_ONLY_GIT_SUBCOMMANDS.contains(subcommand) {
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
        OutputMode::Human => {
            let mut output = format!(
                "Lane doctor: {}\nWorktree: {}\nBranch: {}\nHEAD: {}\nContext-SHA: {}\nLane: {} ({})\nRemote: {}\nUpstream: {}\n",
                report.classification.as_str(),
                facts.repository_root,
                facts.actual_branch,
                facts.head,
                facts.context_sha,
                facts.lane_id,
                facts.lane_page,
                facts.remote,
                facts.upstream.as_deref().unwrap_or("UNAVAILABLE"),
            );
            push_human_paths(&mut output, "Staged", &facts.status.staged);
            push_human_paths(&mut output, "Unstaged", &facts.status.unstaged);
            push_human_paths(&mut output, "Untracked", &facts.status.untracked);
            push_human_paths(&mut output, "Conflicted", &facts.status.conflicted);
            output
        }
        OutputMode::Machine => {
            let mut output = format!(
                "classification\t{}\nrequested_worktree\t{}\nworktree\t{}\nexpected_branch\t{}\nbranch\t{}\nhead\t{}\nexpected_base\t{}\ncontext_sha\t{}\nlane_id\t{}\nlane_page\t{}\ndirty\t{}\nahead\t{}\nbehind\t{}\nremote\t{}\nupstream\t{}\ncredential_helper\t{}\nbase_is_ancestor\t{}\n",
                report.classification.as_str(),
                escape_field(&facts.requested_worktree),
                escape_field(&facts.repository_root),
                escape_field(&facts.expected_branch),
                escape_field(&facts.actual_branch),
                facts.head,
                facts.expected_base,
                facts.context_sha,
                escape_field(&facts.lane_id),
                escape_field(&facts.lane_page),
                yes_no(facts.status.dirty()),
                optional_number(facts.ahead),
                optional_number(facts.behind),
                facts.remote,
                escape_field(facts.upstream.as_deref().unwrap_or("UNAVAILABLE")),
                facts.credential_helper,
                yes_no(facts.base_is_ancestor),
            );
            push_machine_paths(&mut output, "staged", &facts.status.staged);
            push_machine_paths(&mut output, "unstaged", &facts.status.unstaged);
            push_machine_paths(&mut output, "untracked", &facts.status.untracked);
            push_machine_paths(&mut output, "conflicted", &facts.status.conflicted);
            output
        }
    }
}

fn push_human_paths(output: &mut String, label: &str, paths: &[String]) {
    output.push_str(&format!("{label} paths ({}):", paths.len()));
    if paths.is_empty() {
        output.push_str(" none\n");
    } else {
        output.push(' ');
        output.push_str(
            &paths
                .iter()
                .map(|path| escape_field(path))
                .collect::<Vec<_>>()
                .join(", "),
        );
        output.push('\n');
    }
}

fn push_machine_paths(output: &mut String, name: &str, paths: &[String]) {
    output.push_str(&format!("{name}_count\t{}\n", paths.len()));
    for path in paths {
        output.push_str(&format!("{name}_path\t{}\n", escape_field(path)));
    }
}

fn escape_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
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
    use crate::test_support::TempDir;
    use std::process::Command;

    fn facts() -> DoctorFacts {
        DoctorFacts {
            requested_worktree: "/repo".to_owned(),
            repository_root: "/repo".to_owned(),
            expected_branch: "lane".to_owned(),
            actual_branch: "lane".to_owned(),
            head: "1111111111111111111111111111111111111111".to_owned(),
            expected_base: "0000000000000000000000000000000000000000".to_owned(),
            status: WorktreeStatus::default(),
            context_stale: false,
            direct_overlap: false,
            red_line: false,
            ahead: None,
            behind: None,
            base_is_ancestor: true,
            remote: "not-configured".to_owned(),
            upstream: None,
            credential_helper: "not-configured".to_owned(),
            context_sha: "a".repeat(64),
            lane_id: "lane".to_owned(),
            lane_page: "docs/lanes/lane.md".to_owned(),
        }
    }

    fn git(repo: &Path, args: &[&str]) -> Output {
        Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .output()
            .expect("run test git command")
    }

    fn git_ok(repo: &Path, args: &[&str]) {
        let output = git(repo, args);
        assert!(
            output.status.success(),
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn init_repo(label: &str) -> TempDir {
        let temp = TempDir::new(label);
        git_ok(temp.path(), &["init", "--quiet"]);
        git_ok(temp.path(), &["config", "user.name", "Fleet Test"]);
        git_ok(
            temp.path(),
            &["config", "user.email", "fleet-test@example.invalid"],
        );
        fs::write(temp.path().join("tracked.txt"), "base\n").expect("write tracked file");
        git_ok(temp.path(), &["add", "tracked.txt"]);
        git_ok(temp.path(), &["commit", "--quiet", "-m", "base"]);
        temp
    }

    fn head(repo: &Path) -> String {
        let output = git(repo, &["rev-parse", "HEAD"]);
        assert!(output.status.success());
        String::from_utf8(output.stdout)
            .expect("head UTF-8")
            .trim()
            .to_owned()
    }

    fn branch(repo: &Path) -> String {
        let output = git(repo, &["branch", "--show-current"]);
        assert!(output.status.success());
        String::from_utf8(output.stdout)
            .expect("branch UTF-8")
            .trim()
            .to_owned()
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
    fn wrong_or_unrelated_base_is_red_line_conflict() {
        let mut value = facts();
        value.base_is_ancestor = false;
        value.red_line = true;
        assert_eq!(classify(&value), Classification::RedLineConflict);
    }

    #[test]
    fn behind_only_upstream_is_fast_forward_available() {
        let temp = init_repo("lane-behind-only");
        let original_branch = branch(temp.path());
        git_ok(temp.path(), &["checkout", "--quiet", "-b", "upstream"]);
        fs::write(temp.path().join("upstream.txt"), "upstream\n").expect("write upstream");
        git_ok(temp.path(), &["add", "upstream.txt"]);
        git_ok(temp.path(), &["commit", "--quiet", "-m", "upstream"]);
        git_ok(temp.path(), &["checkout", "--quiet", &original_branch]);
        let (ahead, behind) = divergence(temp.path(), "upstream").expect("measure divergence");
        assert_eq!((ahead, behind), (0, 1));
        let mut value = facts();
        value.ahead = Some(ahead);
        value.behind = Some(behind);
        assert_eq!(classify(&value), Classification::FastForwardAvailable);
    }

    #[test]
    fn ahead_and_behind_upstream_requires_local_merge() {
        let temp = init_repo("lane-diverged");
        let original_branch = branch(temp.path());
        git_ok(temp.path(), &["checkout", "--quiet", "-b", "upstream"]);
        fs::write(temp.path().join("upstream.txt"), "upstream\n").expect("write upstream");
        git_ok(temp.path(), &["add", "upstream.txt"]);
        git_ok(temp.path(), &["commit", "--quiet", "-m", "upstream"]);
        git_ok(temp.path(), &["checkout", "--quiet", &original_branch]);
        fs::write(temp.path().join("local.txt"), "local\n").expect("write local");
        git_ok(temp.path(), &["add", "local.txt"]);
        git_ok(temp.path(), &["commit", "--quiet", "-m", "local"]);
        let (ahead, behind) = divergence(temp.path(), "upstream").expect("measure divergence");
        assert_eq!((ahead, behind), (1, 1));
        let mut value = facts();
        value.ahead = Some(ahead);
        value.behind = Some(behind);
        assert_eq!(classify(&value), Classification::LocalMergeRequired);
    }

    #[test]
    fn no_upstream_does_not_become_auth_failure() {
        let mut value = facts();
        value.remote = "configured".to_owned();
        value.upstream = None;
        assert_eq!(classify(&value), Classification::Ready);
    }

    #[test]
    fn staged_unstaged_and_untracked_paths_are_reported_separately() {
        let temp = init_repo("lane-status-paths");
        fs::write(temp.path().join("staged.txt"), "staged\n").expect("write staged");
        git_ok(temp.path(), &["add", "staged.txt"]);
        fs::write(temp.path().join("tracked.txt"), "changed\n").expect("write unstaged");
        fs::write(temp.path().join("untracked.txt"), "untracked\n").expect("write untracked");

        let status = collect_status(temp.path()).expect("collect status");
        assert_eq!(status.staged, ["staged.txt"]);
        assert_eq!(status.unstaged, ["tracked.txt"]);
        assert_eq!(status.untracked, ["untracked.txt"]);
        assert!(status.conflicted.is_empty());
    }

    #[test]
    fn conflicted_path_is_reported_without_mutation() {
        let temp = init_repo("lane-conflict");
        let original_branch = branch(temp.path());
        git_ok(temp.path(), &["checkout", "--quiet", "-b", "other"]);
        fs::write(temp.path().join("tracked.txt"), "other\n").expect("write other");
        git_ok(temp.path(), &["commit", "--quiet", "-am", "other"]);
        git_ok(temp.path(), &["checkout", "--quiet", &original_branch]);
        fs::write(temp.path().join("tracked.txt"), "current\n").expect("write current");
        git_ok(temp.path(), &["commit", "--quiet", "-am", "current"]);
        let merge = git(temp.path(), &["merge", "other"]);
        assert!(!merge.status.success());

        let before_head = head(temp.path());
        let status = collect_status(temp.path()).expect("collect conflict");
        assert_eq!(status.conflicted, ["tracked.txt"]);
        assert_eq!(head(temp.path()), before_head);
    }

    #[test]
    fn direct_overlap_is_classified() {
        let mut value = facts();
        value.direct_overlap = true;
        assert_eq!(classify(&value), Classification::DirectOverlap);
    }

    #[test]
    fn canonicalized_equivalent_worktree_is_not_wrong_worktree() {
        let temp = init_repo("lane-canonical");
        let equivalent = temp.path().join(".");
        let report = inspect(&equivalent, &branch(temp.path()), &head(temp.path()), None)
            .expect("inspect equivalent path");
        assert_ne!(report.classification, Classification::WrongWorktree);
    }

    #[test]
    fn machine_output_contains_lane_and_separate_path_records() {
        let mut value = facts();
        value.status.staged.push("staged\tpath".to_owned());
        value.status.unstaged.push("unstaged\npath".to_owned());
        value.status.untracked.push("untracked\\path".to_owned());
        value.status.conflicted.push("conflicted".to_owned());
        let report = DoctorReport {
            classification: Classification::DirtyUnknownWork,
            facts: value,
        };
        let output = format_report(&report, OutputMode::Machine);
        assert!(output.contains("lane_id\tlane\n"));
        assert!(output.contains("lane_page\tdocs/lanes/lane.md\n"));
        assert!(output.contains("staged_count\t1\n"));
        assert!(output.contains("staged_path\tstaged\\tpath\n"));
        assert!(output.contains("unstaged_path\tunstaged\\npath\n"));
        assert!(output.contains("untracked_path\tuntracked\\\\path\n"));
        assert!(output.contains("conflicted_path\tconflicted\n"));
    }

    #[test]
    fn active_lane_memory_matches_declared_worktree_and_branch() {
        let temp = TempDir::new("lane-memory");
        fs::create_dir_all(temp.path().join("docs/lanes")).expect("create lanes");
        fs::write(
            temp.path().join("docs/lanes/lane-registry.md"),
            format!(
                "## Active lanes\n\n| Lane ID | Page | Worktree | Branch | Status |\n| --- | --- | --- | --- | --- |\n| `lane-a` | [`lane-a.md`](lane-a.md) | `{}` | `lane/a` | active |\n\n## Retired lanes\n",
                temp.path().display()
            ),
        )
        .expect("write registry");
        let canonical = fs::canonicalize(temp.path()).expect("canonical temp");
        assert_eq!(
            active_lane_memory(temp.path(), &canonical, "lane/a"),
            ("lane-a".to_owned(), "docs/lanes/lane-a.md".to_owned())
        );
    }

    #[test]
    fn doctor_inspection_leaves_head_and_status_unchanged() {
        let temp = init_repo("lane-read-only");
        let before_head = head(temp.path());
        let before_status = collect_status(temp.path()).expect("status before");
        let _ = inspect(temp.path(), &branch(temp.path()), &before_head, None)
            .expect("inspect repository");
        assert_eq!(head(temp.path()), before_head);
        assert_eq!(
            collect_status(temp.path()).expect("status after"),
            before_status
        );
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
            "reset", "clean", "stash", "push", "checkout", "switch", "rebase", "merge", "fetch",
            "pull",
        ] {
            assert!(!READ_ONLY_GIT_SUBCOMMANDS.contains(&forbidden));
        }
    }

    #[test]
    fn every_v1_classification_has_stable_machine_text() {
        let values = [
            Classification::Ready,
            Classification::FastForwardAvailable,
            Classification::LocalMergeRequired,
            Classification::ContextStale,
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
