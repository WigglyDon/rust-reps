# Editor Backend Audit v1

Date: 2026-07-06

This is an audit-only packet. It recommends an editor/highlighting path for Rust
Reps without changing dependencies or product behavior.

## Current Baseline

Rust Reps currently has:

- `ratatui = 0.29.0` and `crossterm = 0.29.0` only.
- A client-owned one-line draft input string, cursor blink state, status
  projection, reward projection, and semantic `CodeToken` values.
- A TUI-owned renderer that maps client semantics into `ratatui` lines, spans,
  styles, layout, and theme colors.
- A small internal one-line tokenizer used only for display.
- Plain mode as the exact output oracle.

This baseline is clean architecturally, but it is not yet a serious editor:
there is no multiline buffer model, vertical cursor movement, selection,
viewport/scroll model, indentation behavior, or parse-aware highlighting.

## Sources Checked

- `tui-textarea` 0.7.0 docs: <https://docs.rs/tui-textarea/latest/tui_textarea/>
- `tui-textarea::TextArea` docs: <https://docs.rs/tui-textarea/latest/tui_textarea/struct.TextArea.html>
- `tui-syntax` 0.4.0 docs: <https://docs.rs/tui-syntax/latest/tui_syntax/>
- `tree-sitter` docs: <https://docs.rs/tree-sitter/latest/tree_sitter/>
- `tree-sitter-rust` 0.24.2 docs: <https://docs.rs/tree-sitter-rust/latest/tree_sitter_rust/>
- `tree-sitter-highlight` docs: <https://docs.rs/tree-sitter-highlight/latest/tree_sitter_highlight/>
- `ratatui-code-editor` docs: <https://docs.rs/ratatui-code-editor/latest/ratatui_code_editor/>
- `ratatui-code-editor` repository: <https://github.com/vipmax/ratatui-code-editor>
- `syntect` docs: <https://docs.rs/syntect/latest/syntect/>
- `two-face` docs: <https://docs.rs/two-face/latest/two_face/>

## Option Matrix

| Option | Problem solved | Dependencies added | Ratatui fit | Multiline editing | Rust highlighting | Client projection cleanliness | App logic ownership risk | Offline testability | Rollback/removal condition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1. Keep current custom editor + internal highlighter | Keeps current one-line reps tiny and portable | None | Native current code | No | Partial one-line only | Best; client owns semantic tokens and TUI owns styles | Low | Excellent; pure unit tests and render snapshots | Keep as fallback until multiline/editor behavior is required |
| 2. Add `tui-textarea` | Provides mature textarea operations, cursor movement, undo/redo, line numbers, scrolling, selection | `tui-textarea`, `unicode-width`; avoid `search` to avoid `regex` | Good; current 0.7.0 targets `ratatui ^0.29.0` | Yes | No parser/highlighter by itself | Mixed; `TextArea` owns buffer/cursor/edit state unless wrapped carefully | Medium; easy for widget state to become the app state | Good if tested through a wrapper and `TextArea::lines()` | Remove if it forces TUI-local buffer truth or conflicts with client-owned editor model |
| 3. Add `tui-syntax` / Tree-sitter highlighting | Fast path to styled `ratatui::Line` / `Span` output | `tui-syntax`, `tree-sitter`, `tree-sitter-highlight`, built-in language deps, `serde`, `toml` | Direct | No | Extensible, but docs show built-in SQL/HTML/JSON, not Rust | Poor for architecture if `ratatui` styled text crosses into `client` | Medium; it owns styled projection and theme format | Moderate; test output spans, but client semantics get bypassed | Remove if it prevents semantic token projection or forces theme ownership outside `tui/theme.rs` |
| 4. Add `tree-sitter-rust` directly with a small adapter | Accurate Rust parsing/highlighting while preserving Rust Reps projection | Likely `tree-sitter`, `tree-sitter-rust`, and possibly `tree-sitter-highlight` | Indirect; TUI maps semantic tokens to ratatui styles | No editor by itself | Yes; `tree-sitter-rust` exposes `LANGUAGE` and `HIGHLIGHTS_QUERY` | Best highlighter path; adapter can emit client `CodeToken`-like semantic spans | Low if adapter is display-only and not correctness logic | Excellent; parse/highlight spans can be pure deterministic tests | Disable/remove if build complexity, version skew, or highlighting failures outweigh quality gain |
| 5. Add `ratatui-code-editor` | Full editor widget with buffer, selection, undo/redo, Tree-sitter, themes, mouse, clipboard | Broad stack: `ratatui 0.30`, `crossterm`, `ropey`, `arboard`, `rust-embed`, many `tree-sitter-*` grammars, etc. | Not currently aligned; docs show `ratatui =0.30` while Rust Reps is on 0.29 | Yes | Yes, including Rust | Weak; `Editor` handles rendering and input, and likely becomes TUI-local app state | High | Moderate; more integration and snapshot surface | Do not adopt unless the crate stabilizes, aligns with Rust Reps ratatui version, supports a clean model/projection split, and broad deps are acceptable |
| 6. Use `syntect` / `two-face` instead of Tree-sitter | Mature syntax highlighting from TextMate-style grammars and themes | `syntect`; possibly `two-face`; regex engine and embedded syntax/theme assets | Indirect; custom conversion to ratatui styles | No editor by itself | Yes | Good if adapted into semantic spans, but less Rust-structure-aware | Low/medium; highlighter remains display-only | Good; string-to-ranges tests are deterministic | Remove if binary size, regex/onig build/runtime profile, or non-incremental behavior is not worth it |

