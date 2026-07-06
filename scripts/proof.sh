#!/bin/sh
set -eu

if [ ! -f Cargo.toml ] || [ ! -f src/main.rs ]; then
    echo "error: run from repo root" >&2
    exit 1
fi

section() {
    printf '%s\n' "--- $1 ---"
}

install_root=$(mktemp -d)

cleanup() {
    rm -rf "$install_root"
}

trap cleanup EXIT HUP INT TERM

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

practice_all_correct_input='fn main() {}
println!("Rusty Drill");
let answer = 42;
let name = "Rusty Drill";
let mut score = 0;
score += 1;
let passed = true;
let score: i32 = 0;
let scores = [1, 2, 3];
fn add_one(x: i32) -> i32 { x + 1 }
if score > 0 {}
while score < 10 {}
'

practice_all_correct_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>PASS

Drill 002/012
Type exactly:

println!("Rusty Drill");

>PASS

Drill 003/012
Type exactly:

let answer = 42;

>PASS

Drill 004/012
Type exactly:

let name = "Rusty Drill";

>PASS

Drill 005/012
Type exactly:

let mut score = 0;

>PASS

Drill 006/012
Type exactly:

score += 1;

>PASS

Drill 007/012
Type exactly:

let passed = true;

>PASS

Drill 008/012
Type exactly:

let score: i32 = 0;

>PASS

Drill 009/012
Type exactly:

let scores = [1, 2, 3];

>PASS

Drill 010/012
Type exactly:

fn add_one(x: i32) -> i32 { x + 1 }

>PASS

Drill 011/012
Type exactly:

if score > 0 {}

>PASS

Drill 012/012
Type exactly:

while score < 10 {}

>PASS

COMPLETE
Summary:
Passed: 12/12
Skipped: 0
Attempts: 12'

practice_quit_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>QUIT
Summary:
Passed: 0/12
Skipped: 0
Attempts: 0'

practice_retry_then_quit_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/012
Type exactly:

fn main() {}

>PASS

Drill 002/012
Type exactly:

println!("Rusty Drill");

>QUIT
Summary:
Passed: 1/12
Skipped: 0
Attempts: 2'

practice_help_then_quit_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>Commands:
:help    show commands
:status  show session status
:list    show drills
:skip    skip current drill
:q       quit
:quit    quit

Drill 001/012
Type exactly:

fn main() {}

>QUIT
Summary:
Passed: 0/12
Skipped: 0
Attempts: 0'

practice_status_then_quit_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>Status:
Current: 001/012
Passed: 0/12
Skipped: 0
Attempts: 0

Drill 001/012
Type exactly:

fn main() {}

>QUIT
Summary:
Passed: 0/12
Skipped: 0
Attempts: 0'

practice_list_then_quit_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>Available drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}

Drill 001/012
Type exactly:

fn main() {}

>QUIT
Summary:
Passed: 0/12
Skipped: 0
Attempts: 0'

practice_skip_then_quit_output='Rusty Drill

Practice
Type :help for commands.

Drill 001/012
Type exactly:

fn main() {}

>SKIP

Drill 002/012
Type exactly:

println!("Rusty Drill");

>QUIT
Summary:
Passed: 0/12
Skipped: 1
Attempts: 0'

expect_stdout "practice default COMPLETE" "$practice_all_correct_output" sh -c 'printf "%s" "$1" | cargo run -q' sh "$practice_all_correct_input"
expect_stdout "practice play COMPLETE" "$practice_all_correct_output" sh -c 'printf "%s" "$1" | cargo run -q -- play' sh "$practice_all_correct_input"
expect_stdout "practice quit" "$practice_quit_output" sh -c "printf ':q\\n' | cargo run -q"
expect_stdout "practice help then quit" "$practice_help_then_quit_output" sh -c "printf ':help\\n:q\\n' | cargo run -q"
expect_stdout "practice status then quit" "$practice_status_then_quit_output" sh -c "printf ':status\\n:q\\n' | cargo run -q"
expect_stdout "practice list then quit" "$practice_list_then_quit_output" sh -c "printf ':list\\n:q\\n' | cargo run -q"
expect_stdout "practice skip then quit" "$practice_skip_then_quit_output" sh -c "printf ':skip\\n:q\\n' | cargo run -q"
expect_stdout "practice retry then quit" "$practice_retry_then_quit_output" sh -c "printf 'fn main(){}\\nfn main() {}\\n:q\\n' | cargo run -q"

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
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}' cargo run -q -- --list

expect_stdout "help" 'Rusty Drill

