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
println!("Rust Reps");
let answer = 42;
let name = "Rust Reps";
let mut score = 0;
score += 1;
let passed = true;
let score: i32 = 0;
let scores = [1, 2, 3];
fn add_one(x: i32) -> i32 { x + 1 }
if score > 0 {}
while score < 10 {}
fn five() -> i32 { 5 }
->
fn five()->i32{5}
fn five()->i32{5}
fn five()->i32{5}
fn seven() -> u64 { 7 }
->
fn seven()->u64{7}
fn seven()->u64{7}
fn seven()->u64{7}
'

drill_001_try_again_output='Rust Reps
----------------

Drill 001
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}'

drill_001_blank_line_output='Rust Reps
----------------

Drill 001
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}'

drill_001_crlf_pass_output='Rust Reps
----------------

Drill 001
Prompt:

Copy this line exactly:
fn main() {}

>PASS'

practice_all_correct_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>PASS

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>PASS

Drill 003/022
Prompt:

Copy this line exactly:
let answer = 42;

>PASS

Drill 004/022
Prompt:

Copy this line exactly:
let name = "Rust Reps";

>PASS

Drill 005/022
Prompt:

Copy this line exactly:
let mut score = 0;

>PASS

Drill 006/022
Prompt:

Copy this line exactly:
score += 1;

>PASS

Drill 007/022
Prompt:

Copy this line exactly:
let passed = true;

>PASS

Drill 008/022
Prompt:

Copy this line exactly:
let score: i32 = 0;

>PASS

Drill 009/022
Prompt:

Copy this line exactly:
let scores = [1, 2, 3];

>PASS

Drill 010/022
Prompt:

Copy this line exactly:
fn add_one(x: i32) -> i32 { x + 1 }

>PASS

Drill 011/022
Prompt:

Copy this line exactly:
if score > 0 {}

>PASS

Drill 012/022
Prompt:

Copy this line exactly:
while score < 10 {}

>PASS

Drill 013/022
Prompt:

Copy this line exactly: Type the complete function exactly.
fn five() -> i32 { 5 }

>PASS

Drill 014/022
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn five() ___ i32 { 5 }

>PASS

Drill 015/022
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn five() ___ i32 { 5 }

>PASS

Drill 016/022
Prompt:

Repair the return-type syntax in the highlighted region: Submit the complete corrected function.
fn five(): i32 { 5 }

>PASS

Drill 017/022
Prompt:

Add the declared return type after the parameter list: Submit the complete transformed function.
fn five() { 5 }

>PASS

Drill 018/022
Prompt:

Copy this line exactly: Type the complete function exactly.
fn seven() -> u64 { 7 }

>PASS

Drill 019/022
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn seven() ___ u64 { 7 }

>PASS

Drill 020/022
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn seven() ___ u64 { 7 }

>PASS

Drill 021/022
Prompt:

Repair the return-type syntax in the highlighted region: Submit the complete corrected function.
fn seven(): u64 { 7 }

>PASS

Drill 022/022
Prompt:

Add the declared return type after the parameter list: Submit the complete transformed function.
fn seven() { 7 }

>PASS

COMPLETE
Summary:
Passed: 22/22
Skipped: 0
Attempts: 22'

practice_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_unknown_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>UNKNOWN COMMAND
Type :help for commands.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_unknown_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>UNKNOWN COMMAND
Type :help for commands.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Status:
Current: 001/022
Passed: 0/22
Skipped: 0
Attempts: 0

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_blank_line_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Status:
Current: 001/022
Passed: 0/22
Skipped: 0
Attempts: 1

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 1'

practice_trailing_space_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Status:
Current: 001/022
Passed: 0/22
Skipped: 0
Attempts: 1

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 1'

practice_crlf_correct_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>PASS

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>QUIT
Summary:
Passed: 1/22
Skipped: 0
Attempts: 1'

practice_unknown_then_correct_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>UNKNOWN COMMAND
Type :help for commands.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>PASS

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>QUIT
Summary:
Passed: 1/22
Skipped: 0
Attempts: 1'

