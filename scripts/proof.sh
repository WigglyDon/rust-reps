#!/bin/sh
set -eu

if [ ! -f Cargo.toml ] || [ ! -f src/main.rs ]; then
    echo "error: run from repo root" >&2
    exit 1
fi

section() {
    printf '%s\n' "--- $1 ---"
}

expect_error() {
    label=$1
    expected=$2
    shift 2

    section "$label"
    set +e
    output=$("$@" 2>&1)
    status=$?
    set -e

    printf '%s\n' "$output"
    printf 'status: %s\n' "$status"

    if [ "$status" -eq 0 ]; then
        echo "error: expected command to exit nonzero" >&2
        exit 1
    fi

    if [ "$output" != "$expected" ]; then
        echo "error: unexpected stderr" >&2
        exit 1
    fi
}

section "fmt check"
cargo fmt --check

section "tests"
cargo test

section "Drill 001 default PASS"
printf 'fn main() {}\n' | cargo run -q

section "list"
cargo run -q -- --list

section "Drill 002 PASS"
printf 'println!("Rusty Drill");\n' | cargo run -q -- 002

section "Drill 003 PASS"
printf 'let answer = 42;\n' | cargo run -q -- 003

section "Drill 004 PASS"
printf 'let name = "Rusty Drill";\n' | cargo run -q -- 004

section "Drill 005 PASS"
printf 'let mut score = 0;\n' | cargo run -q -- 005

section "Drill 006 PASS"
printf 'score += 1;\n' | cargo run -q -- 006

section "Drill 006 TRY AGAIN"
printf 'score += 1\n' | cargo run -q -- 006

expect_error "unknown id error" "error: unknown drill id: 999" cargo run -q -- 999
expect_error "too many args error" "error: expected at most one argument" cargo run -q -- 001 extra

section "unstaged diff check"
git diff --check

section "staged diff check"
git diff --cached --check

section "git status"
git status --short

section "latest commits"
git --no-pager log --oneline -5
