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

section "unstaged diff check"
git diff --check

section "staged diff check"
git diff --cached --check

section "git status"
git status --short

section "latest commits"
git --no-pager log --oneline -5
