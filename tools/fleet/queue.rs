use std::{collections::BTreeSet, fs, path::Path};

use crate::{AppResult, OutputMode};

const SOURCE: &str = "ops/fleet/integration-queue.tsv";
const VIEW: &str = "docs/lanes/integration-queue.md";
const HEADER: [&str; 13] = [
    "lane_id",
    "candidate_head",
    "base",
    "context_sha",
    "artifact",
    "targeted_validation",
    "full_verify_status",
    "overlap_risk",
    "dependency",
    "authority_impact",
    "revalidation_state",
    "integration_decision",
    "push_state",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct QueueRow {
    fields: Vec<String>,
}

pub(crate) fn command_render(repo: &Path, mode: OutputMode) -> AppResult<()> {
    let rows = read_queue(&repo.join(SOURCE))?;
    let rendered = render(&rows);
    fs::write(repo.join(VIEW), rendered.as_bytes())
        .map_err(|error| format!("write generated queue view: {error}"))?;
    match mode {
        OutputMode::Human => println!("Integration queue rendered: {} row(s)", rows.len()),
        OutputMode::Machine => {
            println!("status\tPASS");
            println!("row_count\t{}", rows.len());
        }
    }
    Ok(())
}

pub(crate) fn command_verify(repo: &Path, mode: OutputMode) -> AppResult<()> {
    let rows = read_queue(&repo.join(SOURCE))?;
    let expected = render(&rows);
    let actual = fs::read_to_string(repo.join(VIEW))
        .map_err(|error| format!("read generated queue view: {error}"))?;
    let fresh = actual == expected;
    match mode {
        OutputMode::Human => println!(
            "Integration queue view: {} ({} row(s))",
            if fresh { "PASS" } else { "STALE" },
            rows.len()
        ),
        OutputMode::Machine => {
            println!("status\t{}", if fresh { "PASS" } else { "FAIL" });
            println!("row_count\t{}", rows.len());
            println!("view_fresh\t{}", if fresh { "yes" } else { "no" });
        }
    }
    if fresh {
        Ok(())
    } else {
        Err("generated integration queue view is stale".to_owned())
    }
}

fn read_queue(path: &Path) -> AppResult<Vec<QueueRow>> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("read integration queue {}: {error}", path.display()))?;
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| "integration queue is empty".to_owned())?
        .split('\t')
        .collect::<Vec<_>>();
    if header != HEADER {
        return Err("integration queue header does not match the strict schema".to_owned());
    }

    let mut rows = Vec::new();
    let mut lane_ids = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        let line_number = index + 2;
        let fields = line.split('\t').map(str::to_owned).collect::<Vec<_>>();
        if fields.len() != HEADER.len() {
            return Err(format!(
                "integration queue line {} has {} fields; expected {}",
                line_number,
                fields.len(),
                HEADER.len()
            ));
        }
        if fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "integration queue line {} has an empty field",
                line_number
            ));
        }
        if fields
            .iter()
            .any(|field| field.contains('\t') || field.contains('\n') || field.contains('\r'))
        {
            return Err(format!(
                "integration queue line {line_number} contains an embedded delimiter"
            ));
        }
        let lane_id = &fields[0];
        if !valid_lane_id(lane_id) {
            return Err(format!(
                "integration queue line {line_number} has invalid lane ID: {lane_id}"
            ));
        }
        if !lane_ids.insert(lane_id.clone()) {
            return Err(format!("duplicate integration queue lane ID: {lane_id}"));
        }
        if !is_lower_hex(&fields[1], 40) {
            return Err(format!(
                "integration queue line {line_number} has invalid candidate HEAD"
            ));
        }
        if !is_lower_hex(&fields[2], 40) {
            return Err(format!(
                "integration queue line {line_number} has invalid base"
            ));
        }
        if !is_lower_hex(&fields[3], 64) {
            return Err(format!(
                "integration queue line {line_number} has invalid Context-SHA"
            ));
        }
        rows.push(QueueRow { fields });
    }
    Ok(rows)
}

