#!/bin/sh
set -eu

if [ ! -f Cargo.toml ] || [ ! -f src/main.rs ]; then
    echo "error: run from repo root" >&2
    exit 1
fi

section() {
    printf '%s\n' "--- $1 ---"
}

section "fmt check"
cargo fmt --check

section "tests"
cargo test

section "LF pass"
printf 'fn main() {}\n' | cargo run -q

section "CRLF pass"
printf 'fn main() {}\r\n' | cargo run -q

section "missing-space fail"
printf 'fn main(){}\n' | cargo run -q

section "double-space fail"
printf 'fn  main() {}\n' | cargo run -q

section "Drill 002 LF pass"
printf 'println!("Rusty Drill");\n' | cargo run -q -- 002

section "Drill 002 CRLF pass"
printf 'println!("Rusty Drill");\r\n' | cargo run -q -- 002

section "Drill 002 missing-semicolon fail"
printf 'println!("Rusty Drill")\n' | cargo run -q -- 002

section "unknown id error"
set +e
unknown_output=$(cargo run -q -- 999 2>&1)
unknown_status=$?
set -e
printf '%s\n' "$unknown_output"
printf 'status: %s\n' "$unknown_status"
if [ "$unknown_status" -eq 0 ]; then
    echo "error: expected unknown drill id to exit nonzero" >&2
    exit 1
fi

section "unstaged diff check"
git diff --check

section "staged diff check"
git diff --cached --check

section "git status"
git status --short

section "latest commits"
git --no-pager log --oneline -5