practice_retry_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>PASS

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>QUIT
Summary:
Passed: 1/22
Skipped: 0
Attempts: 2'

practice_help_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Commands:
:help    show commands
:status  show session status
:list    show drills
:skip    skip current drill
:q       quit
:quit    quit

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_wrong_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 1'

practice_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Status:
Current: 001/022
Passed: 0/22
Skipped: 0
Attempts: 0

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_list_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Available drills:
001  fn main() {}
002  println!("Rust Reps");
003  let answer = 42;
004  let name = "Rust Reps";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}
013  fn five() -> i32 { 5 }
014  fn five() ___ i32 { 5 }
015  fn five() ___ i32 { 5 }
016  fn five(): i32 { 5 }
017  fn five() { 5 }
018  fn seven() -> u64 { 7 }
019  fn seven() ___ u64 { 7 }
020  fn seven() ___ u64 { 7 }
021  fn seven(): u64 { 7 }
022  fn seven() { 7 }

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_skip_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>SKIP

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>QUIT
Summary:
Passed: 0/22
Skipped: 1
Attempts: 0'

practice_help_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Commands:
:help    show commands
:status  show session status
:list    show drills
:skip    skip current drill
:q       quit
:quit    quit

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Status:
Current: 001/022
Passed: 0/22
Skipped: 0
Attempts: 0

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 0'

practice_wrong_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>Status:
Current: 001/022
Passed: 0/22
Skipped: 0
Attempts: 1

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>QUIT
Summary:
Passed: 0/22
Skipped: 0
Attempts: 1'

practice_correct_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>PASS

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>Status:
Current: 002/022
Passed: 1/22
Skipped: 0
Attempts: 1

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>QUIT
Summary:
Passed: 1/22
Skipped: 0
Attempts: 1'

practice_skip_then_status_then_quit_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>SKIP

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>Status:
Current: 002/022
Passed: 0/22
Skipped: 1
Attempts: 0

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>QUIT
Summary:
Passed: 0/22
Skipped: 1
Attempts: 0'

practice_all_skip_input=':skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
'

practice_all_skip_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>SKIP

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>SKIP

Drill 003/022
Prompt:

Copy this line exactly:
let answer = 42;

>SKIP

Drill 004/022
Prompt:

Copy this line exactly:
let name = "Rust Reps";

>SKIP

Drill 005/022
Prompt:

Copy this line exactly:
let mut score = 0;

>SKIP

Drill 006/022
Prompt:

Copy this line exactly:
score += 1;

>SKIP

Drill 007/022
Prompt:

Copy this line exactly:
let passed = true;

>SKIP

Drill 008/022
Prompt:

Copy this line exactly:
let score: i32 = 0;

>SKIP

Drill 009/022
Prompt:

Copy this line exactly:
let scores = [1, 2, 3];

>SKIP

Drill 010/022
Prompt:

Copy this line exactly:
fn add_one(x: i32) -> i32 { x + 1 }

>SKIP

Drill 011/022
Prompt:

Copy this line exactly:
if score > 0 {}

>SKIP

Drill 012/022
Prompt:

Copy this line exactly:
while score < 10 {}

>SKIP

Drill 013/022
Prompt:

Copy this line exactly: Type the complete function exactly.
fn five() -> i32 { 5 }

>SKIP

Drill 014/022
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn five() ___ i32 { 5 }

>SKIP

Drill 015/022
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn five() ___ i32 { 5 }

>SKIP

Drill 016/022
Prompt:

Repair the return-type syntax in the highlighted region: Submit the complete corrected function.
fn five(): i32 { 5 }

>SKIP

Drill 017/022
Prompt:

Add the declared return type after the parameter list: Submit the complete transformed function.
fn five() { 5 }

>SKIP

Drill 018/022
Prompt:

Copy this line exactly: Type the complete function exactly.
fn seven() -> u64 { 7 }

>SKIP

Drill 019/022
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn seven() ___ u64 { 7 }

>SKIP

Drill 020/022
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn seven() ___ u64 { 7 }

>SKIP

