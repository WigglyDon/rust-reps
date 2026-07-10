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

The next authority gate is a separately reviewed admission design after inert
materialization proof. `LIVE_REPO_FACT`: non-Copy admission is blocked by prompt
propagation limitations recorded in current state. Update this page when schema,
ordering, admission, or visible catalog behavior changes.
