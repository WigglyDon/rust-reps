# Current Project State

Context role: Canonical current-state ledger.
Scope: Accepted product state, live lineages, active lanes, gates, blockers, and unknowns.
Canonical for: The shared current-state snapshot on local canonical master.
Not canonical for: Product behavior details, full history, or decision rationale.
Inherits: [`README.md`](README.md) and [`../../AGENTS.md`](../../AGENTS.md).
Current-state owner: This document.
Related evidence: [`../evidence/README.md`](../evidence/README.md) and live Git/source.
Update triggers: Accepted-main change, lane transition, authority change, blocker resolution, or gate change.

## Product And Lineage

- `LIVE_REPO_FACT`: Rust Reps is one Git repository and one Cargo package at
  `/home/don/rust-reps`; package and binary name are `rust-reps`.
- `LIVE_REPO_FACT`: canonical local `master` fast-forwarded from
  `5f8a1d9504234f909760d55e026c46638aad602a` through the accepted infrastructure
  lineage ending at `34874d51a6ec54a5ec219f9f2ca99d1449eadf6e`.
- `LIVE_REPO_FACT`: the integrated `infra/project-context-fleet-v1` worktree and
  branch were retired after the fast-forward; parked donor worktrees remain.
- `LIVE_REPO_FACT`: Static Core Task / Guidance Schema Split V1 was accepted and
  fast-forwarded through candidate `1ae5da321a7b1aeb0265a6856be7f8db7d1325f1`;
  its lane worktree and branch were retired after integration.
- `UNKNOWN`: no remote is configured, so remote-tracking identity and remote
  currentness are unavailable. `gh` exists but was not authenticated during discovery.
- `USER_DECISION`: Cross-Lingual Guidance and Presentation Architecture V1 is
  accepted. Its first inert source-only boundary is integrated; no product
  behavior or authority changed.

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
curriculum/catalog own normal content and admission; procgen owns inert core-task
source and pure expansion; guidance owns inert authored instruction source only;
engine owns session truth; answer owns Exact; validators and tool runner own
closed non-exact capability seams; client owns portable UI state; TUI owns
terminal projection; CLI/practice/presentation/main own launch and plain terminal
adaptation; lab owns synthetic stress data only.

## Core Task And Guidance State

- `LIVE_REPO_FACT`: static procgen source and an owned deterministic expansion
  core exist under `src/curriculum/procgen*` and are intentionally inert.
- `LIVE_REPO_FACT`: `TemplateSpec` now separates `instruction_key`, `stimulus`,
  and `expected`; `ExpandedTask` is locale-independent owned output using
  `content_seed` and `stimulus_variant`.
- `LIVE_REPO_FACT`: `DomainValueSpec` carries an optional `meaning_key`; all
  current Rust production values use `meaning_key: None`.
- `LIVE_REPO_FACT`: `curriculum::guidance` contains one inert English pack,
  `en@v1`, with instruction key `copy.exact_line` and exact text
  `Copy this line exactly:`. Guidance owns no expected-answer bytes.
- `LIVE_REPO_FACT`: no catalog, engine, CLI, practice, client, TUI, lab,
  formatter, validator, tool-runner, or presentation product caller admits
  expanded output or resolves guidance.
- `LIVE_REPO_FACT`: no guidance resolver, Ukrainian pack, French curriculum,
  guidance-language setting, interface-language setting, or fallback exists.
- `LIVE_REPO_FACT`: `engine::catalog_exercise_packet_at` hard-codes the Copy
  instruction, and `presentation::print_one_shot_prompt` prints `expected`
  instead of `prompt`. Non-Copy admission is blocked until a separately
  authorized, transcript-tested prompt propagation boundary addresses both.
- `USER_DECISION`: Pure Guidance Resolution V1 is the next planned boundary and
  remains inert until a fresh lane packet authorizes it. Catalog admission,
  settings, fallback, and product callers remain unauthorized.

## Lanes

- Active implementation lanes: none.
- Retired: [`static-guidance-schema-v1`](../lanes/static-guidance-schema-v1.md),
  accepted and integrated into local canonical master.
- Planned but not active: Pure Guidance Resolution V1; Language-Agnostic
  Capability Selector Architecture V1; Tiny Materialization And
  Product-Integration Decision V1.
- Paused: new Ukrainian/French curriculum, validator integration, lab cleanup,
  product settings, interface localization, persistence, and scheduling.
- No guidance language setting currently exists.
- Parked donor/reference: clean legacy worktrees `curriculum-data-seam-v0` and
  `curriculum-sharding-v0`; neither is an active product lane.

## Validation And Authority

Required existing project gates are `cargo fmt --check`, `cargo test`,
`./scripts/proof.sh`, and `git diff --check`; staged work also requires
`git diff --cached --check`. Fleet tooling has independent checks documented in
[`../operations/RELEASE_AND_EVIDENCE.md`](../operations/RELEASE_AND_EVIDENCE.md).

- `LIVE_REPO_FACT`: recursive context and the standalone std-only fleet binary
  behind `scripts/fleet.sh` are accepted on local canonical master for context,
  lane, packet, respawn, queue, and evidence diagnostics/generation.
- `LIVE_REPO_FACT`: the fleet tool is not linked into product Cargo or
  `scripts/proof.sh`, and no product module calls it.

Accepted infrastructure authority remains documentation and standalone local
diagnostics only. Product behavior, dependencies, runtime authority, and release
authority are unchanged; the fleet tool has no product runtime integration.
