# Rusty Drill

A terminal-first Rust learning/drill CLI that teaches through tiny exact typing reps.

## Current V0 Behavior

Rusty Drill V0 shows Drill 001 and asks the user to type exactly:

```rust
fn main() {}
```

It reads one line from standard input.

For the exact answer, it prints:

```text
PASS
```

For anything else, it prints:

```text
TRY AGAIN
Expected:
fn main() {}
```

## Newline Rule

A trailing LF or CRLF from pressing Enter is ignored.

Spaces, punctuation, braces, tabs, casing, Unicode, and internal spacing are not normalized.

## Run

```sh
cargo run
```

## Test

```sh
cargo test
```

## Manual Verification

```sh
printf 'fn main() {}\n' | cargo run -q
printf 'fn main(){}\n' | cargo run -q
```

## Current Limitations

- One drill.
- One-line answers.
- Exact-text validation only.
- No progress tracking.
- No compilation validation.
- No external crates.
