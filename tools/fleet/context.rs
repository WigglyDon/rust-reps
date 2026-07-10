use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::{hash, AppResult, OutputMode};

const MANIFEST: &str = "ops/fleet/context-manifest.tsv";
const REPORTED_SHA: &str = "ops/fleet/context-sha256.txt";

const REQUIRED_FILES: &[&str] = &[
    "AGENTS.md",
    "docs/INDEX.md",
    "docs/context/README.md",
    "docs/context/CURRENT_STATE.md",
    "docs/context/CONTEXT_MATRIX.md",
    "docs/context/PROJECT_HISTORY.md",
    "docs/context/DECISION_LOG.md",
    "docs/lanes/lane-registry.md",
    "docs/process/PACKET_PROTOCOL.md",
    "docs/process/HANDOFF_AND_RESPAWN.md",
    "docs/evidence/README.md",
    "docs/evidence/RUNTIME_EVIDENCE_SCHEMAS.md",
    "docs/operations/RELEASE_AND_EVIDENCE.md",
    "ops/fleet/integration-queue.tsv",
];

const SCOPED_AGENTS: &[&str] = &[
    "docs/AGENTS.md",
    "src/curriculum/AGENTS.md",
    "src/engine/AGENTS.md",
    "src/client/AGENTS.md",
    "src/tui/AGENTS.md",
    "scripts/AGENTS.md",
    "tools/AGENTS.md",
    "ops/AGENTS.md",
];

const REQUIRED_HEADERS: &[&str] = &[
    "Context role:",
    "Scope:",
    "Canonical for:",
    "Not canonical for:",
    "Inherits:",
    "Current-state owner:",
    "Related evidence:",
    "Update triggers:",
];

const PLACEHOLDERS: &[&str] = &["TODO", "TBD", "FIXME", "CHANGEME"];
const VALID_LABELS: &[&str] = &[
    "USER_DECISION",
    "LIVE_REPO_FACT",
    "RUNTIME_FACT",
    "WORKER_CLAIM",
    "INFERENCE",
    "UNKNOWN",
];

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Verification {
    pub(crate) context_sha: Option<String>,
    pub(crate) errors: Vec<String>,
}

impl Verification {
    pub(crate) fn passed(&self) -> bool {
        self.errors.is_empty()
    }
}

pub(crate) fn command_verify(repo: &Path, mode: OutputMode) -> AppResult<()> {
    let verification = verify_repo(repo);
    print_verification(&verification, mode);
    if verification.passed() {
        Ok(())
    } else {
        Err(format!(
            "context verification failed with {} error(s)",
            verification.errors.len()
        ))
    }
}

pub(crate) fn command_manifest(repo: &Path, mode: OutputMode) -> AppResult<()> {
    let manifest_path = repo.join(MANIFEST);
    let existing = parse_manifest(&manifest_path)?;
    let mut paths: Vec<String> = existing.into_keys().collect();
    paths.sort();

    let mut manifest = String::new();
    for relative in paths {
        let path = repo.join(&relative);
        if !path.is_file() {
            return Err(format!("context member missing: {relative}"));
        }
        let digest = hash::file_sha256(&path)?;
        manifest.push_str(&digest);
        manifest.push('\t');
        manifest.push_str(&relative);
        manifest.push('\n');
    }
    fs::write(&manifest_path, manifest.as_bytes())
        .map_err(|error| format!("write {}: {error}", manifest_path.display()))?;
    let context_sha = hash::sha256(manifest.as_bytes());
    fs::write(repo.join(REPORTED_SHA), format!("{context_sha}\n"))
        .map_err(|error| format!("write reported Context-SHA: {error}"))?;

    match mode {
        OutputMode::Human => println!("Context manifest regenerated: {context_sha}"),
        OutputMode::Machine => {
            println!("status\tPASS");
            println!("context_sha\t{context_sha}");
        }
    }
    Ok(())
}

