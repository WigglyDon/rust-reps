use std::io::Write;
use std::process::{Command, Output, Stdio};

const DRILL_001_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 001\nType exactly:\n\nfn main() {}\n\n>PASS\n";
const DRILL_001_TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 001\nType exactly:\n\nfn main() {}\n\n>TRY AGAIN\nExpected:\nfn main() {}\n";
const DRILL_002_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 002\nType exactly:\n\nprintln!(\"Rusty Drill\");\n\n>PASS\n";
const DRILL_002_TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 002\nType exactly:\n\nprintln!(\"Rusty Drill\");\n\n>TRY AGAIN\nExpected:\nprintln!(\"Rusty Drill\");\n";
const DRILL_003_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 003\nType exactly:\n\nlet answer = 42;\n\n>PASS\n";
const DRILL_004_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 004\nType exactly:\n\nlet name = \"Rusty Drill\";\n\n>PASS\n";
const DRILL_005_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 005\nType exactly:\n\nlet mut score = 0;\n\n>PASS\n";
const DRILL_006_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 006\nType exactly:\n\nscore += 1;\n\n>PASS\n";
const DRILL_006_TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 006\nType exactly:\n\nscore += 1;\n\n>TRY AGAIN\nExpected:\nscore += 1;\n";
const DRILL_007_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 007\nType exactly:\n\nlet passed = true;\n\n>PASS\n";
const DRILL_008_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 008\nType exactly:\n\nlet score: i32 = 0;\n\n>PASS\n";
const DRILL_009_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 009\nType exactly:\n\nlet scores = [1, 2, 3];\n\n>PASS\n";
const DRILL_010_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 010\nType exactly:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>PASS\n";
const DRILL_011_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 011\nType exactly:\n\nif score > 0 {}\n\n>PASS\n";
const DRILL_012_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 012\nType exactly:\n\nwhile score < 10 {}\n\n>PASS\n";
const DRILL_012_TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 012\nType exactly:\n\nwhile score < 10 {}\n\n>TRY AGAIN\nExpected:\nwhile score < 10 {}\n";
const LIST_OUTPUT: &str = "Rusty Drill\n\nAvailable drills:\n001  fn main() {}\n002  println!(\"Rusty Drill\");\n003  let answer = 42;\n004  let name = \"Rusty Drill\";\n005  let mut score = 0;\n006  score += 1;\n007  let passed = true;\n008  let score: i32 = 0;\n009  let scores = [1, 2, 3];\n010  fn add_one(x: i32) -> i32 { x + 1 }\n011  if score > 0 {}\n012  while score < 10 {}\n";
const HELP_OUTPUT: &str = "Rusty Drill\n\nUsage:\n  rusty-drill\n  rusty-drill [DRILL_ID]\n  rusty-drill play\n  rusty-drill --list\n  rusty-drill --help\n  rusty-drill --version\n\nDefault:\n  Starts a practice session.\n\nCurrent drills:\n001  fn main() {}\n002  println!(\"Rusty Drill\");\n003  let answer = 42;\n004  let name = \"Rusty Drill\";\n005  let mut score = 0;\n006  score += 1;\n007  let passed = true;\n008  let score: i32 = 0;\n009  let scores = [1, 2, 3];\n010  fn add_one(x: i32) -> i32 { x + 1 }\n011  if score > 0 {}\n012  while score < 10 {}\n";
const VERSION_OUTPUT: &str = "rusty-drill 0.1.0\n";
const PRACTICE_ALL_CORRECT_INPUT: &str = concat!(
    "fn main() {}\n",
    "println!(\"Rusty Drill\");\n",
    "let answer = 42;\n",
    "let name = \"Rusty Drill\";\n",
    "let mut score = 0;\n",
    "score += 1;\n",
    "let passed = true;\n",
    "let score: i32 = 0;\n",
    "let scores = [1, 2, 3];\n",
    "fn add_one(x: i32) -> i32 { x + 1 }\n",
    "if score > 0 {}\n",
    "while score < 10 {}\n",
);
const PRACTICE_ALL_CORRECT_OUTPUT: &str = concat!(
    "Rusty Drill\n\n",
    "Practice\n",
    "Type :q to quit.\n\n",
    "Drill 001/012\nType exactly:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nType exactly:\n\nprintln!(\"Rusty Drill\");\n\n>PASS\n\n",
    "Drill 003/012\nType exactly:\n\nlet answer = 42;\n\n>PASS\n\n",
    "Drill 004/012\nType exactly:\n\nlet name = \"Rusty Drill\";\n\n>PASS\n\n",
    "Drill 005/012\nType exactly:\n\nlet mut score = 0;\n\n>PASS\n\n",
    "Drill 006/012\nType exactly:\n\nscore += 1;\n\n>PASS\n\n",
    "Drill 007/012\nType exactly:\n\nlet passed = true;\n\n>PASS\n\n",
    "Drill 008/012\nType exactly:\n\nlet score: i32 = 0;\n\n>PASS\n\n",
    "Drill 009/012\nType exactly:\n\nlet scores = [1, 2, 3];\n\n>PASS\n\n",
    "Drill 010/012\nType exactly:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>PASS\n\n",
    "Drill 011/012\nType exactly:\n\nif score > 0 {}\n\n>PASS\n\n",
    "Drill 012/012\nType exactly:\n\nwhile score < 10 {}\n\n>PASS\n\n",
    "COMPLETE\n",
    "Summary:\n",
    "Passed: 12/12\n",
    "Attempts: 12\n",
);
const PRACTICE_QUIT_OUTPUT: &str = concat!(
    "Rusty Drill\n\n",
    "Practice\n",
    "Type :q to quit.\n\n",
    "Drill 001/012\nType exactly:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Attempts: 0\n",
);
const PRACTICE_RETRY_THEN_QUIT_OUTPUT: &str = concat!(
    "Rusty Drill\n\n",
    "Practice\n",
    "Type :q to quit.\n\n",
    "Drill 001/012\nType exactly:\n\nfn main() {}\n\n>TRY AGAIN\n",
    "Expected:\n",
    "fn main() {}\n\n",
    "Drill 001/012\nType exactly:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nType exactly:\n\nprintln!(\"Rusty Drill\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 1/12\n",
    "Attempts: 2\n",
);

fn run_rusty_drill(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rusty-drill"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start rusty-drill");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    stdin
        .write_all(input.as_bytes())
        .expect("failed to write stdin");
    drop(stdin);

    child.wait_with_output().expect("failed to wait for output")
}

#[test]
fn default_practice_all_correct_exits_successfully_and_prints_complete() {
    let output = run_rusty_drill(&[], PRACTICE_ALL_CORRECT_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_ALL_CORRECT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn play_practice_all_correct_exits_successfully_and_prints_complete() {
    let output = run_rusty_drill(&["play"], PRACTICE_ALL_CORRECT_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_ALL_CORRECT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_wrong_answer_repeats_then_accepts_correct_answer() {
    let output = run_rusty_drill(&[], "fn main(){}\nfn main() {}\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_RETRY_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_q_exits_successfully_and_prints_quit() {
    let output = run_rusty_drill(&[], ":q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_quit_exits_successfully_and_prints_quit() {
    let output = run_rusty_drill(&[], ":quit\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_eof_before_completion_exits_successfully_and_prints_quit() {
    let output = run_rusty_drill(&[], "");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn explicit_drill_001_crlf_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["001"], "fn main() {}\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_PASS_OUTPUT
    );
}

#[test]
fn missing_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&["001"], "fn main(){}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn double_internal_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&["001"], "fn  main() {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn explicit_drill_001_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["001"], "fn main() {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_PASS_OUTPUT
    );
}

#[test]
fn one_shot_selected_drill_outputs_do_not_include_summary() {
    let drill_001_output = run_rusty_drill(&["001"], "fn main() {}\n");
    let drill_012_output = run_rusty_drill(&["012"], "while score < 10 {}\n");

    assert!(drill_001_output.status.success());
    assert!(drill_012_output.status.success());
    assert!(
        !String::from_utf8(drill_001_output.stdout)
            .unwrap()
            .contains("Summary:")
    );
    assert!(
        !String::from_utf8(drill_012_output.stdout)
            .unwrap()
            .contains("Summary:")
    );
}

#[test]
fn drill_002_lf_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["002"], "println!(\"Rusty Drill\");\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_002_PASS_OUTPUT
    );
}

#[test]
fn drill_002_crlf_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["002"], "println!(\"Rusty Drill\");\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_002_PASS_OUTPUT
    );
}

#[test]
fn drill_002_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&["002"], "println!(\"Rusty Drill\")\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_002_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn list_command_exits_successfully_and_prints_catalog() {
    let output = run_rusty_drill(&["--list"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), LIST_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn help_command_exits_successfully_and_prints_help() {
    let output = run_rusty_drill(&["--help"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), HELP_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn short_help_command_exits_successfully_and_prints_help() {
    let output = run_rusty_drill(&["-h"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), HELP_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn version_command_exits_successfully_and_prints_version() {
    let output = run_rusty_drill(&["--version"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), VERSION_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn drill_003_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["003"], "let answer = 42;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_003_PASS_OUTPUT
    );
}

#[test]
fn drill_004_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["004"], "let name = \"Rusty Drill\";\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_004_PASS_OUTPUT
    );
}

#[test]
fn drill_005_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["005"], "let mut score = 0;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_005_PASS_OUTPUT
    );
}

#[test]
fn drill_006_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["006"], "score += 1;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_006_PASS_OUTPUT
    );
}

#[test]
fn drill_006_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&["006"], "score += 1\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_006_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn drill_007_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["007"], "let passed = true;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_007_PASS_OUTPUT
    );
}

