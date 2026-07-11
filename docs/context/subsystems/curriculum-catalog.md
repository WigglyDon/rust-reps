# Curriculum And Catalog

Context role: Subsystem authority and navigation.
Scope: `src/curriculum`, normal static reps, and `src/catalog.rs`.
Canonical for: Curriculum/catalog ownership, ordering, IDs, lookup, and admission boundary.
Not canonical for: Session truth, generation internals, or terminal presentation.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and [`../CONTEXT_MATRIX.md`](../CONTEXT_MATRIX.md).
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: Catalog tests and [`../../evidence/README.md`](../../evidence/README.md).
Update triggers: Rep schema, shard registration, catalog invariants, or admission changes.

## Mission And Authority

Curriculum supplies immutable rep data; catalog owns the playable view, stable
short IDs, canonical metadata, ordering, lookup, rendering, and admission
invariants. Source roots are `src/curriculum/mod.rs`, shard modules such as
`syntax_basics.rs`, and `src/catalog.rs`.

Allowed authority: normal reps, metadata, `DRILL_SHARDS`, `DRILLS`, lookup, and
catalog tests. Forbidden authority: attempts, pass/skip truth, terminal I/O,
runtime generation, validation weakening, and lab admission.

Allowed direction is catalog reading curriculum data and engine reading the
ordinary catalog. Curriculum must not depend on engine, client, TUI, or
practice. Generated-shaped data remains unplayable until a future explicit
catalog admission boundary produces ordinary `RepSpec` data.

## Adjacent Inert Planner Source And Pure Planning

`src/curriculum/planner.rs` owns inert representation kinds, support levels,
learning-focus targets, capability records, and explicit SetPlan source. The
accepted Rust function-return-literal set authors Copy, Fill, Produce, Repair,
and Transform in source order. Learning focus is evidence intent and does not
replace all required task concepts or constraints; set traversal is structure,
not mastery.

`src/curriculum/planner/planning.rs` owns the inert pure planning boundary:
`CandidateSummary`, `GuidanceAvailability`, `PlannedRep`, `PlanProvenance`,
`PracticePlan`, `PlanError`, `plan_cardinality`, and `build_practice_plan`.
Candidate summaries are supplied read-only metadata, not catalog reads;
content seed is provenance only. Guidance availability carries exact
locale/version/instruction-key facts without text, resolution, or fallback.

Candidate IDs must be unique and source order must already be strict catalog
order. Eligibility uses exact capability, caller-supported answer rule, and
selected guidance availability. Checked mixed radix gives zero steps
cardinality one, accepts only seeds in `0..cardinality`, and varies the first
SetStep fastest. Output contains planning provenance and supplied rep IDs only.

Planner source does not inspect stimulus or expected bytes, expand procgen
templates, generate tasks, read catalog, resolve guidance, execute a plan,
persist state, adapt from attempts, use XP/mastery/scheduling, or enter catalog,
engine, presentation, client, TUI, CLI, practice, or lab product flow. Future
catalog integration must construct truthful candidate summaries from admitted
reps without moving catalog authority into planner.

## Consistency And Proof

Ordering is explicit source order. Short IDs are unique three-digit strings;
canonical IDs and required metadata are unique/present; prerequisites resolve;
normal reps use explicit answer rules and transitions. Lab data never enters
the catalog.

Accepted proof: catalog unit invariants, CLI `--list`/lookup tests, and the full
proof transcript. Those gates do not prove learning quality or future admission
safety. Runtime integration is current for hand-authored normal reps only.

Rollback is commit-level removal of an unaccepted data/schema change while
preserving IDs and transcripts. Observability is catalog listing and tests;
there is no persistence or telemetry. Resource cost is static data only.

## Gate And Unknowns

The next planned boundary is Tiny Multi-Operation Rust Set Materialization V1;
admission remains later, after inert materialization proof. `LIVE_REPO_FACT`:
non-Copy admission is blocked by prompt propagation limitations recorded in
current state. Update this page when schema, planning, ordering, admission, or
visible catalog behavior changes.
