# Rust Reps

Rust Reps is a terminal-first Rust practice game.
It teaches one-line Rust reps through exact-match typing in plain terminal output.
The catalog is small and placeholder right now.

## Player Mode

After local install, the command is `rust-reps` (`rust-reps.exe` on Windows).

```sh
cargo install --path . --force
rust-reps
rust-reps plain
rust-reps 001
rust-reps --list
rust-reps --help
rust-reps --version
cargo uninstall rust-reps
```

`rust-reps` without arguments starts the fullscreen TUI when run in an interactive terminal.

When stdin/stdout are piped, `rust-reps` uses deterministic plain practice mode.

`rust-reps plain` always starts deterministic plain practice mode.

`rust-reps 001` runs just drill 001 in one-shot mode.

`rust-reps --list` prints the current catalog.

`rust-reps --help` prints all CLI forms.

`rust-reps --version` prints `rust-reps 0.1.0`.

### Fullscreen TUI

`rust-reps` starts fullscreen terminal mode in an interactive terminal.
It uses the same drill and answer truth as plain mode, but renders a simple game window:

- top half: problem
- bottom half: workbox
- status and summary outside the game window

Keys:
- type normal characters to build input
- `Backspace` deletes
- `Enter` inserts a newline
- `Ctrl-Enter` submits
- `Esc` or `Ctrl-C` quits
- `F1` or `?` toggles help panel
- `Tab` is currently ignored

Plain mode remains available with `rust-reps plain` and for piped stdin.

## Practice Mode

`rust-reps plain` starts practice mode with drills in fixed order.

- `PASS` means exact answer match and advances to the next drill.
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

## Current Drills (Placeholder Catalog)

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

## Developer Mode

```sh
cargo run
cargo run -- plain
cargo run -- 001
cargo test
./scripts/proof.sh
```

For CLI-only validation, use:

```sh
cargo fmt --check
```

## Proof / Quality Gates

- `cargo fmt --check`
- `cargo test`
- `./scripts/proof.sh`
- `git diff --check`
- `git diff --cached --check` (if staged)
- installed-command proof when install behavior changes (requires real command path checks)

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
- No persistence
- No random mode
- No compilation validation
- Plain practice remains available through `plain` and piped stdin.
