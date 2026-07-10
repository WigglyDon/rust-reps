# Rust Reps Evidence Guidance

Context role: Canonical evidence-class and artifact guidance.
Scope: Product transcripts, installed commands, manual TUI, procgen, tools, and integration records.
Canonical for: What evidence records and what it cannot claim.
Not canonical for: Product law, branch authority, or acceptance decisions.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`../context/README.md`](../context/README.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: [`RUNTIME_EVIDENCE_SCHEMAS.md`](RUNTIME_EVIDENCE_SCHEMAS.md) and external archives.
Update triggers: Evidence class, required field, artifact, or proof limitation changes.

Evidence uses the repository-wide labels `USER_DECISION`, `LIVE_REPO_FACT`,
`RUNTIME_FACT`, `WORKER_CLAIM`, `INFERENCE`, and `UNKNOWN`. A result records the
exact command, conditions, output or digest, status, executable/source SHA, and
unavailable facts. It never upgrades a packet claim into live truth.

## Evidence Classes

- CLI transcript: exact command, stdin, stdout bytes, stderr bytes, exit status,
  executable path, source SHA, environment facts, and unavailable facts.
- Installed command: install command/result, `command -v`, `--version`, smokes,
  exact installed path, and source SHA.
- TUI/manual playtest: source SHA, terminal environment, action sequence,
  observations, Don judgment, limitations; no fake fullscreen/mouse claims.
- Deterministic procgen: template/domain versions, seed, provenance,
  prompt/stimulus/expected digests, duplicate-output result, and source SHA.
- Tool runner: fixed request, executable, timeout/output bound, stdout/stderr,
  status, failure class, and source SHA.
- Release/integration: integration/parent/context SHAs, candidates, gates,
  runtime evidence availability, artifacts/checksums, authority, rollback,
  observability, resources, deploy/push state.

## Artifact Law

Detailed logs belong in checksummed external artifacts, not bloated packet
responses or the repository. Artifacts exclude credentials, private home files,
target output, and unrelated repository content. `UNKNOWN` or `unavailable` is
recorded explicitly instead of omitted. A checksum proves bytes, not truth of
the claims inside.
