# AGENTS: Scripts And Proof

## Scope And Inheritance

Applies under `scripts/`. Inherits [`../AGENTS.md`](../AGENTS.md).

## Owner And Authority

Scripts own explicit verification and thin standalone-tool launchers. Product
proof may assert existing stdout/stderr/exit contracts. Fleet scripts may
compile/run local diagnostics outside product Cargo. Scripts may not silently
change product behavior, mutate canonical master, push, deploy, reset, clean,
stash, or expose secrets.

## Canonical Links

- Root law: [`../AGENTS.md`](../AGENTS.md)
- Index: [`../docs/INDEX.md`](../docs/INDEX.md)
- Current state: [`../docs/context/CURRENT_STATE.md`](../docs/context/CURRENT_STATE.md)
- Subsystem: [`../docs/context/subsystems/proof-release.md`](../docs/context/subsystems/proof-release.md)
- Lane registry: [`../docs/lanes/lane-registry.md`](../docs/lanes/lane-registry.md)

## Validation And Stop Conditions

Run shell syntax where relevant, exact proof, fleet tool tests, and diff checks.
Stop on transcript drift, destructive commands, hidden network mutation, or
secret output. Update on gate or launcher ownership changes.
