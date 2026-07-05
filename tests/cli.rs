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
const LIST_OUTPUT: &str = "Rusty Drill\n\nAvailable drills:\n001  fn main() {}\n002  println!(\"Rusty Drill\");\n003  let answer = 42;\n004  let name = \"Rusty Drill\";\n005  let mut score = 0;\n006  score += 1;\n";

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
fn exact_lf_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&[], "fn main() {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_PASS_OUTPUT
    );
}

#[test]
fn exact_crlf_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill(&[], "fn main() {}\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_PASS_OUTPUT
    );
}

#[test]
fn missing_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&[], "fn main(){}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn double_internal_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill(&[], "fn  main() {}\n");

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
fn stderr_is_empty_for_normal_product_results() {
    let pass_output = run_rusty_drill(&[], "fn main() {}\n");
    let try_again_output = run_rusty_drill(&["006"], "score += 1\n");
    let list_output = run_rusty_drill(&["--list"], "");

    assert!(pass_output.status.success());
    assert!(try_again_output.status.success());
    assert!(list_output.status.success());
    assert_eq!(String::from_utf8(pass_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(try_again_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(list_output.stderr).unwrap(), "");
}
