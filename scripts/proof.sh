#!/bin/sh
set -eu

if [ ! -f Cargo.toml ] || [ ! -f src/main.rs ]; then
    echo "error: run from repo root" >&2
    exit 1
fi

section() {
    printf '%s\n' "--- $1 ---"
}

expect_stdout() {
    label=$1
    expected=$2
    shift 2

    section "$label"
    expected_file=$(mktemp)
    actual_file=$(mktemp)
    stderr_file=$(mktemp)

    printf '%s\n' "$expected" > "$expected_file"

    set +e
    "$@" > "$actual_file" 2> "$stderr_file"
    status=$?
    set -e

    if [ "$status" -ne 0 ]; then
        echo "error: expected command to exit successfully" >&2
        printf 'status: %s\n' "$status" >&2
        echo "stderr:" >&2
        cat "$stderr_file" >&2
        rm -f "$expected_file" "$actual_file" "$stderr_file"
        exit 1
    fi

    if [ -s "$stderr_file" ]; then
        echo "error: expected empty stderr" >&2
        echo "actual stderr:" >&2
        cat "$stderr_file" >&2
        rm -f "$expected_file" "$actual_file" "$stderr_file"
        exit 1
    fi

    if ! cmp -s "$expected_file" "$actual_file"; then
        echo "error: unexpected stdout" >&2
        echo "expected:" >&2
        cat "$expected_file" >&2
        echo "actual:" >&2
        cat "$actual_file" >&2
        rm -f "$expected_file" "$actual_file" "$stderr_file"
        exit 1
    fi

    cat "$actual_file"
    rm -f "$expected_file" "$actual_file" "$stderr_file"
}

expect_error() {
    label=$1
    expected=$2
    shift 2

    section "$label"
    expected_file=$(mktemp)
    stdout_file=$(mktemp)
    stderr_file=$(mktemp)

    printf '%s\n' "$expected" > "$expected_file"

    set +e
    "$@" > "$stdout_file" 2> "$stderr_file"
    status=$?
    set -e

    cat "$stderr_file"
    printf 'status: %s\n' "$status"

    if [ "$status" -eq 0 ]; then
        echo "error: expected command to exit nonzero" >&2
        rm -f "$expected_file" "$stdout_file" "$stderr_file"
        exit 1
    fi

    if [ -s "$stdout_file" ]; then
        echo "error: expected empty stdout" >&2
        echo "actual stdout:" >&2
        cat "$stdout_file" >&2
        rm -f "$expected_file" "$stdout_file" "$stderr_file"
        exit 1
    fi

    if ! cmp -s "$expected_file" "$stderr_file"; then
        echo "error: unexpected stderr" >&2
        echo "expected:" >&2
        cat "$expected_file" >&2
        echo "actual:" >&2
        cat "$stderr_file" >&2
        rm -f "$expected_file" "$stdout_file" "$stderr_file"
        exit 1
    fi

    rm -f "$expected_file" "$stdout_file" "$stderr_file"
}

section "fmt check"
cargo fmt --check

section "tests"
cargo test

expect_stdout "Drill 001 default PASS" 'Rusty Drill

Drill 001
Type exactly:

fn main() {}

>PASS' sh -c "printf 'fn main() {}\\n' | cargo run -q"

expect_stdout "Drill 001 explicit PASS" 'Rusty Drill

Drill 001
Type exactly:

fn main() {}

>PASS' sh -c "printf 'fn main() {}\\n' | cargo run -q -- 001"

expect_stdout "list" 'Rusty Drill

Available drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;' cargo run -q -- --list

expect_stdout "help" 'Rusty Drill

Usage:
  rusty-drill [DRILL_ID]
  rusty-drill --list
  rusty-drill --help

Default:
  Runs Drill 001.

Current drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;' cargo run -q -- --help

expect_stdout "short help" 'Rusty Drill

Usage:
  rusty-drill [DRILL_ID]
  rusty-drill --list
  rusty-drill --help

Default:
  Runs Drill 001.

Current drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;' cargo run -q -- -h

expect_stdout "Drill 002 PASS" 'Rusty Drill

Drill 002
Type exactly:

println!("Rusty Drill");

>PASS' sh -c "printf 'println!(\"Rusty Drill\");\\n' | cargo run -q -- 002"

expect_stdout "Drill 006 PASS" 'Rusty Drill

Drill 006
Type exactly:

score += 1;

>PASS' sh -c "printf 'score += 1;\\n' | cargo run -q -- 006"

expect_stdout "Drill 006 TRY AGAIN" 'Rusty Drill

Drill 006
Type exactly:

score += 1;

>TRY AGAIN
Expected:
score += 1;' sh -c "printf 'score += 1\\n' | cargo run -q -- 006"

expect_error "unknown id error" "error: unknown drill id: 999" cargo run -q -- 999
expect_error "unknown option error" "error: unknown argument: --wat" cargo run -q -- --wat
expect_error "too many args error" "error: expected at most one argument" cargo run -q -- 001 extra
expect_error "too many help args error" "error: expected at most one argument" cargo run -q -- --help extra

section "unstaged diff check"
git diff --check

section "staged diff check"
git diff --cached --check

section "git status"
git status --short

section "latest commits"
git --no-pager log --oneline -5