Drill 021/022
Prompt:

Repair the return-type syntax in the highlighted region: Submit the complete corrected function.
fn seven(): u64 { 7 }

>SKIP

Drill 022/022
Prompt:

Add the declared return type after the parameter list: Submit the complete transformed function.
fn seven() { 7 }

>SKIP
COMPLETE
Summary:
Passed: 0/22
Skipped: 22
Attempts: 0'

practice_mixed_complete_input='wrong
fn main() {}
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
:skip
'

practice_mixed_complete_output='Rust Reps
----------------

Practice
Deterministic Rust practice.
Follow the task instruction.
Type :help for commands and :q to quit.

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>TRY AGAIN
Expected:
fn main() {}

Drill 001/022
Prompt:

Copy this line exactly:
fn main() {}

>PASS

Drill 002/022
Prompt:

Copy this line exactly:
println!("Rust Reps");

>SKIP

Drill 003/022
Prompt:

Copy this line exactly:
let answer = 42;

>SKIP

Drill 004/022
Prompt:

Copy this line exactly:
let name = "Rust Reps";

>SKIP

Drill 005/022
Prompt:

Copy this line exactly:
let mut score = 0;

>SKIP

Drill 006/022
Prompt:

Copy this line exactly:
score += 1;

>SKIP

Drill 007/022
Prompt:

Copy this line exactly:
let passed = true;

>SKIP

Drill 008/022
Prompt:

Copy this line exactly:
let score: i32 = 0;

>SKIP

Drill 009/022
Prompt:

Copy this line exactly:
let scores = [1, 2, 3];

>SKIP

Drill 010/022
Prompt:

Copy this line exactly:
fn add_one(x: i32) -> i32 { x + 1 }

>SKIP

Drill 011/022
Prompt:

Copy this line exactly:
if score > 0 {}

>SKIP

Drill 012/022
Prompt:

Copy this line exactly:
while score < 10 {}

>SKIP

Drill 013/022
Prompt:

Copy this line exactly: Type the complete function exactly.
fn five() -> i32 { 5 }

>SKIP

Drill 014/022
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn five() ___ i32 { 5 }

>SKIP

Drill 015/022
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn five() ___ i32 { 5 }

>SKIP

Drill 016/022
Prompt:

Repair the return-type syntax in the highlighted region: Submit the complete corrected function.
fn five(): i32 { 5 }

>SKIP

Drill 017/022
Prompt:

Add the declared return type after the parameter list: Submit the complete transformed function.
fn five() { 5 }

>SKIP

Drill 018/022
Prompt:

Copy this line exactly: Type the complete function exactly.
fn seven() -> u64 { 7 }

>SKIP

Drill 019/022
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn seven() ___ u64 { 7 }

>SKIP

Drill 020/022
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn seven() ___ u64 { 7 }

>SKIP

Drill 021/022
Prompt:

Repair the return-type syntax in the highlighted region: Submit the complete corrected function.
fn seven(): u64 { 7 }

>SKIP

Drill 022/022
Prompt:

Add the declared return type after the parameter list: Submit the complete transformed function.
fn seven() { 7 }

>SKIP
COMPLETE
Summary:
Passed: 1/22
Skipped: 21
Attempts: 2'

