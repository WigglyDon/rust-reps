# Current Project State

Context role: Canonical current-state ledger.
Scope: Accepted product state, live lineages, active lanes, gates, blockers, and unknowns.
Canonical for: The shared current-state snapshot in this infrastructure candidate.
Not canonical for: Product behavior details, full history, or decision rationale.
Inherits: [`README.md`](README.md) and [`../../AGENTS.md`](../../AGENTS.md).
Current-state owner: This document.
Related evidence: [`../evidence/README.md`](../evidence/README.md) and live Git/source.
Update triggers: Accepted-main change, lane transition, authority change, blocker resolution, or gate change.

## Product And Lineage

- `LIVE_REPO_FACT`: Rust Reps is one Git repository and one Cargo package at
  `/home/don/rust-reps`; package and binary name are `rust-reps`.
- `LIVE_REPO_FACT`: canonical local `master` was clean at infrastructure-lane
  creation, at `5f8a1d9504234f909760d55e026c46638aad602a` (`Add pure procgen expansion core`).
- `LIVE_REPO_FACT`: `infra/project-context-fleet-v1` was created from that exact
  commit in `/home/don/rust-reps-infra-v1`. Git remains the authority for its
  moving candidate tip.
- `UNKNOWN`: no remote is configured, so remote-tracking identity and remote
  currentness are unavailable. `gh` exists but was not authenticated during discovery.
- `USER_DECISION`: feature progression is paused while this infrastructure
  candidate is reviewed. Product behavior and authority must not change.

## Accepted Product State

- `LIVE_REPO_FACT`: the catalog contains 12 ordered normal Copy reps with
  static metadata; all use `ExactTrimFinalNewline` and `StateTransition::None`.
- `LIVE_REPO_FACT`: plain terminal stdout/stderr and exit behavior remain the
  baseline oracle; interactive default is the sealed fullscreen TUI client.
- `LIVE_REPO_FACT`: exact checking removes at most one final LF or CRLF and
  normalizes nothing else.
- `LIVE_REPO_FACT`: correct and wrong submissions increment attempts; skip
  advances without becoming a skill attempt. XP remains undisplayed metadata.
- `LIVE_REPO_FACT`: approved direct product dependencies are only `ratatui`
  and `crossterm`.

## Ownership Snapshot

The canonical matrix is [`CONTEXT_MATRIX.md`](CONTEXT_MATRIX.md). In brief,
curriculum/catalog own normal content and admission; procgen owns inert source
and pure expansion; engine owns session truth; answer owns Exact; validators
and tool runner own closed non-exact capability seams; client owns portable UI
state; TUI owns terminal projection; CLI/practice/presentation/main own launch
and plain terminal adaptation; lab owns synthetic stress data only.

## Procgen State And Gates

- `LIVE_REPO_FACT`: static procgen source and an owned deterministic expansion
  core exist under `src/curriculum/procgen*` and are intentionally inert.
- `LIVE_REPO_FACT`: no catalog, engine, CLI, practice, client, TUI, lab,
  formatter, validator, or tool-runner product caller admits expanded output.
- `LIVE_REPO_FACT`: `engine::catalog_exercise_packet_at` hard-codes the Copy
  instruction, and `presentation::print_one_shot_prompt` prints `expected`
  instead of `prompt`. Non-Copy admission is blocked until a separately
  authorized, transcript-tested prompt propagation boundary addresses both.

## Lanes

- Active: [`master-infrastructure`](../lanes/master-infrastructure.md).
- Paused by `USER_DECISION`: cross-lingual guidance/presentation architecture,
  Tiny Checked Procgen Materialization V1, selector work, and new curriculum.
- Parked donor/reference: clean legacy worktrees `curriculum-data-seam-v0` and
  `curriculum-sharding-v0`; neither is an active product lane.
- `UNKNOWN`: no repository/worktree evidence establishes a live cross-lingual
  supervisor or worker. Chat intent is not a live lane.

## Validation And Authority

Required existing project gates are `cargo fmt --check`, `cargo test`,
`./scripts/proof.sh`, and `git diff --check`; staged work also requires
`git diff --cached --check`. Fleet tooling has independent checks documented in
[`../operations/RELEASE_AND_EVIDENCE.md`](../operations/RELEASE_AND_EVIDENCE.md).

Infrastructure candidate authority is documentation and standalone local
diagnostics only. It has no product runtime integration, release, push, merge,
deployment, deletion, or canonical-master authority.
