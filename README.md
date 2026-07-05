# Rusty Drill

A terminal-first Rust learning/drill CLI that teaches through tiny exact typing reps.

## Current Behavior

Rusty Drill shows a drill and asks the user to type an exact answer.

By default, it runs Drill 001:

```sh
cargo run
```

You can also select a drill by id:

```sh
cargo run -- 001
cargo run -- 002
```

Current drills:

- `001` expects `fn main() {}`
- `002` expects `println!("Rusty Drill");`

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

Unknown drill ids are CLI errors and exit nonzero.

## Newline Rule

A trailing LF or CRLF from pressing Enter is ignored.

Spaces, punctuation, braces, tabs, casing, Unicode, and internal spacing are not normalized.

## Run

```sh
cargo run
cargo run -- 001
cargo run -- 002
```

## Test

```sh
cargo test
```

## Manual Verification

```sh
printf 'fn main() {}\n' | cargo run -q
printf 'fn main(){}\n' | cargo run -q
printf 'println!("Rusty Drill");\n' | cargo run -q -- 002
```

## Local Proof

```sh
./scripts/proof-v0.sh
```

This runs formatting, tests, CLI checks, and git whitespace checks.

## Current Limitations

- Two fixed drills.
- One-line answers.
- Exact-text validation only.
- No progress tracking.
- No compilation validation.
- No external crates.
