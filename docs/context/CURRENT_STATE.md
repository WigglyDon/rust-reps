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
- `LIVE_REPO_FACT`: Pure Guidance Resolution V1 was accepted and fast-forwarded
  through candidate `b61d509e19c7627d63d062fe31c95b1ce579b928`; its lane
  worktree was removed and its branch deleted after integration.
- `LIVE_REPO_FACT`: Static Capability And SetPlan Schema V1 was accepted and
  fast-forwarded through candidate `bff7107d6485c06773af25987b321a5494b4e47e`;
  its lane worktree was removed and its branch deleted after integration.
- `LIVE_REPO_FACT`: Pure Deterministic PracticePlan Builder V1 was accepted and
  fast-forwarded through candidate `0521e448dfa204993c6a65edc51cb9018ef9b185`;
  its lane worktree was removed and its branch deleted after integration.
- `LIVE_REPO_FACT`: Tiny Multi-Operation Rust Set Materialization V1 was
  accepted and fast-forwarded through candidate
  `67e0e55ee7ebc7970c81c41123c1c994eccbf12c`; its lane worktree was
  removed and its branch deleted after integration.
- `LIVE_REPO_FACT`: Resolved Core Task And Orchestration Contract V1 was
  accepted and fast-forwarded through candidate
  `e4ab24cf9447601abc79f8a2bcead2e588252483`; its lane worktree was
  removed and its branch deleted after integration.
- `LIVE_REPO_FACT`: Truthful Task Projection Contract V1 was accepted and
  fast-forwarded through candidate
  `ad56f9725eab16203f44e8518e31e230c2d3987b`; its lane worktree was removed
  and its branch deleted after integration.
- `UNKNOWN`: no remote is configured, so remote-tracking identity and remote
  currentness are unavailable. `gh` exists but was not authenticated during discovery.
- `USER_DECISION`: Cross-Lingual Guidance and Presentation Architecture V1 is
  accepted. Its first inert source-only boundary is integrated; no product
  behavior or authority changed.
- `USER_DECISION`: the second inert cross-lingual boundary, Pure Guidance
  Resolution V1, is accepted; product behavior and authority remain unchanged.
- `USER_DECISION`: Language-Agnostic Capability Selector Architecture V1 and
  its first three inert implementation boundaries are accepted: static
  capability/SetPlan source, pure deterministic PracticePlan construction, and
  one checked five-task materialization. Those accepted boundaries remain
  inert; product behavior and authority remain unchanged.
- `USER_DECISION`: Product Integration Decision V1 and both prerequisites,
  Resolved Core Task And Orchestration Contract V1 and Truthful Task Projection
  Contract V1, are accepted and integrated.
- `USER_DECISION`: Varied Deterministic Rust Practice Flow V1 and Beginner
  Concept Scaffolding V1 are accepted product boundaries.
- `USER_DECISION`: Problem-First Task Hierarchy and Zero-Knowledge Brief V1 is
  the selected next product boundary.
- `LIVE_REPO_FACT`: canonical local `master` fast-forwarded to candidate
  `a0bdbdc00751b08d49b36956fb82600001348217`; the integrated feature worktree
  and branch were then retired without force.
- `LIVE_REPO_FACT`: Don's real-TTY playtest passed with observations. The
  screenshot ambiguity gate classified local Wrong feedback as
  `NONBLOCKING_LOCAL_FEEDBACK_STYLE` and both Repair focus ranges as
  `NO_FOCUS_DEFECT_FOUND`.
- `LIVE_REPO_FACT`: the varied-flow feature is integrated and its implementation
  lane is closed. The integration queue remains valid, empty, and fresh.
- `LIVE_REPO_FACT`: `problem-first-curriculum-v1` is registered on canonical
  master. Its exact declared worktree and branch now exist, the worker is clean,
  and lane-doctor reported `READY`. Implementation remains unstarted, no
  candidate exists, and no integration-queue row exists.
- `LIVE_REPO_FACT`: canonical installed-command proof for the closed integrated
  state was completed from local `master` and is retained in the external
  integration evidence. Registration changes no product behavior or installed
  command; no push or deployment occurred.

## Accepted Product State

- `LIVE_REPO_FACT`: the normal catalog contains 22 ordered tasks with stable
  IDs 001-022. IDs 001-012 retain their accepted Copy behavior; IDs 013-022 are
  two reviewed correlated rows in Copy, Fill, Produce, Repair, Transform order.
- `LIVE_REPO_FACT`: plain terminal stdout/stderr and exit behavior remain the
  baseline oracle; interactive default is the sealed fullscreen TUI client.