pub(crate) fn verify_repo(repo: &Path) -> Verification {
    let mut errors = Vec::new();
    for relative in REQUIRED_FILES.iter().chain(SCOPED_AGENTS.iter()) {
        if !repo.join(relative).is_file() {
            errors.push(format!("missing required file: {relative}"));
        }
    }

    let manifest_path = repo.join(MANIFEST);
    let manifest = match parse_manifest(&manifest_path) {
        Ok(manifest) => Some(manifest),
        Err(error) => {
            errors.push(error);
            None
        }
    };

    if let Some(manifest) = &manifest {
        verify_required_members(manifest, &mut errors);
        verify_manifest_files(repo, manifest, &mut errors);
        verify_headers_and_owners(repo, manifest, &mut errors);
        verify_links(repo, manifest, &mut errors);
        verify_labels_and_placeholders(repo, manifest, &mut errors);
    }
    verify_scoped_agents(repo, &mut errors);
    verify_lanes(repo, &mut errors);
    verify_authority(repo, &mut errors);
    verify_index_owners(repo, &mut errors);

    let context_sha = fs::read(&manifest_path)
        .ok()
        .map(|bytes| hash::sha256(&bytes));
    match (&context_sha, fs::read_to_string(repo.join(REPORTED_SHA))) {
        (Some(calculated), Ok(reported)) if reported.trim() == calculated => {}
        (Some(calculated), Ok(reported)) => errors.push(format!(
            "reported Context-SHA mismatch: calculated {calculated}, reported {}",
            reported.trim()
        )),
        (Some(_), Err(_)) => errors.push(format!("missing reported Context-SHA: {REPORTED_SHA}")),
        (None, _) => {}
    }

    Verification {
        context_sha,
        errors,
    }
}

fn parse_manifest(path: &Path) -> AppResult<BTreeMap<String, String>> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("read context manifest {}: {error}", path.display()))?;
    let mut manifest = BTreeMap::new();
    let mut previous = None::<String>;
    for (line_number, line) in text.lines().enumerate() {
        let Some((digest, relative)) = line.split_once('\t') else {
            return Err(format!(
                "manifest line {} is not two-column TSV",
                line_number + 1
            ));
        };
        if !hash::is_lower_hex_sha256(digest) {
            return Err(format!(
                "manifest line {} has invalid SHA-256",
                line_number + 1
            ));
        }
        if relative.is_empty() || Path::new(relative).is_absolute() || relative.contains("..") {
            return Err(format!("manifest line {} has unsafe path", line_number + 1));
        }
        if let Some(previous) = &previous {
            if previous.as_str() >= relative {
                return Err("context manifest paths are not uniquely bytewise sorted".to_owned());
            }
        }
        previous = Some(relative.to_owned());
        if manifest
            .insert(relative.to_owned(), digest.to_owned())
            .is_some()
        {
            return Err(format!("duplicate context manifest path: {relative}"));
        }
    }
    if manifest.is_empty() {
        return Err("context manifest is empty".to_owned());
    }
    Ok(manifest)
}

fn verify_required_members(manifest: &BTreeMap<String, String>, errors: &mut Vec<String>) {
    for required in REQUIRED_FILES.iter().chain(SCOPED_AGENTS.iter()) {
        if !manifest.contains_key(*required) {
            errors.push(format!(
                "required context member absent from manifest: {required}"
            ));
        }
    }
}

fn verify_manifest_files(
    repo: &Path,
    manifest: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    for (relative, expected) in manifest {
        let path = repo.join(relative);
        match hash::file_sha256(&path) {
            Ok(actual) if &actual == expected => {}
            Ok(actual) => errors.push(format!(
                "stale manifest entry: {relative} expected {expected}, found {actual}"
            )),
            Err(error) => errors.push(error),
        }
    }
}

fn carries_context_header(relative: &str) -> bool {
    relative.ends_with(".md")
        && (relative.starts_with("docs/context/")
            || relative == "docs/lanes/lane-registry.md"
            || relative == "docs/lanes/master-infrastructure.md"
            || relative.starts_with("docs/process/")
            || relative.starts_with("docs/evidence/")
            || relative.starts_with("docs/operations/"))
}

