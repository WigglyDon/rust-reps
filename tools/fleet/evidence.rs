use std::{collections::BTreeMap, fs, path::Path};

use crate::{hash, AppResult};

const REQUIRED: &[&str] = &[
    "record_type",
    "integration_sha",
    "parent_sha",
    "context_sha",
    "candidate_commits",
    "targeted_gates",
    "full_verification",
    "runtime_evidence_state",
    "artifact_checksums",
    "authority_changes",
    "rollback_state",
    "observability_state",
    "performance_resource_state",
    "deploy_state",
    "push_state",
    "recovery_restart_evidence",
];

pub(crate) fn command_generate(input: &Path, output: &Path) -> AppResult<()> {
    let fields = read_input(input)?;
    let rendered = render(&fields)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create evidence directory {}: {error}", parent.display()))?;
    }
    fs::write(output, rendered.as_bytes())
        .map_err(|error| format!("write evidence manifest {}: {error}", output.display()))?;
    println!("Evidence manifest written: {}", output.display());
    println!("SHA-256: {}", hash::sha256(rendered.as_bytes()));
    Ok(())
}

fn read_input(path: &Path) -> AppResult<BTreeMap<String, String>> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("read evidence input {}: {error}", path.display()))?;
    parse(&text)
}

fn parse(text: &str) -> AppResult<BTreeMap<String, String>> {
    let mut fields = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        let Some((key, value)) = line.split_once('\t') else {
            return Err(format!("evidence line {} is not two-column TSV", index + 1));
        };
        if key.is_empty() || value.is_empty() || value.contains('\t') {
            return Err(format!(
                "evidence line {} has an empty or invalid field",
                index + 1
            ));
        }
        if fields.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(format!("duplicate evidence field: {key}"));
        }
    }
    validate(&fields)?;
    Ok(fields)
}

fn validate(fields: &BTreeMap<String, String>) -> AppResult<()> {
    for required in REQUIRED {
        if !fields.contains_key(*required) {
            return Err(format!("missing required evidence field: {required}"));
        }
    }
    if fields.get("record_type").map(String::as_str) != Some("release_integration") {
        return Err("record_type must be release_integration".to_owned());
    }
    for field in ["integration_sha", "parent_sha", "context_sha"] {
        let value = &fields[field];
        if !hash::is_lower_hex_sha256(value) {
            return Err(format!("{field} must be lowercase SHA-256"));
        }
    }
    for value in fields["candidate_commits"].split(',').map(str::trim) {
        if !is_commit_sha(value) {
            return Err(format!(
                "candidate_commits contains invalid commit SHA: {value}"
            ));
        }
    }
    let runtime = &fields["runtime_evidence_state"];
    if !(runtime.starts_with("AVAILABLE:") || runtime.starts_with("UNAVAILABLE:")) {
        return Err(
            "runtime_evidence_state must explicitly start with AVAILABLE: or UNAVAILABLE:"
                .to_owned(),
        );
    }
    for checksum in fields["artifact_checksums"].split(',').map(str::trim) {
        let Some((path, digest)) = checksum.rsplit_once('=') else {
            return Err("artifact_checksums entries must be path=sha256".to_owned());
        };
        if path.is_empty() || !hash::is_lower_hex_sha256(digest) {
            return Err("artifact_checksums contains an invalid path or SHA-256".to_owned());
        }
    }
    for value in fields.values() {
        if value.contains("<replace-me>") {
            return Err("evidence input contains unresolved placeholder".to_owned());
        }
    }
    Ok(())
}

fn render(fields: &BTreeMap<String, String>) -> AppResult<String> {
    validate(fields)?;
    let mut output = String::new();
    for field in REQUIRED {
        output.push_str(field);
        output.push('\t');
        output.push_str(&fields[*field]);
        output.push('\n');
    }
    Ok(output)
}

fn is_commit_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input_emits_deterministic_record() {
        let text = include_str!("tests/fixtures/evidence-valid.tsv");
        let fields = parse(text).expect("valid evidence");
        let first = render(&fields).expect("render evidence");
        let second = render(&fields).expect("render evidence");
        assert_eq!(first, second);
        assert!(first.contains("runtime_evidence_state\tUNAVAILABLE:no-new-runtime-proof\n"));
    }

    #[test]
    fn missing_sha_field_fails() {
        let text = include_str!("tests/fixtures/evidence-missing-sha.tsv");
        assert!(parse(text)
            .expect_err("missing SHA fails")
            .contains("context_sha"));
    }

    #[test]
    fn missing_checksum_field_fails() {
        let text = include_str!("tests/fixtures/evidence-valid.tsv").replace(
            "artifact_checksums\t/tmp/report.md=",
            "artifact_checksums\t/tmp/report.md-",
        );
        assert!(parse(&text)
            .expect_err("invalid checksum fails")
            .contains("artifact_checksums"));
    }

    #[test]
    fn unavailable_runtime_evidence_is_explicit() {
        let fields =
            parse(include_str!("tests/fixtures/evidence-valid.tsv")).expect("valid evidence");
        assert!(fields["runtime_evidence_state"].starts_with("UNAVAILABLE:"));
    }
}