- `LIVE_REPO_FACT`: exact checking removes at most one final LF or CRLF and
  normalizes nothing else.
- `LIVE_REPO_FACT`: current product task packets keep resolved instruction,
  support presentation, exact stimulus, and private expected-answer truth
  separate. Copy and controlled Fill use `ExactTrimFinalNewline` with formatting
  disabled; Produce, Repair, and Transform use `RustfmtExact` with F1 formatting
  enabled.
- `LIVE_REPO_FACT`: plain, one-shot, client, and TUI task projection reads
  instruction and stimulus, never expected-answer truth. The TUI aligns editor
  and stimulus source geometry and marks soft stimulus continuations with an
  out-of-source `↪ ` gutter.
- `LIVE_REPO_FACT`: correct and wrong submissions increment attempts; skip
  advances without becoming a skill attempt. XP remains undisplayed metadata.
- `LIVE_REPO_FACT`: normal interactive practice selects coherent deterministic
  five-task rounds and continues until Esc. Correct or Skip at 5/5 opens 1/5 of
  the next row; Wrong remains local. Plain, automatic fallback, and one-shot
  execution remain finite.
- `LIVE_REPO_FACT`: beginner varied tasks carry static support stages and
  Unicode-safe stimulus focus ranges. F2 toggles resolved concept help in
  transient client state without submission, validation, attempt, skip, XP, or
  engine-state effects; unsupported tasks report `help unavailable`.
- `LIVE_REPO_FACT`: approved direct product dependencies are only `ratatui`
  and `crossterm`.

## Ownership Snapshot

The canonical matrix is [`CONTEXT_MATRIX.md`](CONTEXT_MATRIX.md). In brief,
curriculum/catalog own normal content, admission, and locale-independent core
task records and static task support; procgen owns reviewed source rows and pure
expansion; planner owns SetPlan source and deterministic planning over admitted
candidate summaries; guidance owns authored prose and pure resolution;
orchestration owns product lookup, resolution, planning, and packet construction;
engine owns session truth over resolved packets; answer owns Exact; validators
and tool runner own RustfmtExact dispatch and fixed rustfmt execution; client
owns portable editor and transient help state; TUI owns terminal input and
projection; CLI/practice/presentation/main own launch and finite plain terminal
adaptation; lab owns synthetic stress data only.

## Core Task And Guidance State

- `LIVE_REPO_FACT`: static procgen source and an owned deterministic expansion
  core exist under `src/curriculum/procgen*` and are intentionally inert.
- `LIVE_REPO_FACT`: `TemplateSpec` now separates `instruction_key`, `stimulus`,
  and `expected`; `ExpandedTask` is locale-independent owned output using
  `content_seed` and `stimulus_variant`.
- `LIVE_REPO_FACT`: `DomainValueSpec` carries an optional `meaning_key`; all
  current Rust production values use `meaning_key: None`.
- `LIVE_REPO_FACT`: `curriculum::guidance` contains two inert five-key proof
  packs, `en@v1` and `uk@v1`, for Copy, Fill, Produce, Repair, and Transform.
  Both gloss sets are empty; exact resolution is tested for every materialized
  task, and Ukrainian text still requires human linguistic/product review
  before any product admission.
- `LIVE_REPO_FACT`: `curriculum::guidance::resolution` provides pure,
  deterministic, exact, case-sensitive resolution. `GuidanceProvenance` records
  a source-neutral core task ID, locale, pack version, and instruction key;
  `ResolvedGuidance` contains only provenance and exact instruction.
- `LIVE_REPO_FACT`: current admitted `DrillSpec` records explicitly separate
  instruction key, static guidance assignments, stimulus, expected response,
  optional capability ID, and optional generated provenance. Catalog canonical
  identity remains product identity; generated identity remains optional source
  provenance.
- `LIVE_REPO_FACT`: the original twelve reps retain IDs 001-012, canonical IDs,
  metadata, stimulus, expected bytes, Copy operation, Exact answer rule, disabled
  formatting, and no generated provenance.
- `LIVE_REPO_FACT`: `src/orchestration.rs` defines explicit initial `en@v1`
  guidance constants, pure `resolve_core_task` construction, and current
  product lookup/collection/session construction. `ResolvedCoreTask` separates
  stable identity, localized instruction, exact stimulus, exact expected
  response, unchanged metadata, and guidance provenance.
- `LIVE_REPO_FACT`: the initial guidance constants are neither fallback nor
  user settings. Orchestration is the sole current product caller for guidance
  resolution and supplies already-resolved packets to engine sessions.
