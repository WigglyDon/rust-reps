# Project Context Index

This index is the entry point for durable Rust Reps project memory. Product
behavior remains owned by live source, tests, and terminal proof—not by this
documentation.

## Agent Discovery Order

1. Read repository-root [`AGENTS.md`](../AGENTS.md).
2. Read every more-specific `AGENTS.md` from project root to the working directory.
3. Read this `docs/INDEX.md`.
4. Read [`docs/context/CURRENT_STATE.md`](context/CURRENT_STATE.md).
5. Read the canonical ownership, migration, retirement, or lane ledger relevant to the task.
6. Read the subsystem page linked by the nearest scoped `AGENTS.md`.
7. Read the active lane page when operating in a lane.
8. Read relevant contracts, decisions, fixtures, reports, artifacts, and proof.
9. If documents conflict, stop and report the conflict rather than silently choosing.
10. Stronger current source, repository, test, or runtime evidence may correct stale shared memory.

## Canonical Context

- Context-system law: [`context/README.md`](context/README.md)
- Current state: [`context/CURRENT_STATE.md`](context/CURRENT_STATE.md)
- Context matrix: [`context/CONTEXT_MATRIX.md`](context/CONTEXT_MATRIX.md)
- Project history: [`context/PROJECT_HISTORY.md`](context/PROJECT_HISTORY.md)
- Decision log: [`context/DECISION_LOG.md`](context/DECISION_LOG.md)
- Context manifest: [`../ops/fleet/context-manifest.tsv`](../ops/fleet/context-manifest.tsv)
- Reported Context-SHA: [`../ops/fleet/context-sha256.txt`](../ops/fleet/context-sha256.txt)

## Subsystems

- [`context/subsystems/curriculum-catalog.md`](context/subsystems/curriculum-catalog.md)
- [`context/subsystems/procgen.md`](context/subsystems/procgen.md)
- [`context/subsystems/engine-session.md`](context/subsystems/engine-session.md)
- [`context/subsystems/answer-validation-tools.md`](context/subsystems/answer-validation-tools.md)
- [`context/subsystems/cli-practice-presentation.md`](context/subsystems/cli-practice-presentation.md)
- [`context/subsystems/client-tui.md`](context/subsystems/client-tui.md)
- [`context/subsystems/lab.md`](context/subsystems/lab.md)
- [`context/subsystems/proof-release.md`](context/subsystems/proof-release.md)

## Coordination And Process

- Lane registry: [`lanes/lane-registry.md`](lanes/lane-registry.md)
- Active static capability/SetPlan lane: [`lanes/static-capability-setplan-v1.md`](lanes/static-capability-setplan-v1.md)
- Retired pure guidance resolution lane record: [`lanes/pure-guidance-resolution-v1.md`](lanes/pure-guidance-resolution-v1.md)
- Retired static guidance schema lane record: [`lanes/static-guidance-schema-v1.md`](lanes/static-guidance-schema-v1.md)
- Retired infrastructure lane record: [`lanes/master-infrastructure.md`](lanes/master-infrastructure.md)
- Accepted architecture: Language-Agnostic Capability Selector Architecture V1.
- Planned after the active inert schema boundary: Pure Deterministic PracticePlan
  Builder V1, Tiny Multi-Operation Rust Set Materialization V1, and Product
  Integration Decision V1; none is active.
- Integration queue source: [`../ops/fleet/integration-queue.tsv`](../ops/fleet/integration-queue.tsv)
- Generated integration view: [`lanes/integration-queue.md`](lanes/integration-queue.md)
- Packet protocol: [`process/PACKET_PROTOCOL.md`](process/PACKET_PROTOCOL.md)
- Handoff and respawn: [`process/HANDOFF_AND_RESPAWN.md`](process/HANDOFF_AND_RESPAWN.md)

## Evidence And Operations

- Evidence guidance: [`evidence/README.md`](evidence/README.md)
- Runtime schemas: [`evidence/RUNTIME_EVIDENCE_SCHEMAS.md`](evidence/RUNTIME_EVIDENCE_SCHEMAS.md)
- Release and evidence operations: [`operations/RELEASE_AND_EVIDENCE.md`](operations/RELEASE_AND_EVIDENCE.md)

## Historical Reference Documents

These pages are donor/reference evidence, not current architecture owners:

- [`codex_tui_reference_audit.md`](codex_tui_reference_audit.md)
- [`editor_backend_audit.md`](editor_backend_audit.md)
