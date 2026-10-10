> Historical design audit. Its former instructions do not authorize current changes; follow [development](DEVELOPMENT.md) and the active assignment.

# Codex TUI Reference Audit v1

**Local Codex reference path:** `/home/don/rust-reps-reference/codex`

## 1) Reference source/path inspected
- `/home/don/rust-reps-reference/codex/codex-rs/Cargo.toml`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/Cargo.toml`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/main.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/lib.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/tui.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/tui/event_stream.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/app.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/app/input.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/bottom_pane/mod.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/bottom_pane/footer.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/chatwidget.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/chatwidget/rendering.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/src/tui/test_support.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/all.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/manager_dependency_regression.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/suite/mod.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/suite/status_indicator.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/suite/vt100_history.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/suite/vt100_live_commit.rs`
- `/home/don/rust-reps-reference/codex/codex-rs/tui/tests/suite/resize_reflow.rs`
- `/home/don/rust-reps/src/tui.rs`
- `/home/don/rust-reps/src/presentation.rs`
- `/home/don/rust-reps/src/main.rs`

## 2) Codex TUI architecture observations
- `codex-rs` is a workspace with `codex-rs/tui` as a dedicated terminal UI crate.
- `tui/src/lib.rs` owns boot/shutdown orchestration and wires the `Tui` abstraction to `App::run`.
- `tui/src/tui.rs` manages terminal lifecycle: `init`, raw mode enablement, alt-screen enter/leave, and RAII-style restore helpers.
- `tui/src/tui/event_stream.rs` owns event sourcing and event types (`Key`, `Paste`, `Resize`, `Draw`) with a polling broker.
- `App` in `tui/src/app.rs` owns the main state-driven loop and selects across multiple asynchronous event sources.
- Rendering (`chatwidget`, `bottom_pane`, `footer`, `status_*`) is driven from `App::render`/`handle_tui_event` and never writes to domain state directly.
- Input in `app/input.rs` is classified as app-level command/routing first, then delegated into focused widgets.
- Bottom and footer areas are explicit renderable regions with separate concerns: main transcript/composer, footer/status overlays, and mode/help hints.
- Tests split behavior into small units (state-independent functions and text rendering contracts) plus integration smoke tests for terminal reflow, live resize, and cursor geometry.

## 3) What Rust Reps should conceptually copy
- Keep terminal raw-mode and restore in one module with explicit enter/exit symmetry.
- Create a dedicated `App` runner with a strict `update`/`render` split.
- Keep mutable practice/session logic in existing modules; move TUI-only orchestration and view models into `tui`.
- Introduce a key-event classifier that converts terminal input into high-level intents before mutating session state.
- Separate command/status/help areas in UI state into dedicated render models (even if still text-only initially).
- Use test backends for renderer/state contracts before broad manual terminal checks.
- Preserve plain CLI command behavior by testing from the same domain session methods that the TUI calls.

## 4) What Rust Reps should not copy yet
- Not the full Codex concurrency architecture (`select!` across async channels/processes) for this packet.
- Not full chat/thread/session model.
- Not tmux-based ignored integration suite as required default test surface.
- Not additional crates beyond ratatui/crossterm until packet 2.
- Not Codex styling/rendering complexity; preserve rust-reps minimal UI and deterministic output guarantees.

## 5) Proposed Rust Reps TUI architecture
- Module split target:
  - `src/tui.rs` (or `src/tui/mod.rs`): TUI bootstrap and lifecycle guard.
  - `src/tui/app.rs`: state machine loop (`App::run`, `update`, `paint`, exit conditions).
  - `src/tui/state.rs`: pure session view model + UI-derived transient fields (prompt buffer, selected panel).
  - `src/tui/input.rs`: key event decoding (`enter`, `esc`, `ctrl+c`, `:?`, text input) and command dispatch.
  - `src/tui/render.rs`: pure render functions for header/drill/session/help/footer slices.
  - `src/tui/test_support.rs`: in-memory terminal constructor for deterministic unit tests.
- Ownership map:
  - Session truth: `practice::PracticeSession` owns completion/attempts/skip behavior.
  - Command interpretation: `tui::input` maps keystrokes into practice events (`Quit`, `Status`, `Help`, `List`, `Skip`, answer submit).
  - TUI view model: draw-only formatting derived from immutable copies of session snapshots plus transient UI state.
- Cleanup: lifecycle guard must always restore terminal on normal exit and on errors, mirroring current direct `run_tui` behavior.
- Plain CLI fallback: no behavior path for practice logic changes; keep `cli`/`practice` as source of user-facing truth.

## 6) Approved dependency proposal for next packet
- Add/enable (next packet):
  - `ratatui` features used for richer event/size control and testability.
  - `crossterm` features used by codex-style input streaming (`event-stream`) and paste handling where needed.
- Keep for now (do not add this packet):
  - no async runtime changes beyond current structure.
  - no extra widget/layout frameworks.
  - no snapshot/render-to-image tooling.

## 7) Test/proof plan
- Keep all existing plain CLI tests as regression oracle; no plain output contract changes.
- Add unit tests around:
  - `input` intent classification.
  - `render` view-model generation for status/help/footer and completed-session states.
  - existing helper functions currently in `tui.rs` that can be pure-isolated.
- Add TUI-level deterministic tests similar to Codex:
  - render into an off-screen test terminal and assert expected strings/cells.
  - history insertion/wrap/emoji/basic cursor assertions at the rendering boundary.
- Manual proof (optional packet-2 pre-release):
  - capture a short `rust-reps tui` session and verify draw updates after commands, including status/help/help-toggle and completion path.
- `proof.sh` remains the official end-to-end baseline and should include both CLI mode and tui smoke checks in a later packet.

## 8) Risks and rollback condition
- High risk: terminal lifecycle failures can leave user terminal in raw/alternate-screen state on panic or early return.
- Medium risk: event handling that mutates session state directly from input can reintroduce one-shot/CLI drift.
- Medium risk: visual-only refactors can pass unit tests but regress plain output if session command semantics are duplicated.
- Rollback condition:
  - if either `cargo test` or `scripts/proof.sh` shows any behavior regression, revert the TUI architecture change immediately to current CLI-only flow and restore existing direct stdin loop in `tui.rs`.
