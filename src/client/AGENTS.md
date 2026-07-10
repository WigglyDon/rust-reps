# AGENTS: Portable Client

## Scope And Inheritance

Applies under `src/client/`. Inherits [`../../AGENTS.md`](../../AGENTS.md).

## Owner And Authority

Client owns terminal-free editor and visual projection state. It may consume
engine packets/snapshots. It may not import Ratatui/Crossterm public model types,
own curriculum/session/answer truth, or invoke product tools.

## Canonical Links

- Root law: [`../../AGENTS.md`](../../AGENTS.md)
- Index: [`../../docs/INDEX.md`](../../docs/INDEX.md)
- Current state: [`../../docs/context/CURRENT_STATE.md`](../../docs/context/CURRENT_STATE.md)
- Subsystem: [`../../docs/context/subsystems/client-tui.md`](../../docs/context/subsystems/client-tui.md)
- Lane registry: [`../../docs/lanes/lane-registry.md`](../../docs/lanes/lane-registry.md)

## Validation And Stop Conditions

Run portable model/projection tests and full proof; obtain real-TTY evidence for
feel claims. Stop on terminal-type leakage, answer mutation, or duplicated
session truth. The boundary is sealed unless playtesting exposes a real defect.
Update on model ownership or accepted interaction changes.