expect_stdout "practice default COMPLETE" "$practice_all_correct_output" sh -c 'printf "%s" "$1" | cargo run -q' sh "$practice_all_correct_input"
expect_stdout "practice play COMPLETE" "$practice_all_correct_output" sh -c 'printf "%s" "$1" | cargo run -q -- play' sh "$practice_all_correct_input"
expect_stdout "practice quit" "$practice_quit_output" sh -c "printf ':q\\n' | cargo run -q"
expect_stdout "practice help then quit" "$practice_help_then_quit_output" sh -c "printf ':help\\n:q\\n' | cargo run -q"
expect_stdout "practice help then status then quit" "$practice_help_then_status_then_quit_output" sh -c "printf ':help\\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice unknown then status then quit" "$practice_unknown_then_status_then_quit_output" sh -c "printf ':unknown\\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice blank line then status then quit" "$practice_blank_line_then_status_then_quit_output" sh -c "printf '\\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice trailing space then status then quit" "$practice_trailing_space_then_status_then_quit_output" sh -c "printf 'fn main() {} \\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice CRLF correct then quit" "$practice_crlf_correct_then_quit_output" sh -c "printf 'fn main() {}\\r\\n:q\\r\\n' | cargo run -q"
expect_stdout "practice unknown then correct then quit" "$practice_unknown_then_correct_then_quit_output" sh -c "printf ':unknown\\nfn main() {}\\n:q\\n' | cargo run -q"
expect_stdout "practice unknown command then quit" "$practice_unknown_then_quit_output" sh -c "printf ':stats\\n:q\\n' | cargo run -q"
expect_stdout "practice wrong then status then quit" "$practice_wrong_then_status_then_quit_output" sh -c "printf 'wrong\\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice correct then status then quit" "$practice_correct_then_status_then_quit_output" sh -c "printf 'fn main() {}\\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice status then quit" "$practice_status_then_quit_output" sh -c "printf ':status\\n:q\\n' | cargo run -q"
expect_stdout "practice list then quit" "$practice_list_then_quit_output" sh -c "printf ':list\\n:q\\n' | cargo run -q"
expect_stdout "practice quit command" "$practice_quit_output" sh -c "printf ':quit\\n' | cargo run -q"
expect_stdout "practice skip then quit" "$practice_skip_then_quit_output" sh -c "printf ':skip\\n:q\\n' | cargo run -q"
expect_stdout "practice skip then status then quit" "$practice_skip_then_status_then_quit_output" sh -c "printf ':skip\\n:status\\n:q\\n' | cargo run -q"
expect_stdout "practice retry then quit" "$practice_retry_then_quit_output" sh -c "printf 'fn main(){}\\nfn main() {}\\n:q\\n' | cargo run -q"
expect_stdout "practice wrong then quit" "$practice_wrong_then_quit_output" sh -c "printf 'wrong\\n:q\\n' | cargo run -q"
expect_stdout "practice unknown then correct then quit" "$practice_unknown_then_correct_then_quit_output" sh -c "printf ':unknown\\nfn main() {}\\n:q\\n' | cargo run -q"
expect_stdout "practice all skip complete" "$practice_all_skip_output" sh -c "printf '%s' \"$practice_all_skip_input\" | cargo run -q"
expect_stdout "practice mixed complete" "$practice_mixed_complete_output" sh -c "printf '%s' \"$practice_mixed_complete_input\" | cargo run -q"

expect_stdout "Drill 001 explicit PASS" 'Rust Reps
----------------

Drill 001
Prompt:

Copy this line exactly:
fn main() {}

>PASS' sh -c "printf 'fn main() {}\\n' | cargo run -q -- 001"

expect_stdout "list" 'Rust Reps
----------------

Available drills:
001  fn main() {}
002  println!("Rust Reps");
003  let answer = 42;
004  let name = "Rust Reps";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}
013  fn five() -> i32 { 5 }
014  fn five() ___ i32 { 5 }
015  fn five() ___ i32 { 5 }
016  fn five(): i32 { 5 }
017  fn five() { 5 }
018  fn seven() -> u64 { 7 }
019  fn seven() ___ u64 { 7 }
020  fn seven() ___ u64 { 7 }
021  fn seven(): u64 { 7 }
022  fn seven() { 7 }' cargo run -q -- --list

expect_stdout "help" 'Rust Reps
----------------

Usage:
  rust-reps
  rust-reps plain
  rust-reps [DRILL_ID]
  rust-reps play
  rust-reps lab
  rust-reps --list
  rust-reps --help
  rust-reps --version

Default:
  Starts fullscreen TUI in an interactive terminal.
  Uses plain practice mode when stdin/stdout are piped.

Lab:
  rust-reps lab starts a fullscreen client stress lab in an interactive terminal.
  It is not curriculum and does not appear in --list.

