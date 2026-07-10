# AGENTS: Documentation Context

## Scope And Inheritance

Applies under `docs/`. Inherits [`../AGENTS.md`](../AGENTS.md) without weakening it.

## Owner And Authority

Documentation/context owns durable navigation, law, current state, history,
decisions, lane memory, process, and evidence schemas. It may correct stale
shared memory using labeled stronger evidence. It may not redefine product
behavior, terminal output, or source ownership by prose alone.

Allowed dependencies are links to root law, live source owners, Git evidence,
tests, artifacts, and the canonical context graph. Do not copy mutable truth
when a canonical owner exists. Do not import generated product data or private
chat transcripts.

## Canonical Links

- Root law: [`../AGENTS.md`](../AGENTS.md)
- Index: [`INDEX.md`](INDEX.md)
- Current state: [`context/CURRENT_STATE.md`](context/CURRENT_STATE.md)
- Context system: [`context/README.md`](context/README.md)
- Lane registry: [`lanes/lane-registry.md`](lanes/lane-registry.md)
- Documentation/proof subsystem: [`context/subsystems/proof-release.md`](context/subsystems/proof-release.md)

## Validation And Stop Conditions

Run context/link/header/manifest verification and `git diff --check`. Stop on
conflicting canonical owners, unlabeled stale claims, broken local links, or any
attempt to grant product/release authority through documentation. Update this
file when documentation ownership or validation changes.
