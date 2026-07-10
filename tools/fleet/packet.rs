use std::{collections::BTreeMap, fs, path::Path};

use crate::{hash, AppResult, OutputMode};

const BEGIN: &str = "BEGIN RUST REPS PROJECT PACKET";
const END: &str = "END RUST REPS PROJECT PACKET";

const REQUIRED_FIELDS: &[&str] = &[
    "Packet-Version",
    "Packet-ID",
    "Reply-To",
    "From",
    "To",
    "Project-ID",
    "Lane-ID",
    "Goal-ID",
    "Context-SHA",
    "Expected-Base",
    "Worktree",
    "Branch",
    "Authority-State",
    "Expected-Reply-Type",
    "Stale-When",
    "Allowed-Final-Classes",
    "Allowed-Stop-Classes",
];

const REPORT_FIELDS: &[&str] = &[
    "Final-Status",
    "Files-Changed",
    "Validation-Results",
    "Divergence-Classification",
    "Overlap-Classification",
    "Artifact-Path",
    "Artifact-SHA-256",
    "Push-State",
    "Deploy-State",
];

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    fields: BTreeMap<String, String>,
    body: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PacketVerification {
    packet_id: Option<String>,
    errors: Vec<String>,
}

impl PacketVerification {
    fn passed(&self) -> bool {
        self.errors.is_empty()
    }
}

pub(crate) fn command_verify(path: &Path, mode: OutputMode) -> AppResult<()> {
    let verification = verify_file(path);
    let status = if verification.passed() {
        "PASS"
    } else {
        "FAIL"
    };
    match mode {
        OutputMode::Human => {
            println!("Packet verification: {status}");
            println!(
                "Packet-ID: {}",
                verification.packet_id.as_deref().unwrap_or("UNAVAILABLE")
            );
            for error in &verification.errors {
                println!("- {error}");
            }
        }
        OutputMode::Machine => {
            println!("status\t{status}");
            println!(
                "packet_id\t{}",
                verification.packet_id.as_deref().unwrap_or("UNAVAILABLE")
            );
            println!("error_count\t{}", verification.errors.len());
            for error in &verification.errors {
                println!("error\t{error}");
            }
        }
    }
    if verification.passed() {
        Ok(())
    } else {
        Err(format!(
            "packet verification failed with {} error(s)",
            verification.errors.len()
        ))
    }
}

pub(crate) fn validate_generated(text: &str) -> AppResult<()> {
    let verification = verify_text(text);
    if verification.passed() {
        Ok(())
    } else {
        Err(format!(
            "generated packet validation failed: {}",
            verification.errors.join("; ")
        ))
    }
}

pub(crate) fn verify_file(path: &Path) -> PacketVerification {
    match fs::read_to_string(path) {
        Ok(text) => verify_text(&text),
        Err(error) => PacketVerification {
            packet_id: None,
            errors: vec![format!("read packet {}: {error}", path.display())],
        },
    }
}

