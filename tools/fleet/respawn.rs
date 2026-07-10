use std::{collections::BTreeMap, fs, path::Path, process::Command};

use crate::{hash, packet, AppResult};

const CONTEXT_SHA_PATH: &str = "ops/fleet/context-sha256.txt";

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkerState {
    head: String,
    branch: String,
    dirty: bool,
}

pub(crate) fn command_generate(repo: &Path, lane: &str, output: &Path) -> AppResult<()> {
    let bundle = render_bundle(repo, lane)?;
    packet::validate_generated(&bundle)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create output directory {}: {error}", parent.display()))?;
    }
    fs::write(output, bundle.as_bytes())
        .map_err(|error| format!("write respawn bundle {}: {error}", output.display()))?;
    println!("Respawn bundle written: {}", output.display());
    println!("SHA-256: {}", hash::sha256(bundle.as_bytes()));
    Ok(())
}

fn render_bundle(repo: &Path, lane: &str) -> AppResult<String> {
    if !valid_lane_id(lane) {
        return Err(format!("invalid lane ID: {lane}"));
    }
    let lane_path = repo.join("docs/lanes").join(format!("{lane}.md"));
    let text = fs::read_to_string(&lane_path)
        .map_err(|error| format!("read lane page {}: {error}", lane_path.display()))?;
    let fields = lane_fields(&text)?;

    let lane_id = required(&fields, "Lane ID")?;
    if lane_id != lane {
        return Err(format!(
            "lane page ID {lane_id} does not match requested {lane}"
        ));
    }
    let supervisor = required(&fields, "Supervisor destination")?;
    let expected_reply = required(&fields, "Supervisor expected reply type")?;
    let latest_packet = required(&fields, "Latest packet ID")?;
    let goal = required(&fields, "Current goal")?;
    let worktree = required(&fields, "Worker Codex worktree")?;
    let branch = required(&fields, "Branch")?;
    let base_line = required(&fields, "Accepted base source")?;
    let base = first_commit_sha(&base_line)
        .ok_or_else(|| "lane page missing required base SHA".to_owned())?;
    let artifacts = required(&fields, "Artifact references")?;
    let authority = required(&fields, "Authority state")?;
    let prohibited = required(&fields, "Prohibited authority")?;
    let proof = required(&fields, "Expected proof")?;
    let risk = required(&fields, "Unresolved risk")?;
    let next_action = required(&fields, "Exact next action")?;
    let stop = required(&fields, "Stop conditions")?;
    let stale_session = required(&fields, "Stale session identity")?;
    let deactivation = required(&fields, "Deactivation instruction")?;

    let context_sha = fs::read_to_string(repo.join(CONTEXT_SHA_PATH))
        .map_err(|error| format!("read Context-SHA: {error}"))?;
    let context_sha = context_sha.trim();
    if !hash::is_lower_hex_sha256(context_sha) {
        return Err("reported Context-SHA is invalid".to_owned());
    }

    let worker = inspect_worker(Path::new(&worktree), &branch)?;
    let packet_id = packet_id(lane, context_sha, &worker.head);
    let worker_condition = if worker.dirty { "dirty" } else { "clean" };
    let dirty_instruction = if worker.dirty {
        "Worker preservation: preserve and inspect all dirty, staged, and untracked work before editing.\n"
    } else {
        "Worker preservation: preserve the clean worker branch and worktree.\n"
    };

    Ok(format!(
        "BEGIN RUST REPS PROJECT PACKET\n\nPacket-Version: 1\nPacket-ID: {packet_id}\nReply-To: {latest_packet}\nFrom: Master GPT\nTo: {supervisor}\nProject-ID: rust-reps\nLane-ID: {lane}\nGoal-ID: {goal}\nContext-SHA: {context_sha}\nExpected-Base: {base}\nWorktree: {worktree}\nBranch: {branch}\nAuthority-State: {authority}\nExpected-Reply-Type: {expected_reply}\nStale-When: {stop}\nAllowed-Final-Classes: ACCEPTED, NEEDS_FIX, PARTIAL, STOP\nAllowed-Stop-Classes: UNKNOWN_WORK_AT_RISK, AUTHORITY_CONFLICT, VALIDATION_FAILURE\n\nRespawn source lane page: docs/lanes/{lane}.md\nProject identity: rust-reps\nLane ID: {lane}\nCurrent Context-SHA: {context_sha}\nCurrent goal: {goal}\nAccepted base: {base}\nLatest candidate HEAD: {}\nWorker Codex worktree: {worktree}\nBranch: {}\nWorker state: {worker_condition}\nArtifact references: {artifacts}\nUnresolved risk: {risk}\nExact next action: {next_action}\nAuthority state: {authority}\nProhibited authority: {prohibited}\nExpected proof: {proof}\nStale session identity: {stale_session}\nDeactivation instruction: {deactivation}\nRestart-Goal: false\n{dirty_instruction}Donor and unknown-work preservation: preserve all useful branches, worktrees, packets, reports, artifacts, and unknown work.\nNo push or deploy is authorized.\n\nEND RUST REPS PROJECT PACKET\n",
        worker.head, worker.branch
    ))
}

