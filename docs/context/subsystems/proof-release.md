# Scripts, Proof, Installation, And Release Evidence

Context role: Subsystem authority and navigation.
Scope: `scripts`, `tests`, standalone fleet tools, operations data, and external evidence artifacts.
Canonical for: Verification layers, proof limits, and release-evidence ownership.
Not canonical for: Product behavior beyond observed commands or release authorization.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and [`../../operations/RELEASE_AND_EVIDENCE.md`](../../operations/RELEASE_AND_EVIDENCE.md).
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: [`../../evidence/README.md`](../../evidence/README.md).
Update triggers: Gate, proof script, fleet tool, artifact schema, or release authority changes.

## Mission And Authority

Product proof owns executable regression evidence: formatting, unit/integration
tests, exact CLI transcripts, Git diff checks, and temporary installed-command
smokes. Fleet tooling independently verifies context, lanes, packets, queues,
and evidence structure. Operations data owns canonical machine-readable context
membership and integration queue state.

Allowed authority is to report what commands prove at an exact SHA. Forbidden
authority is changing product behavior to satisfy proof, treating green tests as
unrelated runtime proof, deploying, pushing, merging, or turning diagnostics
into automatic worker mutation.

Fleet tools are standalone std-only Rust compiled outside product Cargo. They
may read Git/config and write only explicit generated outputs. They never reset,
clean, stash, force-push, or mutate canonical master.

## Proof, Rollback, And Limits

Existing automated gates are `cargo fmt --check`, `cargo test`,
`./scripts/proof.sh`, and diff checks. Fleet gates use `rustfmt`, `rustc --test`,
fixture tests, context verification, and deterministic render checks.

These do not provide new manual TUI feel evidence, remote currentness when no
remote exists, deployment proof, learning efficacy, or semantic contradiction
resolution. Rollback is removal/reversion of standalone infrastructure; product
proof remains unchanged. Artifacts are checksummed outside the repo. Resource
use is short-lived local processes and text files; no daemon exists.

## Gate And Unknowns

Central proof-script wiring is deferred until Master accepts ownership and
overlap. Release/push/deploy remain unauthorized. Update for any gate, generated
owner, artifact schema, or authority change.
