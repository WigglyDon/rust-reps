# Rust Reps

A terminal-first Rust learning/drill CLI that teaches through tiny exact typing reps.

## Current Behavior

Rust Reps starts a practice session and asks the user to type exact answers.

After local install, the command is `rust-reps` (`rust-reps.exe` on Windows).

By default, it starts practice mode:

```sh
rust-reps
```

You can also start practice mode explicitly:

```sh
rust-reps play
```

In practice mode, `:help` shows commands, `:status` shows session status, `:list` shows drills, and `:skip` skips the current drill. `:q` and `:quit` quit successfully. Passing every drill prints `COMPLETE`.
Practice mode prints a compact summary with passed, skipped, and attempts when it completes or quits; the summary is not saved.

Practice commands:

- `:help` shows commands.
- `:status` shows session status.
- `:list` shows drills.
- `:skip` skips the current drill.
- `:q` / `:quit` quits practice mode.

You can list drills, print help/version output, or run one selected drill by id:

```sh
rust-reps --help
rust-reps --list
rust-reps --version
rust-reps 001
rust-reps 002
rust-reps 003
rust-reps 004
rust-reps 005
rust-reps 006
rust-reps 007
rust-reps 008
rust-reps 009
rust-reps 010
rust-reps 011
rust-reps 012
```

Current drills:

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

Rust Reps reads one line from standard input. For the exact answer, it prints:

```text
PASS
```

For anything else, it prints `TRY AGAIN`, `Expected:`, and the selected drill's expected answer.
For Drill 001, that output is:

```text
TRY AGAIN
Expected:
fn main() {}
```

Unknown options, unknown drill ids, and too many arguments are CLI errors and exit nonzero.

## Newline Rule

A trailing LF or CRLF from pressing Enter is ignored.

Spaces, punctuation, braces, tabs, casing, Unicode, and internal spacing are not normalized.

## Install

```sh
cargo install --path .
```

## Play

```sh
rust-reps
```

Uninstall:

```sh
cargo uninstall rust-reps
```

## Developer Run

```sh
cargo run
cargo run -- play
cargo run -- --help
cargo run -- --list
cargo run -- --version
cargo run -- 001
cargo run -- 012
```

## Test

```sh
cargo test
```

## Local Proof

```sh
./scripts/proof.sh
```

This runs formatting, tests, CLI checks, and git whitespace checks.

## Current Limitations

- Fixed catalog.
- One-line answers.
- Exact-text validation only.
- No saved progress.
- No compilation validation.
- No external crates.