fn lane_fields(text: &str) -> AppResult<BTreeMap<String, String>> {
    let mut fields = BTreeMap::new();
    for line in text.lines().filter_map(|line| line.strip_prefix("- ")) {
        let Some((key, value)) = line.split_once(": ") else {
            continue;
        };
        let value = value.trim_matches('`').to_owned();
        if fields.insert(key.to_owned(), value).is_some() {
            return Err(format!("duplicate lane field: {key}"));
        }
    }
    Ok(fields)
}

fn required(fields: &BTreeMap<String, String>, name: &str) -> AppResult<String> {
    fields
        .get(name)
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("lane page missing required field: {name}"))
}

fn first_commit_sha(text: &str) -> Option<String> {
    text.split(|character: char| !character.is_ascii_hexdigit())
        .find(|part| is_commit_sha(part))
        .map(str::to_owned)
}

fn is_commit_sha(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_lane_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
}

fn packet_id(lane: &str, context_sha: &str, candidate_head: &str) -> String {
    let normalized_lane = lane
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_uppercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    format!(
        "RR-RESPAWN-{normalized_lane}-{}-{}",
        &context_sha[..12],
        &candidate_head[..12]
    )
}

fn inspect_worker(worktree: &Path, expected_branch: &str) -> AppResult<WorkerState> {
    if !worktree.is_dir() {
        return Err(format!(
            "worker worktree is missing or not a directory: {}",
            worktree.display()
        ));
    }
    let requested = fs::canonicalize(worktree)
        .map_err(|error| format!("canonicalize worker worktree: {error}"))?;
    let root = git_text(worktree, &["rev-parse", "--show-toplevel"])?;
    let root = fs::canonicalize(&root)
        .map_err(|error| format!("canonicalize worker repository root {root}: {error}"))?;
    if requested != root {
        return Err(format!(
            "worker worktree path {} is not repository root {}",
            requested.display(),
            root.display()
        ));
    }

    let branch = git_text(worktree, &["branch", "--show-current"])?;
    if branch != expected_branch {
        return Err(format!(
            "worker branch mismatch: expected {expected_branch}, found {branch}"
        ));
    }
    let head = git_text(worktree, &["rev-parse", "HEAD"])?;
    if !is_commit_sha(&head) {
        return Err(format!("worker candidate HEAD is invalid: {head}"));
    }
    let status = git_text(worktree, &["status", "--porcelain=v1"])?;
    Ok(WorkerState {
        head,
        branch,
        dirty: !status.is_empty(),
    })
}