- `LIVE_REPO_FACT`: `GuidanceError` explicitly classifies missing or duplicate
  packs and instructions, duplicate assignments, missing slots or meaning keys,
  and missing or duplicate glosses. There is no fallback, locale normalization,
  translation, network access, or external tool call.
- `LIVE_REPO_FACT`: the same locale-independent `ExpandedTask` supplies both
  proof locales. Core identity, content seed, assignments, stimulus, expected
  response, answer rule, concepts, constraints, XP, and transition do not change
  by guidance locale. Guidance owns no expected-answer bytes.
- `LIVE_REPO_FACT`: `src/curriculum/procgen/materialized.rs` contains two
  explicit reviewed correlated rows. Catalog IDs 013-022 admit their ten static
  materialized tasks in row-major Copy, Fill, Produce, Repair, Transform order;
  runtime product code selects admitted IDs and does not expand templates.
- `LIVE_REPO_FACT`: `src/curriculum/support.rs` carries one opaque primary
  concept, Introduce/Guided/Practice stages, deterministic stimulus focus ranges,
  and static help examples for IDs 013-022. Guidance resolves visible prose
  before client/TUI construction, and expected responses remain private.
- `LIVE_REPO_FACT`: no French curriculum, guidance-language setting,
  interface-language setting, fallback, or generated catalog admission exists.
  The Ukrainian proof source remains product-inert.
- `LIVE_REPO_FACT`: engine performs no catalog or guidance lookup and does not
  hard-code Copy instruction. Presentation's shared task-block writer receives
  instruction and stimulus only; one-shot no longer substitutes expected-answer
  data, and client no longer substitutes `do this:`.
- `USER_DECISION`: Pure Guidance Resolution V1 remains the narrow guidance
  foundation. Truthful projection authorizes orchestration as its sole current
  product caller; catalog admission, settings, fallback, and translation remain
  unauthorized.

## Capability Planning State

- `USER_DECISION`: capability planning belongs outside the engine; the engine
  executes admitted work and does not invent curriculum policy.
- `LIVE_REPO_FACT`: `curriculum::planner` is curriculum source and pure planning
  policy. It
  defines representation kinds, support levels, learning-focus targets,
  capability records, ordered set steps, and SetPlan records.
- `LIVE_REPO_FACT`: capability records keep semantic frame, response system and
  surface, optional real cue system and surface, operation, cue and response
  representations, support level, and learning-focus target explicit. Guidance
  locale, interface locale, content seed, and future plan seed are not
  capability identity.
- `LIVE_REPO_FACT`: learning focus is distinct from the full required task
  concepts and constraints. The accepted Rust function-return-literal SetPlan
  authors Copy, Fill, Produce, Repair, and Transform in that source order; set
  traversal or completion is curriculum structure, not mastery.
- `LIVE_REPO_FACT`: the Rust function-return-literal surface conceptually
  supports those five operations. `TEMPLATES` contains one static template per
  operation, and two reviewed rows materialize one task per template. Only the
  explicitly admitted IDs enter product play; template support alone does not
  imply admission.
- `LIVE_REPO_FACT`: `curriculum::planner::planning` provides the pure
  `plan_cardinality` and `build_practice_plan` boundary. `CandidateSummary`
  describes supplied task metadata and carries `content_seed` only as inert
  provenance; no catalog adapter or lookup exists.
- `LIVE_REPO_FACT`: `GuidanceAvailability` contains exact locale, pack version,
  and instruction key without instruction text or fallback. Supported answer
  rules and selected guidance availability are explicit eligibility inputs.
- `LIVE_REPO_FACT`: `PlannedRep` contains only step, capability, and supplied
  rep IDs. `PlanProvenance` records set ID/version, plan seed/cardinality,
  selected guidance locale/version, and caller-ordered supported answer rules;
  `PracticePlan` contains only provenance and planned rep IDs.
- `LIVE_REPO_FACT`: `PlanError` explicitly classifies invalid guidance source,
  capability/candidate source, eligibility, cardinality, and seed failures.
  Candidate input must already be strict catalog order and is never sorted.
- `LIVE_REPO_FACT`: plan cardinality uses checked mixed radix, zero steps have
  cardinality one, valid plan seeds are exactly `0..cardinality`, and the first
  SetStep varies fastest. No modulo wrapping, PRNG, hash, time, shuffle, or
  hidden entropy exists.
