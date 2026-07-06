# Rusty Drill

A terminal-first Rust learning/drill CLI that teaches through tiny exact typing reps.

## Current Behavior

Rusty Drill starts a practice session and asks the user to type exact answers.

By default, it starts practice mode:

```sh
cargo run
```

You can also start practice mode explicitly:

```sh
cargo run -- play
```

In practice mode, `:help` shows commands, `:status` shows session status, `:list` shows drills, and `:skip` skips the current drill. `:q` and `:quit` quit successfully. Passing every drill prints `COMPLETE`.
Practice mode prints a compact summary with passed, skipped, and attempts when it completes or quits; the summary is not saved.

You can list drills, print help/version output, or run one selected drill by id:

```sh
cargo run -- --help
cargo run -- --list
cargo run -- --version
cargo run -- 001
cargo run -- 002
cargo run -- 003
cargo run -- 004
cargo run -- 005
cargo run -- 006
cargo run -- 007
cargo run -- 008
cargo run -- 009
cargo run -- 010
cargo run -- 011
cargo run -- 012
```

Current drills:

- `001` expects `fn main() {}`
- `002` expects `println!("Rusty Drill");`
- `003` expects `let answer = 42;`
- `004` expects `let name = "Rusty Drill";`
- `005` expects `let mut score = 0;`
- `006` expects `score += 1;`
- `007` expects `let passed = true;`
- `008` expects `let score: i32 = 0;`
- `009` expects `let scores = [1, 2, 3];`
- `010` expects `fn add_one(x: i32) -> i32 { x + 1 }`
- `011` expects `if score > 0 {}`
- `012` expects `while score < 10 {}`

Rusty Drill reads one line from standard input. For the exact answer, it prints:

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

## Run

```sh
cargo run
cargo run -- play
cargo run -- --help
cargo run -- --list
cargo run -- --version
cargo run -- 001
cargo run -- 002
cargo run -- 003
cargo run -- 004
cargo run -- 005
cargo run -- 006
cargo run -- 007
cargo run -- 008
cargo run -- 009
cargo run -- 010
cargo run -- 011
cargo run -- 012
```

## Test

```sh
cargo test
```

## Manual Verification

```sh
printf ':q\n' | cargo run -q
printf 'fn main() {}\n' | cargo run -q -- 001
printf 'fn main(){}\n' | cargo run -q -- 001
printf 'println!("Rusty Drill");\n' | cargo run -q -- 002
printf 'score += 1;\n' | cargo run -q -- 006
printf 'while score < 10 {}\n' | cargo run -q -- 012
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
