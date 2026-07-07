# AGENTS: Rust Reps

## Project Identity
- Product: Rust Reps
- Repo/package/binary/command: `rust-reps`
- Terminal-first Rust practice game with exact one-line typing reps.
- Plain terminal `stdout`/`stderr` is the product surface.

## Core Law
- Terminal output is the source of truth.
- Source behavior, tests, proof script, and CLI transcripts outrank summaries.
- Wrong answers are product results (e.g., `TRY AGAIN`), not crashes.
- Tool/parsing errors are for CLI errors only: print to `stderr` and exit nonzero.
- Plain CLI output remains the baseline proof oracle; TUI uses the same session truth.

## Current Behavior Contract
- `rust-reps` starts fullscreen TUI in an interactive terminal.
- Piped `rust-reps` starts plain practice mode.
- `rust-reps plain` starts plain practice mode.
- `rust-reps 001` runs one-shot drill 001.
- `rust-reps lab` starts fullscreen client stress lab in an interactive terminal.
- Piped `rust-reps lab` prints a CLI error and exits nonzero.
- Practice commands:
  - `:help`, `:status`, `:list`, `:skip`, `:q`, `:quit`.
  - `EOF` exits practice.
  - Completion is distinct from early quit.
- Practice result flow:
  - Exact match prints `PASS` and advances.
  - Wrong answer prints `TRY AGAIN` and repeats the same drill.
  - `:skip` advances without incrementing attempts.
  - `:status`, `:list`, `:help` do not increment attempts.
- Unknown colon-prefixed input in practice is a command error:
  - Print `UNKNOWN COMMAND` / `Type :help for commands.`
  - Do not increment attempts.
- One-shot mode does not use in-session commands; any input is treated as an answer.

## Exact Answer Contract
- Trim exactly one trailing line ending only:
  - `\n` or `\r\n`.
- Do not normalize other input:
  - leading spaces, trailing spaces, internal spaces, tabs, casing, punctuation,
  - braces, quotes, Unicode, or multiline shape.
- Blank line is a wrong answer and prints `TRY AGAIN` + expected answer.
- Empty line ending does not bypass exactness.

## Module Ownership
- `curriculum`: curriculum data schema (`DrillSpec`, enums) and static drill data.
- `catalog`: public catalog API, rendering, and curriculum validation tests.
- `main`: process orchestration and interactive default decision only.
- `answer`: answer normalization/comparison helpers.
- `cli`: launch command selection, usage/help/list/version, CLI/tool errors.
- `engine`: pure game/session simulation, state transitions, and exercise-shaped packets.
- `lab`: client/engine stress fixtures for visible lab mode; not normal curriculum.
- `practice`: plain terminal adapter, stdin/stdout, and in-session command handling.
- `client`: platform-neutral editor model, problem viewport, visual projection from engine packets, semantic code tokens, client frame structs, and visual-only UI state.
- `tui`: terminal adapter/projection, theme/color mapping, fullscreen rendering, key input loop, and terminal lifecycle only.
- `tests` / `scripts/proof.sh`: executable behavior evidence.
- `README`: describe proven behavior only.

## Forbidden Scope (without explicit approval)
- No additional crates beyond this packet's explicit approval.
- No alternative TUI frameworks or GUI.
- No persistence.
- No random mode.
- No hidden tests.
- No drill additions.
- No compilation validation feature.
- No broad refactors; proof behavior must stay exact.

## Approved Dependencies
- `ratatui` (for fullscreen TUI rendering/input loop)
- `crossterm` (terminal control for fullscreen TUI)

## Verification Gates
- `cargo fmt --check`
- `cargo test`
- `./scripts/proof.sh`
- `git diff --check`
- `git diff --cached --check` (if staged)
- If install behavior is changed: `cargo install --path . --force`,
  `command -v rust-reps`, `rust-reps --version`,
  installed-command practice/one-shot smoke checks.

## Workflow Constraints
- Inspect repo state before edits.
- Stop and report if unrelated files are modified.
- Commit only after required verification passes.
- Include commands run, results, and final status in closure.
- Do not push, publish, tag, release, create GitHub workflows, or edit CI unless explicitly requested.