#[test]
fn drill_008_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["008"], "let score: i32 = 0;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_008_PASS_OUTPUT
    );
}

#[test]
fn drill_009_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["009"], "let scores = [1, 2, 3];\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_009_PASS_OUTPUT
    );
}

#[test]
fn drill_010_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["010"], "fn add_one(x: i32) -> i32 { x + 1 }\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_010_PASS_OUTPUT
    );
}

#[test]
fn drill_011_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["011"], "if score > 0 {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_011_PASS_OUTPUT
    );
}

#[test]
fn drill_012_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&["012"], "while score < 10 {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_012_PASS_OUTPUT
    );
}

#[test]
fn drill_012_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&["012"], "while score < 10\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_012_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn unknown_drill_id_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["999"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown drill id: 999\n"
    );
}

#[test]
fn too_many_args_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["001", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn list_extra_arg_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["--list", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn unknown_long_option_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["--wat"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown argument: --wat\n"
    );
}

#[test]
fn unknown_short_option_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["-x"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown argument: -x\n"
    );
}

#[test]
fn help_extra_arg_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["--help", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn version_extra_arg_exits_nonzero_and_prints_error() {
    let output = run_rusty_drill(&["--version", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn stderr_is_empty_for_normal_product_results() {
    let practice_output = run_rusty_drill(&[], PRACTICE_ALL_CORRECT_INPUT);
    let pass_output = run_rusty_drill(&["001"], "fn main() {}\n");
    let try_again_output = run_rusty_drill(&["012"], "while score < 10\n");
    let quit_output = run_rusty_drill(&[], ":q\n");
    let list_output = run_rusty_drill(&["--list"], "");
    let help_output = run_rusty_drill(&["--help"], "");
    let version_output = run_rusty_drill(&["--version"], "");

    assert!(practice_output.status.success());
    assert!(pass_output.status.success());
    assert!(try_again_output.status.success());
    assert!(quit_output.status.success());
    assert!(list_output.status.success());
    assert!(help_output.status.success());
    assert!(version_output.status.success());
    assert_eq!(String::from_utf8(practice_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(pass_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(try_again_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(quit_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(list_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(help_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(version_output.stderr).unwrap(), "");
}
