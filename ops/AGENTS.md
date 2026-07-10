# AGENTS: Fleet Operations Data

## Scope And Inheritance

Applies under `ops/`. Inherits [`../AGENTS.md`](../AGENTS.md).

## Owner And Authority

Operations data owns canonical machine-readable context membership and
integration queue source. Generated views and reported hashes derive from those
owners. This scope does not grant integration, release, push, deploy, deletion,
or product authority.

## Canonical Links

- Root law: [`../AGENTS.md`](../AGENTS.md)
- Index: [`../docs/INDEX.md`](../docs/INDEX.md)
- Current state: [`../docs/context/CURRENT_STATE.md`](../docs/context/CURRENT_STATE.md)
- Subsystem: [`../docs/context/subsystems/proof-release.md`](../docs/context/subsystems/proof-release.md)
- Infrastructure lane: [`../docs/lanes/master-infrastructure.md`](../docs/lanes/master-infrastructure.md)

## Validation And Stop Conditions

Regenerate and verify manifests/views deterministically; run diff checks. Stop
on stale hashes, malformed TSV, duplicate authority, secret material, or any
attempt to encode automatic integration. Update on schema, membership, or queue
state changes.
