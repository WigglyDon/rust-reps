use std::{collections::BTreeMap, fs, path::Path, process::Command};

use crate::{hash, AppResult};

const CONTEXT_SHA_PATH: &str = "ops/fleet/context-sha256.txt";

pub(crate) fn command_generate(repo: &Path, lane: &str, output: &Path) -> AppResult<()> {
    let head = git_head(repo)?;
    let bundle = render_bundle(repo, lane, &head)?;
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

fn render_bundle(repo: &Path, lane: &str, candidate_head: &str) -> AppResult<String> {
    let lane_path = repo.join("docs/lanes").join(format!("{lane}.md"));
    let text = fs::read_to_string(&lane_path)
        .map_err(|error| format!("read lane page {}: {error}", lane_path.display()))?;
    let fields = lane_fields(&text);

    let lane_id = required(&fields, "Lane ID")?;
    if lane_id != lane {
        return Err(format!(
            "lane page ID {lane_id} does not match requested {lane}"
        ));
    }
    let worktree = required(&fields, "Worker Codex worktree")?;
    let branch = required(&fields, "Branch")?;
    let base_line = required(&fields, "Accepted base source")?;
    let base = first_commit_sha(&base_line)
        .ok_or_else(|| "lane page missing required base SHA".to_owned())?;
    let latest_packet = required(&fields, "Latest packet ID")?;
    let goal = required(&fields, "Current goal")?;
    let authority = required(&fields, "Authority state")?;
    let prohibited = required(&fields, "Prohibited authority")?;
    let proof = required(&fields, "Expected proof")?;
    let risk = required(&fields, "Unresolved risk")?;
    let next_action = required(&fields, "Exact next action")?;
    let stop = required(&fields, "Stop conditions")?;

    let context_sha = fs::read_to_string(repo.join(CONTEXT_SHA_PATH))
        .map_err(|error| format!("read Context-SHA: {error}"))?;
    let context_sha = context_sha.trim();
    if !hash::is_lower_hex_sha256(context_sha) {
        return Err("reported Context-SHA is invalid".to_owned());
    }
    if !is_commit_sha(candidate_head) {
        return Err("candidate HEAD is invalid".to_owned());
    }

    let packet_id = format!(
        "RR-RESPAWN-{}-V1",
        lane.chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_uppercase()
                } else {
                    '-'
                }
            })
            .collect::<String>()
    );
    Ok(format!(
        "BEGIN RUST REPS PROJECT PACKET\n\nPacket-Version: 1\nPacket-ID: {packet_id}\nReply-To: {latest_packet}\nFrom: Master GPT\nTo: Master Codex\nProject-ID: rust-reps\nLane-ID: {lane}\nGoal-ID: {goal}\nContext-SHA: {context_sha}\nExpected-Base: {base}\nWorktree: {worktree}\nBranch: {branch}\nAuthority-State: {authority}\nExpected-Reply-Type: MASTER_CODEX_FINAL_REPORT_V1\nStale-When: {stop}\nAllowed-Final-Classes: ACCEPTED, NEEDS_FIX, PARTIAL, STOP\nAllowed-Stop-Classes: UNKNOWN_WORK_AT_RISK, AUTHORITY_CONFLICT, VALIDATION_FAILURE\n\nRespawn source lane page: docs/lanes/{lane}.md\nAccepted base: {base}\nLatest candidate HEAD: {candidate_head}\nLatest packet ID: {latest_packet}\nCurrent goal: {goal}\nUnresolved risk: {risk}\nExact next action: {next_action}\nProhibited authority: {prohibited}\nExpected proof: {proof}\nRestart-Goal: false\nNo push or deploy is authorized. Preserve donor and unknown work.\n\nEND RUST REPS PROJECT PACKET\n"
    ))
}

fn lane_fields(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("- "))
        .filter_map(|line| line.split_once(": "))
        .map(|(key, value)| (key.to_owned(), value.trim_matches('`').to_owned()))
        .collect()
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
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn git_head(repo: &Path) -> AppResult<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|error| format!("run git rev-parse HEAD: {error}"))?;
    if !output.status.success() {
        return Err("git rev-parse HEAD failed".to_owned());
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|error| format!("git HEAD was not UTF-8: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempDir;

    const HEAD: &str = "2222222222222222222222222222222222222222";

    fn write_lane(repo: &Path, include_base: bool) {
        fs::create_dir_all(repo.join("docs/lanes")).expect("create lanes");
        fs::create_dir_all(repo.join("ops/fleet")).expect("create ops");
        let base = if include_base {
            "- Accepted base source: local master at 1111111111111111111111111111111111111111\n"
        } else {
            ""
        };
        fs::write(
            repo.join("docs/lanes/test-lane.md"),
            format!(
                "- Lane ID: `test-lane`\n- Latest packet ID: `RR-TEST-001`\n- Current goal: `test-goal`\n- Worker Codex worktree: `/tmp/test-lane`\n- Branch: `lane/test`\n{base}- Authority state: local inspection only.\n- Prohibited authority: push and deploy.\n- Expected proof: deterministic tests.\n- Unresolved risk: remote unavailable.\n- Exact next action: continue bounded goal.\n- Stop conditions: base differs.\n"
            ),
        )
        .expect("write lane");
        fs::write(repo.join(CONTEXT_SHA_PATH), format!("{}\n", "a".repeat(64)))
            .expect("write context sha");
    }

    #[test]
    fn valid_lane_emits_deterministic_complete_bundle() {
        let temp = TempDir::new("respawn-valid");
        write_lane(temp.path(), true);
        let first = render_bundle(temp.path(), "test-lane", HEAD).expect("render bundle");
        let second = render_bundle(temp.path(), "test-lane", HEAD).expect("render bundle");
        assert_eq!(first, second);
        assert!(first.starts_with("BEGIN RUST REPS PROJECT PACKET\n"));
        assert!(first.ends_with("END RUST REPS PROJECT PACKET\n"));
        assert!(first.contains("Restart-Goal: false"));
        assert!(!first.contains("<replace-me>"));
    }

    #[test]
    fn missing_lane_fails() {
        let temp = TempDir::new("respawn-missing-lane");
        assert!(render_bundle(temp.path(), "missing", HEAD).is_err());
    }

    #[test]
    fn missing_required_base_fails() {
        let temp = TempDir::new("respawn-missing-base");
        write_lane(temp.path(), false);
        let error = render_bundle(temp.path(), "test-lane", HEAD).expect_err("missing base fails");
        assert!(error.contains("Accepted base source"));
    }
}
