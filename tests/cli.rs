use std::io::Write;
use std::process::{Command, Output, Stdio};

const DRILL_001_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 001\nType exactly:\n\nfn main() {}\n\n>PASS\n";
const DRILL_001_TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 001\nType exactly:\n\nfn main() {}\n\n>TRY AGAIN\nExpected:\nfn main() {}\n";
const DRILL_002_PASS_OUTPUT: &str =
    "Rusty Drill\n\nDrill 002\nType exactly:\n\nprintln!(\"Rusty Drill\");\n\n>PASS\n";
const DRILL_002_TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 002\nType exactly:\n\nprintln!(\"Rusty Drill\");\n\n>TRY AGAIN\nExpected:\nprintln!(\"Rusty Drill\");\n";

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
        "error: expected at most one drill id\n"
    );
}

#[test]
fn stderr_is_empty_for_normal_product_results() {
    let pass_output = run_rusty_drill(&[], "fn main() {}\n");
    let try_again_output = run_rusty_drill(&["002"], "println!(\"Rusty Drill\")\n");

    assert!(pass_output.status.success());
    assert!(try_again_output.status.success());
    assert_eq!(String::from_utf8(pass_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(try_again_output.stderr).unwrap(), "");
}