Current drills:
001  fn main() {}
002  println!("Rust Reps");
003  let answer = 42;
004  let name = "Rust Reps";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}
013  fn five() -> i32 { 5 }
014  fn five() ___ i32 { 5 }
015  fn five() ___ i32 { 5 }
016  fn five(): i32 { 5 }
017  fn five() { 5 }
018  fn seven() -> u64 { 7 }
019  fn seven() ___ u64 { 7 }
020  fn seven() ___ u64 { 7 }
021  fn seven(): u64 { 7 }
022  fn seven() { 7 }' cargo run -q -- --help

expect_stdout "short help" 'Rust Reps
----------------

Usage:
  rust-reps
  rust-reps plain
  rust-reps [DRILL_ID]
  rust-reps play
  rust-reps lab
  rust-reps --list
  rust-reps --help
  rust-reps --version

Default:
  Starts fullscreen TUI in an interactive terminal.
  Uses plain practice mode when stdin/stdout are piped.

Lab:
  rust-reps lab starts a fullscreen client stress lab in an interactive terminal.
  It is not curriculum and does not appear in --list.

Current drills:
001  fn main() {}
002  println!("Rust Reps");
003  let answer = 42;
004  let name = "Rust Reps";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}
013  fn five() -> i32 { 5 }
014  fn five() ___ i32 { 5 }
015  fn five() ___ i32 { 5 }
016  fn five(): i32 { 5 }
017  fn five() { 5 }
018  fn seven() -> u64 { 7 }
019  fn seven() ___ u64 { 7 }
020  fn seven() ___ u64 { 7 }
021  fn seven(): u64 { 7 }
022  fn seven() { 7 }' cargo run -q -- -h

expect_stdout "version" 'rust-reps 0.1.0' cargo run -q -- --version

expect_stdout "one-shot wrong" "$drill_001_try_again_output" sh -c "printf 'wrong\\n' | cargo run -q -- 001"
expect_stdout "one-shot blank line" "$drill_001_blank_line_output" sh -c "printf '\\n' | cargo run -q -- 001"
expect_stdout "one-shot trailing space" "$drill_001_try_again_output" sh -c "printf 'fn main() {} \\n' | cargo run -q -- 001"
expect_stdout "one-shot CRLF correct" "$drill_001_crlf_pass_output" sh -c "printf 'fn main() {}\\r\\n' | cargo run -q -- 001"
expect_stdout "one-shot help as wrong answer" "$drill_001_try_again_output" sh -c "printf ':help\\n' | cargo run -q -- 001"

expect_stdout "Drill 013 exact Copy PASS" 'Rust Reps
----------------

Drill 013
Prompt:

Copy this line exactly: Type the complete function exactly.
fn five() -> i32 { 5 }

>PASS' sh -c "printf 'fn five() -> i32 { 5 }\\n' | cargo run -q -- 013"

expect_stdout "Drill 014 controlled Fill PASS" 'Rust Reps
----------------

Drill 014
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn five() ___ i32 { 5 }

>PASS' sh -c "printf '%s\\n' '->' | cargo run -q -- 014"

expect_stdout "Drill 014 extra space TRY AGAIN" 'Rust Reps
----------------

Drill 014
Prompt:

Complete the missing return-type arrow: Type only the code replacing ___.
fn five() ___ i32 { 5 }

>TRY AGAIN
Expected:
->' sh -c "printf '%s\\n' '-> ' | cargo run -q -- 014"

expect_stdout "Drill 015 formatting-different PASS" 'Rust Reps
----------------

Drill 015
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn five() ___ i32 { 5 }

>PASS' sh -c "printf 'fn five()->i32{5}\\n' | cargo run -q -- 015"

expect_stdout "Drill 015 valid different TRY AGAIN" 'Rust Reps
----------------

Drill 015
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn five() ___ i32 { 5 }

>TRY AGAIN
Expected:
fn five() -> i32 { 5 }' sh -c "printf 'fn other()->i32{5}\\n' | cargo run -q -- 015"

expect_stdout "Drill 015 invalid Rust TRY AGAIN" 'Rust Reps
----------------

Drill 015
Prompt:

