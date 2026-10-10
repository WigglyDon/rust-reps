# Rust Reps

Rust Reps is a terminal Rust learning app for making something you understand:
type it, compile it, inspect its output, change it and fix mistakes. Curiosity
about the underlying mechanism motivates the work; no programming or CPU
knowledge is assumed.

The restored 22-drill course remains available. An opt-in workshop now provides
one real edit–compile–run preview while the curriculum and first-use experience
await Don's review. See [product direction](docs/PRODUCT.md) and
[curriculum design](docs/CURRICULUM.md). Task discussion lives on
[GitHub](https://github.com/WigglyDon/rust-reps/issues/1).

## Workshop preview

From this repository in an interactive Unix terminal, run:

```sh
cargo run --bin rust-reps -- workshop
```

Source is saved at `.rust-reps-local/workshop/main.rs` under the launch directory
(`/home/don/rust-reps/.rust-reps-local/workshop/main.rs` for Don's checkout).
A new file starts empty; Guidance shows a tiny complete program to type.
Guidance steps never replace your work. Reopening resumes your source.

- Ctrl-S: Save. F5: Save and Compile with installed `rustc` (Rust edition 2024).
- F6: Run the successful build for the unchanged displayed revision.
- F8: Stop. Esc/Ctrl-C: Save and Quit; write failures keep the buffer open.
- F2: Guidance/Compiler/Program tabs. F3/F4: previous/next guidance step.
- Tab: source/information focus. Arrows/Home/End edit or navigate; PageUp/PageDown
  and mouse wheel scroll information; the wheel also scrolls source under the pointer.

Try typing the first program, Compile, inspect diagnostics, then Run and inspect
Program output. Change its message, Compile and Run again. F4 describes adding
and changing a value in the same program; the final step invites removing `mut`,
reading a genuine compiler error and repairing it yourself. Authored examples
and observed output are labeled separately; successful compilation awards no score.

This Unix-only slice supports a single-file, standard-library-only program,
with closed stdin, one job at a time, a 30-second compilation limit, a 5-second
run limit and 64 KiB capture per stream. Previous revision/build output is labeled;
edits and failed/cancelled builds block stale execution. Build artifacts are
private temporary paths, removed on normal closure; the source persists and a
fresh build is needed after reopening. Save refuses ordinary external file
changes. Abrupt termination can lose edits since the last Save/Compile.

**Trusted locally authored code only.** Programs run deliberately with your user
permissions and can access your files and network. The working directory,
minimal environment, time limits and process groups are not a security sandbox.
Do not use this preview for hostile code or downloaded projects. It does not
support Cargo projects, dependencies, interactive input, an IDE or an emulator.
The [workshop contract](docs/context/subsystems/workshop.md) explains ownership,
cleanup and source recovery. Preview guidance needs Don's review.

## Player Mode

After local install, the command is `rust-reps` (`rust-reps.exe` on Windows).

```sh
cargo install --path . --force
rust-reps
rust-reps plain
rust-reps 001
rust-reps lab
rust-reps --list
rust-reps --help
rust-reps --version
cargo uninstall rust-reps
```

`rust-reps` without arguments starts the fullscreen TUI when run in an interactive terminal.

When stdin/stdout are piped, `rust-reps` uses deterministic plain practice mode.

`rust-reps plain` always starts deterministic plain practice mode.

`rust-reps 001` runs just drill 001 in one-shot mode.

`rust-reps lab` starts the fullscreen client stress lab in an interactive terminal.
It is an endless deterministic stress matrix, not curriculum, and it does not appear in `--list`.
Use it to test wrapped problem scrolling, multiline editor viewport behavior, syntax highlighting, exact multiline answers, and stable layout.

`rust-reps --list` prints the current catalog.

`rust-reps --help` prints all CLI forms.

`rust-reps --version` prints `rust-reps 0.1.0`.

### Fullscreen TUI

`rust-reps` starts fullscreen terminal mode in an interactive terminal.
It continues through deterministic five-task rounds until `Esc`, using Copy, Fill,
Produce, Repair, and Transform in that order. Two reviewed content rows alternate
deterministically. The TUI renders a terminal editor shell:

- left tablet: Rust editor/workbox
- right tablet: task/problem reference
- top-left over the editor: compact green round-position counter (`1/5` through `5/5`)
- below the editor: `F1 format · F2 help · F5 submit · F7 skip · Esc quit`

Keys:
- type normal characters to build input
- `{`, `(`, `[`, and `"` insert matching pairs and place the cursor between them
- generated closing delimiters are skipped over instead of duplicated
- `Backspace` deletes both delimiters for an empty generated pair
- `Enter` inserts a newline, copies indentation, and expands simple `{}` blocks
- `Delete` forward-deletes
- arrows, `Home`, and `End` move the cursor
- mouse click moves the editor cursor
- mouse wheel scrolls the editor or task pane under the pointer
- `F5` submits
- `F7` skips
- `F1` formats the editor buffer with `rustfmt` when the task enables formatting
- `F2` toggles resolved help for the current task without submitting or recording an attempt
- `Ctrl-Enter` submits only when the terminal reports it distinctly
- `Shift-Enter` inserts a newline when the terminal reports it distinctly
- `Esc` or `Ctrl-C` quits
- `Tab` is currently ignored

F1 format is an editor helper only. Copy and controlled Fill tasks keep it disabled.
Produce, Repair, and Transform enable it. It invokes the `rustfmt` executable directly,
never Clippy, Cargo, or `rustc`. When `rustfmt` is unavailable, fails, or times out,
Rust Reps leaves the editor buffer unchanged and shows local feedback.

The beginner varied sets introduce one return-type concept before guided practice.
Static focus emphasis marks the relevant stimulus region, and `F2` explains the
concept with a different worked example without displaying the active expected
answer. Help is presentation state only: it does not validate, advance, skip, or
award a rep.

Plain mode remains available with `rust-reps plain` and for piped stdin.

## Practice Mode

`rust-reps plain` starts one finite pass over all 22 drills in fixed order. Piped
noninteractive fallback is the same finite proof surface, and one-shot commands remain finite.

- `PASS` means the task’s answer rule accepted the input and advances to the next drill.
- `TRY AGAIN` means a wrong answer and repeats the same drill.
- `:skip` skips the current drill and advances without incrementing attempts.
- `:status` prints current state (`Current`, `Passed`, `Skipped`, `Attempts`) without advancing.
- `:list` prints all drills and does not advance.
- `:help` prints in-session commands.
- `:q` or `:quit` quits and prints:

```text
>QUIT
Summary:
Passed: ...
Skipped: ...
Attempts: ...
```

- `EOF` also exits with `QUIT` and summary.
- Completing all drills prints:

```text
COMPLETE
Summary:
Passed: ...
Skipped: ...
Attempts: ...
```

`rust-reps 001` does not show practice summary and does not use practice commands as control flow.

In practice mode, unknown colon input is a command error (not a try again):

```text
>UNKNOWN COMMAND
Type :help for commands.
```

## Exact Answer Contract

Exact answers are literal and deterministic:

- Only one trailing line ending is trimmed: `\n` or `\r\n`.
- Leading spaces are required and must match.
- Trailing spaces before newline are required and must match.
- Internal spacing is required and must match.
- Tabs, casing, punctuation, braces, quotes, and Unicode must match exactly.
- Multiline answers are treated as multiline and must match exactly.
- A blank line is a wrong answer.
- Wrong answers print `TRY AGAIN` plus `Expected:` and the exact expected answer.
- Unknown colon commands in practice are command errors and do not count attempts.
- Colon text in one-shot mode is just answer text, so `:help` can be a wrong answer there.

Copy and controlled Fill tasks use this exact contract. Produce, Repair, and
Transform use `RustfmtExact`: input is correct when its `rustfmt` output matches
the trusted answer's `rustfmt` output. That ignores formatting differences only;
it is not compilation, testing, or semantic equivalence.

## Current Drills

- `001` expects `fn main() {}`
- `002` expects `println!("Rust Reps");`
- `003` expects `let answer = 42;`
- `004` expects `let name = "Rust Reps";`
- `005` expects `let mut score = 0;`
- `006` expects `score += 1;`
- `007` expects `let passed = true;`
- `008` expects `let score: i32 = 0;`
- `009` expects `let scores = [1, 2, 3];`
- `010` expects `fn add_one(x: i32) -> i32 { x + 1 }`
- `011` expects `if score > 0 {}`
- `012` expects `while score < 10 {}`
- `013`–`017` practice one `five`/`i32`/`5` row as Copy, Fill, Produce, Repair, Transform
- `018`–`022` practice one `seven`/`u64`/`7` row in the same operation order

Each normal catalog rep also has static metadata: short ID, canonical ID, system,
track, semantic frame, surface frame, operation, concepts, constraints, XP,
answer rule, and transition. XP is stored metadata only and is not displayed.
Copy and controlled Fill use `ExactTrimFinalNewline`; full-code Produce, Repair,
and Transform use `RustfmtExact`. Every normal rep has no state transition.

## Development

Read [AGENTS.md](AGENTS.md), [the context index](docs/INDEX.md) and the
[development workflow](docs/DEVELOPMENT.md) for the assigned change.

```sh
cargo run --bin rust-reps
cargo run --bin rust-reps -- plain
cargo xtask context current --verify
cargo xtask context verify
cargo xtask verify
```

`cargo xtask verify` checks workspace formatting, warnings-denied Clippy,
product and developer-tool tests, context integrity, actual CLI transcripts,
a temporary installation and Git whitespace. It does not establish teaching
quality or real-terminal feel. Remote information in `context current` is
explicitly a local cache; fetch separately when fresh remote state is needed.

After reviewing changes to context owners, explicitly update their recorded hashes:

```sh
cargo xtask context manifest
cargo xtask context verify
```

Historical fleet commands remain available through `cargo xtask` and the thin
`scripts/fleet.sh` launcher. They preserve old evidence; they do not assign work
or require Don to relay worker packets. Current assignments and handback use
[GitHub issues](https://github.com/WigglyDon/rust-reps/issues). `main` is the
published branch; the restored `master` and donor worktrees preserve earlier work.

## PATH Troubleshooting

After `cargo install --path . --force`, confirm:

```sh
command -v rust-reps
rust-reps --version
```

If `command -v rust-reps` prints nothing, `~/.cargo/bin` is probably not on your `PATH` yet.
The binary is usually at `$HOME/.cargo/bin/rust-reps`.

Try opening a new terminal first.
If still missing, add Cargo bin directory:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

Then run:

```sh
command -v rust-reps
rust-reps --version
```

## Project Boundaries

- Core behavior currently uses `ratatui` + `crossterm` for interactive fullscreen TUI.
- Core practice flow stays terminal-first plain `stdout`/`stderr`.
- Persistence is limited to the opt-in workshop source file.
- No random mode
- Restored course validation remains text comparison; workshop compilation/run is separate.
- No semantic-equivalence, lint or test-runner validation.
- Plain practice remains available through `plain` and piped stdin.