Usage:
  rusty-drill
  rusty-drill [DRILL_ID]
  rusty-drill play
  rusty-drill --list
  rusty-drill --help
  rusty-drill --version

Default:
  Starts a practice session.

Current drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}' cargo run -q -- --help

expect_stdout "short help" 'Rusty Drill

Usage:
  rusty-drill
  rusty-drill [DRILL_ID]
  rusty-drill play
  rusty-drill --list
  rusty-drill --help
  rusty-drill --version

Default:
  Starts a practice session.

Current drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}' cargo run -q -- -h

expect_stdout "version" 'rusty-drill 0.1.0' cargo run -q -- --version

section "temporary install"
cargo install --path . --root "$install_root"

expect_stdout "installed version" 'rusty-drill 0.1.0' "$install_root/bin/rusty-drill" --version
expect_stdout "installed help" 'Rusty Drill

Usage:
  rusty-drill
  rusty-drill [DRILL_ID]
  rusty-drill play
  rusty-drill --list
  rusty-drill --help
  rusty-drill --version

Default:
  Starts a practice session.

Current drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}' "$install_root/bin/rusty-drill" --help
expect_stdout "installed list" 'Rusty Drill

Available drills:
001  fn main() {}
002  println!("Rusty Drill");
003  let answer = 42;
004  let name = "Rusty Drill";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}' "$install_root/bin/rusty-drill" --list
expect_stdout "installed practice quit" "$practice_quit_output" sh -c 'printf ":q\n" | "$1/bin/rusty-drill"' sh "$install_root"
expect_stdout "installed practice help then quit" "$practice_help_then_quit_output" sh -c 'printf ":help\n:q\n" | "$1/bin/rusty-drill"' sh "$install_root"
expect_stdout "installed Drill 001 PASS" 'Rusty Drill

Drill 001
Type exactly:

fn main() {}

>PASS' sh -c 'printf "fn main() {}\n" | "$1/bin/rusty-drill" 001' sh "$install_root"

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

expect_stdout "Drill 007 PASS" 'Rusty Drill

Drill 007
Type exactly:

let passed = true;

>PASS' sh -c "printf 'let passed = true;\\n' | cargo run -q -- 007"

expect_stdout "Drill 008 PASS" 'Rusty Drill

Drill 008
Type exactly:

let score: i32 = 0;

>PASS' sh -c "printf 'let score: i32 = 0;\\n' | cargo run -q -- 008"

expect_stdout "Drill 009 PASS" 'Rusty Drill

Drill 009
Type exactly:

let scores = [1, 2, 3];

>PASS' sh -c "printf 'let scores = [1, 2, 3];\\n' | cargo run -q -- 009"

expect_stdout "Drill 010 PASS" 'Rusty Drill

Drill 010
Type exactly:

fn add_one(x: i32) -> i32 { x + 1 }

>PASS' sh -c "printf 'fn add_one(x: i32) -> i32 { x + 1 }\\n' | cargo run -q -- 010"

expect_stdout "Drill 011 PASS" 'Rusty Drill

Drill 011
Type exactly:

if score > 0 {}

>PASS' sh -c "printf 'if score > 0 {}\\n' | cargo run -q -- 011"

expect_stdout "Drill 012 PASS" 'Rusty Drill

Drill 012
Type exactly:

while score < 10 {}

>PASS' sh -c "printf 'while score < 10 {}\\n' | cargo run -q -- 012"

expect_stdout "Drill 006 TRY AGAIN" 'Rusty Drill

Drill 006
Type exactly:

score += 1;

>TRY AGAIN
Expected:
score += 1;' sh -c "printf 'score += 1\\n' | cargo run -q -- 006"

expect_stdout "Drill 012 TRY AGAIN" 'Rusty Drill

Drill 012
Type exactly:

while score < 10 {}

>TRY AGAIN
Expected:
while score < 10 {}' sh -c "printf 'while score < 10\\n' | cargo run -q -- 012"

expect_error "unknown id error" "error: unknown drill id: 999" cargo run -q -- 999
expect_error "unknown option error" "error: unknown argument: --wat" cargo run -q -- --wat
expect_error "too many args error" "error: expected at most one argument" cargo run -q -- 001 extra
expect_error "too many help args error" "error: expected at most one argument" cargo run -q -- --help extra
expect_error "too many version args error" "error: expected at most one argument" cargo run -q -- --version extra

section "unstaged diff check"
git diff --check

section "staged diff check"
git diff --cached --check

section "git status"
git status --short

section "latest commits"
git --no-pager log --oneline -5