Use the shown parts to write the function with its return type: Submit the complete function.
fn five() ___ i32 { 5 }

>TRY AGAIN
Expected:
fn five() -> i32 { 5 }' sh -c "printf 'fn {\\n' | cargo run -q -- 015"
expect_error "lab requires tty" "error: lab mode requires an interactive terminal" cargo run -q -- lab

section "temporary install"
CARGO_NET_OFFLINE=true cargo install --locked --path . --force --root "$install_root"

expect_stdout "installed command path" "$install_root/bin/rust-reps" env PATH="$install_root/bin:$PATH" sh -c 'command -v rust-reps'
expect_stdout "installed version" 'rust-reps 0.1.0' env PATH="$install_root/bin:$PATH" rust-reps --version
expect_stdout "installed help" 'Rust Reps
----------------

Usage:
  rust-reps
  rust-reps plain
  rust-reps [DRILL_ID]
  rust-reps play
  rust-reps lab
  rust-reps --list
  rust-reps --help
  rust-reps --version

Default:
  Starts fullscreen TUI in an interactive terminal.
  Uses plain practice mode when stdin/stdout are piped.

Lab:
  rust-reps lab starts a fullscreen client stress lab in an interactive terminal.
  It is not curriculum and does not appear in --list.

Current drills:
001  fn main() {}
002  println!("Rust Reps");
003  let answer = 42;
004  let name = "Rust Reps";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}
013  fn five() -> i32 { 5 }
014  fn five() ___ i32 { 5 }
015  fn five() ___ i32 { 5 }
016  fn five(): i32 { 5 }
017  fn five() { 5 }
018  fn seven() -> u64 { 7 }
019  fn seven() ___ u64 { 7 }
020  fn seven() ___ u64 { 7 }
021  fn seven(): u64 { 7 }
022  fn seven() { 7 }' env PATH="$install_root/bin:$PATH" rust-reps --help
expect_error "installed lab requires tty" "error: lab mode requires an interactive terminal" env PATH="$install_root/bin:$PATH" rust-reps lab
expect_stdout "installed list" 'Rust Reps
----------------

Available drills:
001  fn main() {}
002  println!("Rust Reps");
003  let answer = 42;
004  let name = "Rust Reps";
005  let mut score = 0;
006  score += 1;
007  let passed = true;
008  let score: i32 = 0;
009  let scores = [1, 2, 3];
010  fn add_one(x: i32) -> i32 { x + 1 }
011  if score > 0 {}
012  while score < 10 {}
013  fn five() -> i32 { 5 }
014  fn five() ___ i32 { 5 }
015  fn five() ___ i32 { 5 }
016  fn five(): i32 { 5 }
017  fn five() { 5 }
018  fn seven() -> u64 { 7 }
019  fn seven() ___ u64 { 7 }
020  fn seven() ___ u64 { 7 }
021  fn seven(): u64 { 7 }
022  fn seven() { 7 }' env PATH="$install_root/bin:$PATH" rust-reps --list
expect_stdout "installed practice quit" "$practice_quit_output" env PATH="$install_root/bin:$PATH" sh -c 'printf ":q\n" | rust-reps'
expect_stdout "installed practice help then quit" "$practice_help_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c 'printf ":help\n:q\n" | rust-reps'
expect_stdout "installed practice help then status then quit" "$practice_help_then_status_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c 'printf ":help\n:status\n:q\n" | rust-reps'
expect_stdout "installed unknown command then status then quit" "$practice_unknown_then_status_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf ':unknown\\n:status\\n:q\\n' | rust-reps"
expect_stdout "installed unknown command then correct then quit" "$practice_unknown_then_correct_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf ':unknown\\nfn main() {}\\n:q\\n' | rust-reps"
expect_stdout "installed practice correct then status then quit" "$practice_correct_then_status_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf 'fn main() {}\\n:status\\n:q\\n' | rust-reps"
expect_stdout "installed practice skip then status then quit" "$practice_skip_then_status_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf ':skip\\n:status\\n:q\\n' | rust-reps"
expect_stdout "installed practice blank line then quit" "$practice_wrong_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf '\\n:q\\n' | rust-reps"
expect_stdout "installed practice CRLF correct then quit" "$practice_crlf_correct_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf 'fn main() {}\\r\\n:q\\r\\n' | rust-reps"
expect_stdout "installed unknown command smoke" "$practice_unknown_then_quit_output" env PATH="$install_root/bin:$PATH" sh -c "printf ':unknown\\n:q\\n' | rust-reps"
expect_stdout "installed practice all skips complete" "$practice_all_skip_output" env PATH="$install_root/bin:$PATH" sh -c "printf '%s' \"$practice_all_skip_input\" | rust-reps"
expect_stdout "installed practice mixed complete" "$practice_mixed_complete_output" env PATH="$install_root/bin:$PATH" sh -c "printf '%s' \"$practice_mixed_complete_input\" | rust-reps"
expect_stdout "installed Drill 001 PASS" "$drill_001_crlf_pass_output" env PATH="$install_root/bin:$PATH" sh -c 'printf "fn main() {}\n" | rust-reps 001'
expect_stdout "installed Drill 001 CRLF PASS" "$drill_001_crlf_pass_output" env PATH="$install_root/bin:$PATH" sh -c 'printf "fn main() {}\r\n" | rust-reps 001'
expect_stdout "installed Drill 001 blank line" "$drill_001_blank_line_output" env PATH="$install_root/bin:$PATH" sh -c "printf '\\n' | rust-reps 001"