fn git_text(worktree: &Path, args: &[&str]) -> AppResult<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .args(args)
        .output()
        .map_err(|error| format!("run git {}: {error}", args.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "git -C {} {} failed with status {}: {}",
            worktree.display(),
            args.join(" "),
            output
                .status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "signal".to_owned()),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|error| format!("git output was not UTF-8: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempDir;
    use std::process::{Command, Output};

    const BASE: &str = "1111111111111111111111111111111111111111";

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

    fn init_worker(label: &str) -> TempDir {
        let worker = TempDir::new(label);
        git_ok(worker.path(), &["init", "--quiet"]);
        git_ok(worker.path(), &["config", "user.name", "Fleet Test"]);
        git_ok(
            worker.path(),
            &["config", "user.email", "fleet-test@example.invalid"],
        );
        fs::write(worker.path().join("worker.txt"), "one\n").expect("write worker file");
        git_ok(worker.path(), &["add", "worker.txt"]);
        git_ok(worker.path(), &["commit", "--quiet", "-m", "worker one"]);
        worker
    }

    fn worker_head(worker: &Path) -> String {
        git_text(worker, &["rev-parse", "HEAD"]).expect("worker head")
    }

    fn worker_branch(worker: &Path) -> String {
        git_text(worker, &["branch", "--show-current"]).expect("worker branch")
    }

    fn write_lane(repo: &Path, worker: &Path, branch: &str, latest_packet: &str) {
        fs::create_dir_all(repo.join("docs/lanes")).expect("create lanes");
        fs::create_dir_all(repo.join("ops/fleet")).expect("create ops");
        fs::write(
            repo.join("docs/lanes/test-lane.md"),
            format!(
                "- Lane ID: `test-lane`\n- Supervisor destination: `Master GPT`\n- Supervisor expected reply type: `SUPERVISOR_LANE_RESULT_V1`\n- Latest packet ID: `{latest_packet}`\n- Current goal: `test-goal`\n- Worker Codex worktree: `{}`\n- Branch: `{branch}`\n- Accepted base source: local master at `{BASE}`\n- Artifact references: `/tmp/test-artifact`\n- Authority state: local inspection only.\n- Prohibited authority: push and deploy.\n- Expected proof: deterministic tests.\n- Unresolved risk: remote unavailable.\n- Exact next action: continue bounded goal.\n- Stop conditions: base differs.\n- Stale session identity: old test session.\n- Deactivation instruction: preserve old test session after acknowledgement.\n",
                worker.display()
            ),
        )
        .expect("write lane");
        fs::write(repo.join(CONTEXT_SHA_PATH), format!("{}\n", "a".repeat(64)))
            .expect("write context sha");
    }

    fn valid_context(label: &str) -> (TempDir, TempDir) {
        let repo = TempDir::new(&format!("{label}-context"));
        let worker = init_worker(&format!("{label}-worker"));
        write_lane(
            repo.path(),
            worker.path(),
            &worker_branch(worker.path()),
            "RR-TEST-001",
        );
        (repo, worker)
    }

    fn packet_field<'a>(packet: &'a str, name: &str) -> &'a str {
        let prefix = format!("{name}: ");
        packet
            .lines()
            .find_map(|line| line.strip_prefix(&prefix))
            .expect("packet field")
    }

    #[test]
    fn candidate_head_comes_from_lane_worker_not_context_repo() {
        let (repo, worker) = valid_context("respawn-worker-head");
        let bundle = render_bundle(repo.path(), "test-lane").expect("render bundle");
        assert!(bundle.contains(&format!(
            "Latest candidate HEAD: {}",
            worker_head(worker.path())
        )));
    }

    #[test]
    fn worker_branch_mismatch_fails() {
        let (repo, worker) = valid_context("respawn-branch-mismatch");
        write_lane(repo.path(), worker.path(), "wrong-branch", "RR-TEST-001");
        let error = render_bundle(repo.path(), "test-lane").expect_err("branch mismatch");
        assert!(error.contains("worker branch mismatch"));
    }

    #[test]
    fn missing_worker_worktree_fails() {
        let (repo, worker) = valid_context("respawn-missing-worker");
        let missing = worker.path().join("missing");
        write_lane(
            repo.path(),
            &missing,
            &worker_branch(worker.path()),
            "RR-TEST-001",
        );
        let error = render_bundle(repo.path(), "test-lane").expect_err("missing worker");
        assert!(error.contains("worker worktree is missing"));
    }

    #[test]
    fn duplicate_lane_field_fails() {
        let (repo, _worker) = valid_context("respawn-duplicate-field");
        let path = repo.path().join("docs/lanes/test-lane.md");
        let mut text = fs::read_to_string(&path).expect("read lane");
        text.push_str("- Prohibited authority: duplicate.\n");
        fs::write(path, text).expect("write duplicate");
        let error = render_bundle(repo.path(), "test-lane").expect_err("duplicate field");
        assert_eq!(error, "duplicate lane field: Prohibited authority");
    }

    #[test]
    fn supervisor_reply_artifacts_and_handoff_fields_are_used() {
        let (repo, _worker) = valid_context("respawn-handoff-fields");
        let bundle = render_bundle(repo.path(), "test-lane").expect("render bundle");
        assert!(bundle.contains("Reply-To: RR-TEST-001\n"));
        assert!(bundle.contains("To: Master GPT\n"));
        assert!(bundle.contains("Expected-Reply-Type: SUPERVISOR_LANE_RESULT_V1\n"));
        assert!(bundle.contains("Artifact references: /tmp/test-artifact\n"));
        assert!(bundle.contains("Stale session identity: old test session.\n"));
        assert!(bundle.contains(
            "Deactivation instruction: preserve old test session after acknowledgement.\n"
        ));
        assert!(bundle.contains("Restart-Goal: false\n"));
        assert!(bundle.contains("No push or deploy is authorized.\n"));
    }

    #[test]
    fn dirty_worker_state_is_preserved_and_reported() {
        let (repo, worker) = valid_context("respawn-dirty-worker");
        fs::write(worker.path().join("unknown.txt"), "preserve\n").expect("write unknown");
        let bundle = render_bundle(repo.path(), "test-lane").expect("render dirty bundle");
        assert!(bundle.contains("Worker state: dirty\n"));
        assert!(bundle.contains(
            "Worker preservation: preserve and inspect all dirty, staged, and untracked work before editing.\n"
        ));
    }

    #[test]
    fn packet_id_changes_when_candidate_head_changes() {
        let (repo, worker) = valid_context("respawn-state-id");
        let first = render_bundle(repo.path(), "test-lane").expect("first bundle");
        fs::write(worker.path().join("worker.txt"), "two\n").expect("change worker");
        git_ok(worker.path(), &["commit", "--quiet", "-am", "worker two"]);
        let second = render_bundle(repo.path(), "test-lane").expect("second bundle");
        assert_ne!(
            packet_field(&first, "Packet-ID"),
            packet_field(&second, "Packet-ID")
        );
        assert!(packet_field(&first, "Packet-ID").contains(&"a".repeat(12)));
    }

    #[test]
    fn packet_id_changes_when_context_sha_changes() {
        let (repo, _worker) = valid_context("respawn-context-id");
        let first = render_bundle(repo.path(), "test-lane").expect("first bundle");
        fs::write(
            repo.path().join(CONTEXT_SHA_PATH),
            format!("{}\n", "b".repeat(64)),
        )
        .expect("change context sha");
        let second = render_bundle(repo.path(), "test-lane").expect("second bundle");
        assert_ne!(
            packet_field(&first, "Packet-ID"),
            packet_field(&second, "Packet-ID")
        );
    }

    #[test]
    fn generated_packet_passes_verification_and_is_deterministic() {
        let (repo, _worker) = valid_context("respawn-verified");
        let first = render_bundle(repo.path(), "test-lane").expect("first bundle");
        let second = render_bundle(repo.path(), "test-lane").expect("second bundle");
        assert_eq!(first, second);
        packet::validate_generated(&first).expect("generated packet verifies");
    }

    #[test]
    fn invalid_generated_packet_is_not_written() {
        let (repo, worker) = valid_context("respawn-no-invalid-output");
        write_lane(
            repo.path(),
            worker.path(),
            &worker_branch(worker.path()),
            "INVALID PACKET ID",
        );
        let output = repo.path().join("out/respawn.md");
        let error = command_generate(repo.path(), "test-lane", &output)
            .expect_err("invalid packet must fail");
        assert!(error.contains("generated packet validation failed"));
        assert!(!output.exists());
    }
}