fn verify_headers_and_owners(
    repo: &Path,
    manifest: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    let mut owners = BTreeMap::<String, String>::new();
    for relative in manifest.keys().filter(|path| carries_context_header(path)) {
        let Ok(text) = fs::read_to_string(repo.join(relative)) else {
            continue;
        };
        for header in REQUIRED_HEADERS {
            if !text.lines().any(|line| line.starts_with(header)) {
                errors.push(format!("{relative} missing required header {header}"));
            }
        }
        if let Some(owner) = text
            .lines()
            .find_map(|line| line.strip_prefix("Canonical for:"))
            .map(str::trim)
        {
            if owner.is_empty() {
                errors.push(format!("{relative} has empty Canonical for header"));
            } else if let Some(earlier) = owners.insert(owner.to_owned(), relative.to_owned()) {
                errors.push(format!(
                    "duplicate canonical owner declaration in {earlier} and {relative}: {owner}"
                ));
            }
        }
    }
}

fn verify_links(repo: &Path, manifest: &BTreeMap<String, String>, errors: &mut Vec<String>) {
    for relative in manifest.keys().filter(|path| path.ends_with(".md")) {
        let Ok(text) = fs::read_to_string(repo.join(relative)) else {
            continue;
        };
        for target in markdown_targets(&text) {
            if target.starts_with("http://")
                || target.starts_with("https://")
                || target.starts_with("mailto:")
                || target.starts_with('#')
            {
                continue;
            }
            let target = target.split('#').next().unwrap_or("");
            if target.is_empty() {
                continue;
            }
            let parent = Path::new(relative)
                .parent()
                .unwrap_or_else(|| Path::new(""));
            if !repo.join(parent).join(target).exists() {
                errors.push(format!("broken relative link in {relative}: {target}"));
            }
        }
    }
}

fn markdown_targets(text: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else {
            break;
        };
        targets.push(rest[..end].to_owned());
        rest = &rest[end + 1..];
    }
    targets
}

fn verify_labels_and_placeholders(
    repo: &Path,
    manifest: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    for relative in manifest.keys() {
        let Ok(text) = fs::read_to_string(repo.join(relative)) else {
            continue;
        };
        for placeholder in PLACEHOLDERS {
            if text.contains(placeholder) {
                errors.push(format!(
                    "unresolved operational placeholder in {relative}: {placeholder}"
                ));
            }
        }
        for token in uppercase_tokens(&text) {
            if (token.ends_with("_FACT") || token.ends_with("_CLAIM"))
                && !VALID_LABELS.contains(&token.as_str())
            {
                errors.push(format!("invalid evidence label in {relative}: {token}"));
            }
        }
    }
}

fn uppercase_tokens(text: &str) -> BTreeSet<String> {
    text.split(|character: char| !(character.is_ascii_uppercase() || character == '_'))
        .filter(|token| token.len() > 1)
        .map(str::to_owned)
        .collect()
}

fn verify_scoped_agents(repo: &Path, errors: &mut Vec<String>) {
    for relative in SCOPED_AGENTS {
        let Ok(text) = fs::read_to_string(repo.join(relative)) else {
            continue;
        };
        for required in ["AGENTS.md", "INDEX.md", "CURRENT_STATE.md", "subsystems/"] {
            if !text.contains(required) {
                errors.push(format!(
                    "scoped law {relative} missing inheritance/link: {required}"
                ));
            }
        }
    }
}

fn verify_lanes(repo: &Path, errors: &mut Vec<String>) {
    let path = repo.join("docs/lanes/lane-registry.md");
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    let active = lane_ids_in_section(&text, "## Active lanes");
    let retired = lane_ids_in_section(&text, "## Retired lanes");
    for lane in active.intersection(&retired) {
        errors.push(format!("lane is both active and retired: {lane}"));
    }
    for lane in active {
        let page = repo.join("docs/lanes").join(format!("{lane}.md"));
        if !page.is_file() {
            errors.push(format!("active lane missing page: {lane}"));
            continue;
        }
        if let Ok(page_text) = fs::read_to_string(page) {
            if !page_text.contains(&format!("Lane ID: `{lane}`")) {
                errors.push(format!("active lane page does not declare lane ID: {lane}"));
            }
        }
    }
}

