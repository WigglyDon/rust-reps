# AGENTS: Terminal UI Adapter

## Scope And Inheritance

Applies under `src/tui/`. Inherits [`../../AGENTS.md`](../../AGENTS.md).

## Owner And Authority

TUI owns terminal lifecycle, event mapping, frame scheduling, Ratatui rendering,
and theme. It may request client/engine actions. It may not own session truth,
curriculum, Exact semantics, or leak terminal types into client public state.

## Canonical Links

- Root law: [`../../AGENTS.md`](../../AGENTS.md)
- Index: [`../../docs/INDEX.md`](../../docs/INDEX.md)
- Current state: [`../../docs/context/CURRENT_STATE.md`](../../docs/context/CURRENT_STATE.md)
- Subsystem: [`../../docs/context/subsystems/client-tui.md`](../../docs/context/subsystems/client-tui.md)
- Lane registry: [`../../docs/lanes/lane-registry.md`](../../docs/lanes/lane-registry.md)

## Validation And Stop Conditions

Run input/render/lifecycle tests, full plain proof, and real-TTY playtest for
feel claims. Stop on terminal restoration risk, hidden key aliases, session
truth duplication, or plain-output drift. Update on lifecycle, controls, layout,
or ownership changes.