expect_stdout "Drill 002 PASS" 'Rust Reps
----------------

Drill 002
Prompt:

Copy this line exactly:
println!("Rust Reps");

>PASS' sh -c "printf 'println!(\"Rust Reps\");\\n' | cargo run -q -- 002"

expect_stdout "Drill 006 PASS" 'Rust Reps
----------------

Drill 006
Prompt:

Copy this line exactly:
score += 1;

>PASS' sh -c "printf 'score += 1;\\n' | cargo run -q -- 006"

expect_stdout "Drill 007 PASS" 'Rust Reps
----------------

Drill 007
Prompt:

Copy this line exactly:
let passed = true;

>PASS' sh -c "printf 'let passed = true;\\n' | cargo run -q -- 007"

expect_stdout "Drill 008 PASS" 'Rust Reps
----------------

Drill 008
Prompt:

Copy this line exactly:
let score: i32 = 0;

>PASS' sh -c "printf 'let score: i32 = 0;\\n' | cargo run -q -- 008"

expect_stdout "Drill 009 PASS" 'Rust Reps
----------------

Drill 009
Prompt:

Copy this line exactly:
let scores = [1, 2, 3];

>PASS' sh -c "printf 'let scores = [1, 2, 3];\\n' | cargo run -q -- 009"

expect_stdout "Drill 010 PASS" 'Rust Reps
----------------

Drill 010
Prompt:

Copy this line exactly:
fn add_one(x: i32) -> i32 { x + 1 }

>PASS' sh -c "printf 'fn add_one(x: i32) -> i32 { x + 1 }\\n' | cargo run -q -- 010"

expect_stdout "Drill 011 PASS" 'Rust Reps
----------------

Drill 011
Prompt:

Copy this line exactly:
if score > 0 {}

>PASS' sh -c "printf 'if score > 0 {}\\n' | cargo run -q -- 011"

expect_stdout "Drill 012 PASS" 'Rust Reps
----------------

Drill 012
Prompt:

Copy this line exactly:
while score < 10 {}

>PASS' sh -c "printf 'while score < 10 {}\\n' | cargo run -q -- 012"

expect_stdout "Drill 006 TRY AGAIN" 'Rust Reps
----------------

Drill 006
Prompt:

Copy this line exactly:
score += 1;

>TRY AGAIN
Expected:
score += 1;' sh -c "printf 'score += 1\\n' | cargo run -q -- 006"

expect_stdout "Drill 012 TRY AGAIN" 'Rust Reps
----------------

Drill 012
Prompt:

Copy this line exactly:
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