fn valid_lane_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn render(rows: &[QueueRow]) -> String {
    let mut output = String::from(
        "# Integration Queue\n\nGenerated from `ops/fleet/integration-queue.tsv` by `./scripts/fleet.sh queue-render`.\nDo not edit this view by hand.\n\n| Lane ID | Candidate HEAD | Base | Context-SHA | Artifact | Targeted validation | Full verify | Overlap risk | Dependency | Authority impact | Revalidation | Decision | Push state |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    if rows.is_empty() {
        output.push_str("\nNo integration candidates are currently enqueued.\n");
        return output;
    }
    for row in rows {
        output.push('|');
        for field in &row.fields {
            output.push(' ');
            output.push_str(&field.replace('|', "\\|"));
            output.push_str(" |");
        }
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempDir;

    fn queue_text(rows: &[String]) -> String {
        let mut text = HEADER.join("\t");
        text.push('\n');
        for row in rows {
            text.push_str(row);
            text.push('\n');
        }
        text
    }

    fn valid_row(lane: &str) -> String {
        [
            lane.to_owned(),
            "1".repeat(40),
            "0".repeat(40),
            "a".repeat(64),
            "/tmp/lane.tar".to_owned(),
            "unit-pass".to_owned(),
            "full-pass".to_owned(),
            "none".to_owned(),
            "none".to_owned(),
            "none".to_owned(),
            "current".to_owned(),
            "awaiting-review".to_owned(),
            "not-pushed".to_owned(),
        ]
        .join("\t")
    }

    fn write_queue(temp: &TempDir, rows: &[String]) -> std::path::PathBuf {
        let path = temp.path().join("queue.tsv");
        fs::write(&path, queue_text(rows)).expect("write queue");
        path
    }

    #[test]
    fn valid_queue_renders_deterministic_markdown() {
        let path = Path::new("tools/fleet/tests/fixtures/queue-valid.tsv");
        let rows = read_queue(path).expect("valid queue");
        assert_eq!(rows.len(), 1);
        assert_eq!(render(&rows), render(&rows));
        assert!(render(&rows).contains("| lane-a |"));
    }

    #[test]
    fn malformed_field_count_fails() {
        let path = Path::new("tools/fleet/tests/fixtures/queue-malformed.tsv");
        assert!(read_queue(path)
            .expect_err("malformed queue fails")
            .contains("fields"));
    }

    #[test]
    fn malformed_candidate_head_fails() {
        let temp = TempDir::new("queue-candidate-head");
        let row = valid_row("lane-a").replacen(&"1".repeat(40), "ABC", 1);
        let error = read_queue(&write_queue(&temp, &[row])).expect_err("candidate must fail");
        assert!(error.contains("candidate HEAD"));
    }

    #[test]
    fn malformed_base_fails() {
        let temp = TempDir::new("queue-base");
        let row = valid_row("lane-a").replacen(&"0".repeat(40), &"G".repeat(40), 1);
        let error = read_queue(&write_queue(&temp, &[row])).expect_err("base must fail");
        assert!(error.contains("invalid base"));
    }

    #[test]
    fn malformed_context_sha_fails() {
        let temp = TempDir::new("queue-context");
        let row = valid_row("lane-a").replacen(&"a".repeat(64), &"A".repeat(64), 1);
        let error = read_queue(&write_queue(&temp, &[row])).expect_err("context must fail");
        assert!(error.contains("Context-SHA"));
    }

    #[test]
    fn duplicate_lane_id_fails() {
        let temp = TempDir::new("queue-duplicate-lane");
        let row = valid_row("lane-a");
        let error = read_queue(&write_queue(&temp, &[row.clone(), row]))
            .expect_err("duplicate lane must fail");
        assert!(error.contains("duplicate integration queue lane ID: lane-a"));
    }

    #[test]
    fn invalid_lane_id_fails() {
        let temp = TempDir::new("queue-lane-id");
        let error = read_queue(&write_queue(&temp, &[valid_row("lane/a")]))
            .expect_err("invalid lane must fail");
        assert!(error.contains("invalid lane ID"));
    }

    #[test]
    fn empty_field_fails() {
        let temp = TempDir::new("queue-empty-field");
        let row = valid_row("lane-a").replace("/tmp/lane.tar", "");
        let error = read_queue(&write_queue(&temp, &[row])).expect_err("empty field must fail");
        assert!(error.contains("empty field"));
    }

    #[test]
    fn embedded_tab_or_newline_cannot_form_a_valid_row() {
        let tab_temp = TempDir::new("queue-embedded-tab");
        let tab_row = valid_row("lane-a").replace("/tmp/lane.tar", "/tmp/lane\tbad");
        let tab_error = read_queue(&write_queue(&tab_temp, &[tab_row])).expect_err("tab must fail");
        assert!(tab_error.contains("fields"));

        let newline_temp = TempDir::new("queue-embedded-newline");
        let newline_row = valid_row("lane-a").replace("/tmp/lane.tar", "/tmp/lane\nbad");
        let newline_error =
            read_queue(&write_queue(&newline_temp, &[newline_row])).expect_err("newline must fail");
        assert!(newline_error.contains("fields"));
    }

    #[test]
    fn current_empty_source_is_valid() {
        let rows = read_queue(Path::new(SOURCE)).expect("current empty queue");
        assert!(rows.is_empty());
    }

    #[test]
    fn generated_human_view_freshness_is_checked() {
        let temp = TempDir::new("queue-freshness");
        fs::create_dir_all(temp.path().join("ops/fleet")).expect("create ops");
        fs::create_dir_all(temp.path().join("docs/lanes")).expect("create docs");
        let source = fs::read_to_string("tools/fleet/tests/fixtures/queue-valid.tsv")
            .expect("read queue fixture");
        fs::write(temp.path().join(SOURCE), source).expect("write queue source");
        let rows = read_queue(&temp.path().join(SOURCE)).expect("read temp queue");
        fs::write(temp.path().join(VIEW), render(&rows)).expect("write view");
        assert!(command_verify(temp.path(), OutputMode::Machine).is_ok());
        fs::write(temp.path().join(VIEW), "stale\n").expect("stale view");
        assert!(command_verify(temp.path(), OutputMode::Machine).is_err());
    }
}