- `LIVE_REPO_FACT`: plan seed changes only deterministic candidate choice and
  appears only in plan provenance; it does not change supplied rep identity.
  Content seed does not influence planning. Explicit repeated capability steps
  may repeat one supplied rep.
- `LIVE_REPO_FACT`: orchestration supplies admitted catalog summaries to the
  pure planner and constructs deterministic five-task product rounds from the
  selected admitted IDs. The planner does not expand templates, generate tasks,
  resolve guidance, inspect task bytes, or enter engine/client/TUI ownership.
- `USER_DECISION`: no persistence, mastery, adaptive selection, or scheduling
  is authorized; planning remains static and deterministic. Tiny Multi-Operation
  Rust Set Materialization V1 is accepted and integrated as inert source.
  Its accepted successor now admits exactly two reviewed rows as IDs 013-022;
  no persistence, mastery, adaptive selection, scheduling, runtime generation,
  or runtime LLM path exists.

## Lanes

- Retired: [`varied-deterministic-flow-v1`](../lanes/varied-deterministic-flow-v1.md),
  accepted and integrated through candidate
  `a0bdbdc00751b08d49b36956fb82600001348217` after Don's real-TTY playtest;
  worktree removed and branch deleted.
- Retired: [`truthful-task-projection-v1`](../lanes/truthful-task-projection-v1.md),
  accepted and integrated into local canonical master; worktree removed and
  branch deleted after Don's accepted real-TTY playtest.
- Retired: [`resolved-core-task-orchestration-v1`](../lanes/resolved-core-task-orchestration-v1.md),
  accepted and integrated into local canonical master; worktree removed and
  branch deleted.
- Retired: [`tiny-multi-operation-materialization-v1`](../lanes/tiny-multi-operation-materialization-v1.md),
  accepted and integrated into local canonical master; worktree removed and
  branch deleted.
- Retired: [`pure-practiceplan-builder-v1`](../lanes/pure-practiceplan-builder-v1.md),
  accepted and integrated into local canonical master; worktree removed and
  branch deleted.
- Retired: [`static-capability-setplan-v1`](../lanes/static-capability-setplan-v1.md),
  accepted and integrated into local canonical master; worktree removed and
  branch deleted.
- Retired: [`pure-guidance-resolution-v1`](../lanes/pure-guidance-resolution-v1.md),
  accepted and integrated into local canonical master; worktree removed and
  branch deleted.
- Retired: [`static-guidance-schema-v1`](../lanes/static-guidance-schema-v1.md),
  accepted and integrated into local canonical master.
- Active: [`problem-first-curriculum-v1`](../lanes/problem-first-curriculum-v1.md)
  is provisioned for Problem-First Task Hierarchy and Zero-Knowledge Brief V1
  at `/home/don/rust-reps-problem-first-curriculum-v1` on branch
  `codex/problem-first-curriculum-v1`. The worker is clean, lane-doctor reported
  `READY`, implementation remains unstarted, and no candidate exists.
- Paused: new Ukrainian/French curriculum, validator integration, lab cleanup,
  product settings, interface localization, persistence, and scheduling.
- No guidance language setting currently exists.
- The integrated Ukrainian source remains proof-only and requires human
  linguistic/product review before any product admission.
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

## Validator And Formatting Policy

- `USER_DECISION`: Exact is a narrow operation-specific rule that removes at
  most one final LF or CRLF and normalizes nothing else; it is not the universal
  validator for future generated Rust exercises.
- `USER_DECISION`: normal reps 001-012 are literal Copy tasks with
  `ExactTrimFinalNewline` and formatting `Disabled`. Exact Copy is appropriate
  for literal surface reproduction.
- `USER_DECISION`: every future task must choose answer rule and format
  capability explicitly. Neither may be inferred from generated identity,
  source length, stimulus/expected equality, rustfmt output, or operation name
  alone.
- `LIVE_REPO_FACT`: Copy and token-scoped Fill use Exact with formatting
  disabled. Produce, Repair, and Transform use `RustfmtExact` for one canonical
  token-level answer modulo rustfmt formatting and enable the separate F1
  editor helper.
- `USER_DECISION`: `RustfmtExact` means formatting equivalence, not semantic
  equivalence. Broader `RustCheck` or `VisibleTests` semantics require a separate
  permanent validator boundary. Rustfmt-rejected learner input is a normal Wrong
  result; rustfmt infrastructure failure remains a classified product error.
- `LIVE_REPO_FACT`: F1 is formatter-only and never submits, validates, or reads
  expected-answer truth. Exact tasks disable it; explicitly format-enabled
  normal and synthetic lab tasks use ordinary rustfmt.