fn lane_ids_in_section(text: &str, heading: &str) -> BTreeSet<String> {
    let Some(section) = text.split(heading).nth(1) else {
        return BTreeSet::new();
    };
    let body = section.split("\n## ").next().unwrap_or(section);
    body.lines()
        .filter_map(|line| line.strip_prefix("| `"))
        .filter_map(|line| line.split('`').next())
        .map(str::to_owned)
        .collect()
}

fn verify_authority(repo: &Path, errors: &mut Vec<String>) {
    let dangerous = [
        "worker branches pushed: yes",
        "canonical master changed: yes",
        "deploy performed: yes",
        "force-push performed: yes",
        "destructive operation performed: yes",
    ];
    for relative in REQUIRED_FILES.iter().chain(SCOPED_AGENTS.iter()) {
        if let Ok(text) = fs::read_to_string(repo.join(relative)) {
            let lower = text.to_ascii_lowercase();
            for phrase in dangerous {
                if lower.contains(phrase) {
                    errors.push(format!(
                        "accidental authority enabled in {relative}: {phrase}"
                    ));
                }
            }
        }
    }
}

fn verify_index_owners(repo: &Path, errors: &mut Vec<String>) {
    if let Ok(index) = fs::read_to_string(repo.join("docs/INDEX.md")) {
        for owner in ["context/CURRENT_STATE.md", "context/DECISION_LOG.md"] {
            if !index.contains(owner) {
                errors.push(format!(
                    "docs/INDEX.md missing canonical owner link: {owner}"
                ));
            }
        }
    }
}

