# AGENTS: Engine And Session

## Scope And Inheritance

Applies under `src/engine/`. Inherits [`../../AGENTS.md`](../../AGENTS.md).

## Owner And Authority

Engine owns pure session truth, packets, attempts, pass/skip/completion, and
snapshots. It may read admitted catalog data, validator dispatch, and explicit
lab packets. It must not generate curriculum, render terminal output, own editor
state, or import client/TUI types.

## Canonical Links

- Root law: [`../../AGENTS.md`](../../AGENTS.md)
- Index: [`../../docs/INDEX.md`](../../docs/INDEX.md)
- Current state: [`../../docs/context/CURRENT_STATE.md`](../../docs/context/CURRENT_STATE.md)
- Subsystem: [`../../docs/context/subsystems/engine-session.md`](../../docs/context/subsystems/engine-session.md)
- Lane registry: [`../../docs/lanes/lane-registry.md`](../../docs/lanes/lane-registry.md)

## Validation And Stop Conditions

Run engine, adapter, and full transcript proof. Stop if skip becomes an attempt,
wrong answers become errors, Exact changes, generator logic enters engine, or
terminal types cross the boundary. Update on packet or transition changes.