fn verify_text(text: &str) -> PacketVerification {
    let packet = match parse(text) {
        Ok(packet) => packet,
        Err(error) => {
            return PacketVerification {
                packet_id: None,
                errors: vec![error],
            };
        }
    };
    let packet_id = packet.fields.get("Packet-ID").cloned();
    let mut errors = Vec::new();

    for field in REQUIRED_FIELDS {
        match packet.fields.get(*field) {
            Some(value) if !value.trim().is_empty() => {}
            _ => errors.push(format!("missing required field: {field}")),
        }
    }
    if packet.fields.get("Packet-Version").map(String::as_str) != Some("1") {
        errors.push("Packet-Version must be 1".to_owned());
    }
    if packet.fields.get("Project-ID").map(String::as_str) != Some("rust-reps") {
        errors.push("Project-ID must be rust-reps".to_owned());
    }

    if let (Some(id), Some(reply)) = (
        packet.fields.get("Packet-ID"),
        packet.fields.get("Reply-To"),
    ) {
        if !valid_packet_id(id) {
            errors.push("Packet-ID has invalid characters".to_owned());
        }
        if reply != "NONE" && (!valid_packet_id(reply) || reply == id) {
            errors.push("Reply-To must name a different literal packet ID or NONE".to_owned());
        }
    }

    if let Some(context_sha) = packet.fields.get("Context-SHA") {
        if !hash::is_lower_hex_sha256(context_sha) && !context_sha.starts_with("UNKNOWN_") {
            errors
                .push("Context-SHA must be lowercase SHA-256 or explicit UNKNOWN class".to_owned());
        }
    }
    if let Some(base) = packet.fields.get("Expected-Base") {
        if !is_commit_sha(base) && !base.starts_with("UNKNOWN_") {
            errors.push("Expected-Base must be a commit SHA or explicit UNKNOWN class".to_owned());
        }
    }

    let lower = text.to_ascii_lowercase();
    for placeholder in [
        "<replace-me>",
        "[complete semantic body]",
        "literal immutable id",
    ] {
        if lower.contains(placeholder) {
            errors.push(format!("unresolved operational placeholder: {placeholder}"));
        }
    }

    let push_authorized = packet
        .fields
        .get("Authority-State")
        .map(|value| {
            let value = value.to_ascii_lowercase();
            value.contains("push authorized") || value.contains("authorized to push")
        })
        .unwrap_or(false);
    let explicit_no_push = lower.contains("no push")
        || packet.fields.get("Push-State").map(String::as_str) == Some("no");
    if !push_authorized && !explicit_no_push {
        errors.push("packet requires an explicit no-push statement".to_owned());
    }

    if packet.fields.contains_key("Final-Status") {
        for field in REPORT_FIELDS {
            if !packet.fields.contains_key(*field) {
                errors.push(format!("final report missing field: {field}"));
            }
        }
        if !packet.body.contains("TEACHING") {
            errors.push("final report missing TEACHING section".to_owned());
        }
        if let (Some(status), Some(allowed)) = (
            packet.fields.get("Final-Status"),
            packet.fields.get("Allowed-Final-Classes"),
        ) {
            let allowed = allowed.split(',').map(str::trim).collect::<Vec<_>>();
            if !allowed.contains(&status.as_str()) {
                errors.push(format!("Final-Status {status} is not allowed"));
            }
        }
        if let Some(digest) = packet.fields.get("Artifact-SHA-256") {
            if !hash::is_lower_hex_sha256(digest) {
                errors.push("Artifact-SHA-256 must be lowercase SHA-256".to_owned());
            }
        }
    }

    PacketVerification { packet_id, errors }
}

fn parse(text: &str) -> AppResult<Packet> {
    let lines = text.lines().collect::<Vec<_>>();
    if lines.first().copied() != Some(BEGIN) {
        return Err("packet missing complete BEGIN marker".to_owned());
    }
    if lines.last().copied() != Some(END) {
        return Err("packet missing complete END marker".to_owned());
    }

    let mut index = 1;
    while lines.get(index).copied() == Some("") {
        index += 1;
    }
    let mut fields = BTreeMap::new();
    while index < lines.len() - 1 {
        let line = lines[index];
        if line.is_empty() {
            index += 1;
            break;
        }
        let Some((name, value)) = line.split_once(": ") else {
            break;
        };
        if fields.insert(name.to_owned(), value.to_owned()).is_some() {
            return Err(format!("duplicate packet field: {name}"));
        }
        index += 1;
    }
    let body = lines[index..lines.len() - 1].join("\n");
    Ok(Packet { fields, body })
}

fn valid_packet_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

fn is_commit_sha(value: &str) -> bool {
    (value.len() == 40 || value.len() == 7) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = include_str!("tests/fixtures/packet-valid.txt");

    #[test]
    fn complete_packet_passes() {
        assert_eq!(verify_text(VALID).errors, Vec::<String>::new());
    }

    #[test]
    fn missing_packet_id_fails() {
        let text = include_str!("tests/fixtures/packet-missing-id.txt");
        assert!(verify_text(text)
            .errors
            .iter()
            .any(|error| error.contains("Packet-ID")));
    }

    #[test]
    fn broken_reply_to_fails() {
        let text = include_str!("tests/fixtures/packet-broken-reply.txt");
        assert!(verify_text(text)
            .errors
            .iter()
            .any(|error| error.contains("Reply-To")));
    }

    #[test]
    fn missing_context_sha_fails() {
        let text = include_str!("tests/fixtures/packet-missing-context.txt");
        assert!(verify_text(text)
            .errors
            .iter()
            .any(|error| error.contains("Context-SHA")));
    }

    #[test]
    fn unresolved_placeholder_fails() {
        let text = include_str!("tests/fixtures/packet-placeholder.txt");
        assert!(verify_text(text)
            .errors
            .iter()
            .any(|error| error.contains("placeholder")));
    }

    #[test]
    fn fragment_without_end_marker_fails() {
        let text = include_str!("tests/fixtures/packet-fragment.txt");
        assert!(verify_text(text)
            .errors
            .iter()
            .any(|error| error.contains("END marker")));
    }

    #[test]
    fn missing_no_push_statement_fails_when_required() {
        let text = include_str!("tests/fixtures/packet-missing-no-push.txt");
        assert!(verify_text(text)
            .errors
            .iter()
            .any(|error| error.contains("no-push")));
    }
}