fn print_verification(verification: &Verification, mode: OutputMode) {
    let status = if verification.passed() {
        "PASS"
    } else {
        "FAIL"
    };
    match mode {
        OutputMode::Human => {
            println!("Context verification: {status}");
            if let Some(context_sha) = &verification.context_sha {
                println!("Context-SHA: {context_sha}");
            }
            for error in &verification.errors {
                println!("- {error}");
            }
        }
        OutputMode::Machine => {
            println!("status\t{status}");
            println!(
                "context_sha\t{}",
                verification.context_sha.as_deref().unwrap_or("UNAVAILABLE")
            );
            println!("error_count\t{}", verification.errors.len());
            for error in &verification.errors {
                println!("error\t{error}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempDir;

    fn write(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent");
        }
        fs::write(path, text).expect("write fixture");
    }

    fn context_document(title: &str, extra: &str) -> String {
        format!(
            "# {title}\n\nContext role: Fixture.\nScope: Fixture.\nCanonical for: {title}.\nNot canonical for: Runtime.\nInherits: Root fixture.\nCurrent-state owner: Fixture state.\nRelated evidence: Fixture.\nUpdate triggers: Fixture changes.\n\n{extra}\n"
        )
    }

    fn valid_fixture() -> TempDir {
        let temp = TempDir::new("context");
        let repo = temp.path();
        write(&repo.join("AGENTS.md"), "# Root\n");
        write(
            &repo.join("docs/INDEX.md"),
            "[state](context/CURRENT_STATE.md) [decision](context/DECISION_LOG.md)\n",
        );
        for relative in REQUIRED_FILES {
            let path = repo.join(relative);
            if path.exists() {
                continue;
            }
            if relative.ends_with("integration-queue.tsv") {
                write(&path, "lane_id\n");
            } else if relative.ends_with(".md") {
                let title = relative.replace('/', "-");
                write(&path, &context_document(&title, ""));
            } else {
                write(&path, "fixture\n");
            }
        }
        for relative in SCOPED_AGENTS {
            write(
                &repo.join(relative),
                "AGENTS.md docs/INDEX.md CURRENT_STATE.md subsystems/\n",
            );
        }
        write(
            &repo.join("docs/lanes/lane-registry.md"),
            &context_document("Lane Registry", "## Active lanes\n\n## Retired lanes\n"),
        );
        write(
            &repo.join("docs/context/CURRENT_STATE.md"),
            &context_document("Current State", ""),
        );
        write(
            &repo.join("docs/context/DECISION_LOG.md"),
            &context_document("Decision Log", ""),
        );

        let mut paths: Vec<&str> = REQUIRED_FILES
            .iter()
            .chain(SCOPED_AGENTS.iter())
            .copied()
            .collect();
        paths.sort_unstable();
        paths.dedup();
        let mut manifest = String::new();
        for relative in paths {
            let digest = hash::file_sha256(&repo.join(relative)).expect("hash fixture");
            manifest.push_str(&format!("{digest}\t{relative}\n"));
        }
        write(&repo.join(MANIFEST), &manifest);
        write(
            &repo.join(REPORTED_SHA),
            &format!("{}\n", hash::sha256(manifest.as_bytes())),
        );
        temp
    }

    #[test]
    fn valid_graph_passes() {
        let temp = valid_fixture();
        let result = verify_repo(temp.path());
        assert_eq!(result.errors, Vec::<String>::new());
    }

    #[test]
    fn missing_required_file_fails() {
        let temp = valid_fixture();
        fs::remove_file(temp.path().join("docs/context/CONTEXT_MATRIX.md"))
            .expect("remove fixture file");
        assert!(verify_repo(temp.path()).errors.iter().any(|error| {
            error.contains("missing required file: docs/context/CONTEXT_MATRIX.md")
        }));
    }

    #[test]
    fn broken_relative_link_fails() {
        let temp = valid_fixture();
        let path = temp.path().join("docs/context/README.md");
        let mut text = fs::read_to_string(&path).expect("read fixture");
        text.push_str("[broken](missing.md)\n");
        fs::write(path, text).expect("write broken link");
        assert!(verify_repo(temp.path()).errors.iter().any(|error| {
            error.contains("broken relative link") || error.contains("stale manifest entry")
        }));
    }

    #[test]
    fn stale_manifest_fails() {
        let temp = valid_fixture();
        fs::write(temp.path().join("AGENTS.md"), "changed\n").expect("stale fixture");
        assert!(verify_repo(temp.path())
            .errors
            .iter()
            .any(|error| error.contains("stale manifest entry: AGENTS.md")));
    }

    #[test]
    fn duplicate_active_and_retired_lane_fails() {
        let temp = valid_fixture();
        let path = temp.path().join("docs/lanes/lane-registry.md");
        write(
            &path,
            &context_document(
                "Lane Registry",
                "## Active lanes\n| `same-lane` | active |\n\n## Retired lanes\n| `same-lane` | retired |\n",
            ),
        );
        assert!(verify_repo(temp.path())
            .errors
            .iter()
            .any(|error| error.contains("both active and retired")));
    }

    #[test]
    fn invalid_evidence_label_fails() {
        let temp = valid_fixture();
        let path = temp.path().join("docs/context/README.md");
        let mut text = fs::read_to_string(&path).expect("read fixture");
        text.push_str("BAD_FACT: invalid\n");
        fs::write(path, text).expect("write invalid label");
        assert!(verify_repo(temp.path())
            .errors
            .iter()
            .any(|error| error.contains("invalid evidence label")));
    }

    #[test]
    fn missing_required_header_fails() {
        let temp = valid_fixture();
        let path = temp.path().join("docs/context/README.md");
        let text = fs::read_to_string(&path)
            .expect("read fixture")
            .replace("Scope: Fixture.\n", "");
        fs::write(path, text).expect("write missing header");
        assert!(verify_repo(temp.path())
            .errors
            .iter()
            .any(|error| error.contains("missing required header Scope:")));
    }
}
