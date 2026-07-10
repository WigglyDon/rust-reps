use std::{fs, path::Path};

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
    for (index, line) in lines.enumerate() {
        let fields = line.split('\t').map(str::to_owned).collect::<Vec<_>>();
        if fields.len() != HEADER.len() {
            return Err(format!(
                "integration queue line {} has {} fields; expected {}",
                index + 2,
                fields.len(),
                HEADER.len()
            ));
        }
        if fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "integration queue line {} has an empty field",
                index + 2
            ));
        }
        rows.push(QueueRow { fields });
    }
    Ok(rows)
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
