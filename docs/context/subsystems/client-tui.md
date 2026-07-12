# Client And TUI

Context role: Subsystem authority and navigation.
Scope: `src/client` and `src/tui`.
Canonical for: Portable editor/presentation state and terminal-only projection boundaries.
Not canonical for: Curriculum, answer truth, or session transitions.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and [`../CONTEXT_MATRIX.md`](../CONTEXT_MATRIX.md).
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: Client/TUI unit tests, render snapshots, and real-terminal playtests when captured.
Update triggers: Model ownership, key/mouse mapping, layout, lifecycle, or accepted feel changes.

## Mission And Authority

Client owns terminal-independent editor buffer, cursor, viewport, problem
projection, semantic code tokens, reward/failure/skip/format visual state, and
frame structs. TUI owns Crossterm events/lifecycle, Ratatui layout/render/theme,
frame scheduling, and mapping inputs into client/engine actions.

Client may read engine packets and session snapshots. TUI may read client
frames and request engine/client actions. Terminal-library types must not enter
the client public model; neither layer may own curriculum, attempts, validation,
or exact answer semantics.

The accepted client boundary is sealed. Smart pairs/newline, independent
scrolling, mouse cursor placement, F1/F5/F7/Esc mapping, and minimal counter
shell are live behavior, but plain mode remains the proof baseline.

Client task rows carry exact instruction/stimulus projection plus hard-line
versus soft-wrap identity without expected-answer truth. TUI reserves an
out-of-source `↪ ` gutter for soft stimulus continuations, equalizes effective
editor/stimulus source widths, and shares their source baseline.

Format capability is explicit packet data projected as availability only.
Current normal Exact Copy tasks dim F1 and pressing it leaves the editor
unchanged with `format off for exact task`; explicitly Rustfmt-enabled lab tasks
retain ordinary formatter behavior. F1 never submits, validates, or reads
expected-answer truth.

## Proof And Limits

Accepted automated proof includes portable editor actions, projections, render
cell/snapshot tests, input classification, lifecycle helpers, and regression
proof. A real TTY/manual playtest is required for fullscreen geometry, mouse
behavior, terminal capabilities, and final feel; redirected output is not a
substitute. Don owns final feel judgment.

Rollback is restoration of the last sealed client/TUI commit with session and
plain behavior unchanged. Observable automated state is client frames and
render buffers; manual evidence records terminal environment and actions.
Frame pacing is bounded by the scheduler; no performance telemetry exists.

## Gate And Unknowns

Truthful Task Projection Contract V1 is accepted and integrated after automated
render proof and Don's installed real-TTY playtest. Generic `do this:` is gone;
resolved instruction precedes exact stimulus, and expected answers remain out
of task render models. Controls, editor behavior, counters, independent
scrolling, and lifecycle remain sealed. No implementation lane is active.
Update for ownership, layout, controls, lifecycle, or proven feel changes.