## Recommendation

Recommended stack for the next implementation packet:

1. Keep a first-party, client-owned editor model.
2. Add Tree-sitter Rust highlighting directly through a small adapter.
3. Keep Ratatui as the terminal renderer only.
4. Do not adopt `ratatui-code-editor` yet.
5. Do not adopt `tui-syntax` as the main path because it returns ratatui styled
   text directly and makes it too easy to bypass client semantic projection.
6. Do not adopt `syntect` / `two-face` for Rust Reps unless Tree-sitter proves
   too expensive; they are heavier and less aligned with modern parse-aware
   editor behavior.

The practical dependency recommendation for a future implementation is:

- First choice: `tree-sitter`, `tree-sitter-rust`, and `tree-sitter-highlight`
  behind a small Rust Reps adapter.
- Optional later: `tui-textarea` only if it can be wrapped so that `client`
  still owns buffer, cursor, selection, and viewport truth. Use backend-agnostic
  input conversion rather than depending on its crossterm feature if crossterm
  versions do not align.
- Avoid for now: `ratatui-code-editor`, `tui-syntax`, `syntect`, `two-face`.

## Architecture Ownership

Editor buffer truth should live in `client`.

- Use a portable `client::editor` model such as `EditorBuffer { lines }`,
  `EditorCursor { row, column }`, `EditorSelection`, `EditorViewport`, and
  focused edit actions.
- Store submitted text in the exact shape needed by `answer` and `engine`.
- Keep exact-answer semantics unchanged: editor text is just input text.

Cursor, selection, viewport, and scroll state should live in `client`.

- These are user-visible state for the solution editor, not terminal details.
- The TUI can request movement and scrolling through `ClientAction` values, but
  it should not own the resulting truth.

Tree-sitter parse/highlight state should live in `client` or a client-adjacent
pure adapter module, not in `engine`.

- The adapter should accept editor text and return semantic display spans.
- It must be display-only and failure-tolerant: parse errors fall back to plain
  or existing lightweight tokens.
- It must never decide whether an answer is correct.

What stays in `client`:

- Editor buffer, cursor, selection, viewport, display-only highlight spans,
  problem/workbox frame data, reward state, and terminal-free frame structs.

What stays in `tui`:

- Key event translation, terminal lifecycle, frame scheduling, render layout,
  Ratatui widgets, color/theme mapping, and optional terminal capability/color
  fallback handling.

What stays in `engine`:

- Session truth, exercise packets, pass/skip/attempt counts, and state
  transitions.

What must remain proofable in plain mode:

- `rust-reps plain`, piped `rust-reps`, and one-shot `rust-reps 001` outputs.
- Exact answer normalization and comparison.
- Wrong answers as product results, not crashes.
- No Tree-sitter/editor dependency should be required to validate plain output
  semantics at runtime.

## Fresh Terminal / Fresh Linux Install

Installed binary runtime:

- A compiled binary should not need network access during normal play.
- If Tree-sitter Rust is added through crates, the grammar and highlight query
  should be compiled/embedded into the binary or crate artifacts.
- Runtime failures in highlighting should degrade to plain semantic spans, not
  prevent typing or answer submission.

Cargo source install:

- `cargo install --path . --force` in this repo currently needs only Rust,
  Cargo, and the existing dependencies.
- A future Tree-sitter Rust path will add build-time native compilation through
  grammar crates; `tree-sitter-rust` currently has a `cc` build dependency.
- Fresh source installs may therefore need a C compiler/toolchain during build.
  They should not need network after dependencies are fetched.

Generic terminal behavior:

- Ratatui/crossterm should remain the terminal layer.
- Truecolor/RGB theme roles can stay, but the TUI should be able to degrade to
  ANSI or monochrome if the terminal cannot represent the palette.
- `NO_COLOR` is relevant as a future TUI theme behavior: if set, disable
  decorative colors while preserving layout, symbols, text, and answer flow.
  This belongs in `tui/theme.rs`, not in `client` or `engine`.
- Plain mode remains the guaranteed fallback for non-interactive or color-hostile
  environments.

## Implementation Notes For A Future Packet

- Add a `client::editor` model before replacing the TUI workbox.
- Extend `ClientAction` with portable editor intents such as move left/right/up,
  move line start/end, insert newline, delete next char, paste text, and maybe
  indent/dedent.
- Keep `tui/input.rs` as crossterm-to-client intent translation.
- Keep `tui/render.rs` as client-frame-to-ratatui rendering.
- Add Tree-sitter adapter tests that concatenate highlighted span text back to
  the original input exactly.
- Add a highlight failure test that returns plain spans and does not change the
  answer submitted to `engine`.
- Keep plain CLI transcripts unchanged and run the existing proof script as the
  regression oracle.

## Decision

Choose a first-party client editor model plus direct Tree-sitter Rust
highlighting. This is more work than importing a full editor widget, but it
preserves Rust Reps' architecture: curriculum/catalog feeds engine truth, engine
feeds client projection, and the TUI remains only one terminal projection.
