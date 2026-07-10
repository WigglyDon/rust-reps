# AGENTS: Standalone Fleet Tools

## Scope And Inheritance

Applies under `tools/`. Inherits [`../AGENTS.md`](../AGENTS.md).

## Owner And Authority

Fleet tools own read-only diagnosis and explicit deterministic generation of
context, queue views, respawn bundles, and evidence manifests. They are std-only
standalone tooling, not a product crate or runtime API.

They may inspect Git/config and write only user-requested output/generated owner
files. They must not reset, clean, stash, push, force-push, mutate canonical
master, execute workers, print secrets, add product dependencies, or infer
private chat state.

## Canonical Links

- Root law: [`../AGENTS.md`](../AGENTS.md)
- Index: [`../docs/INDEX.md`](../docs/INDEX.md)
- Current state: [`../docs/context/CURRENT_STATE.md`](../docs/context/CURRENT_STATE.md)
- Subsystem: [`../docs/context/subsystems/proof-release.md`](../docs/context/subsystems/proof-release.md)
- Infrastructure lane: [`../docs/lanes/master-infrastructure.md`](../docs/lanes/master-infrastructure.md)

## Validation And Stop Conditions

Run standalone rustfmt, rustc tests, deterministic fixture tests, and forbidden
destructive-command audits. Stop on product linkage, secret exposure, implicit
mutation, ambiguous output, or a requested operation outside explicit authority.
Update on tool command/schema/authority changes.
