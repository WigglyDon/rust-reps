use std::io::Write;
use std::process::{Command, Output, Stdio};

const PASS_OUTPUT: &str = "Rusty Drill\n\nDrill 001\nType exactly:\n\nfn main() {}\n\n>PASS\n";
const TRY_AGAIN_OUTPUT: &str = "Rusty Drill\n\nDrill 001\nType exactly:\n\nfn main() {}\n\n>TRY AGAIN\nExpected:\nfn main() {}\n";

fn run_rusty_drill(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rusty-drill"))
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
    let output = run_rusty_drill("fn main() {}\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), PASS_OUTPUT);
}

#[test]
fn exact_crlf_input_exits_successfully_and_prints_pass() {
    let output = run_rusty_drill("fn main() {}\r\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), PASS_OUTPUT);
}

#[test]
fn missing_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill("fn main(){}\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), TRY_AGAIN_OUTPUT);
}

#[test]
fn double_internal_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rusty_drill("fn  main() {}\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), TRY_AGAIN_OUTPUT);
}

#[test]
fn stderr_is_empty_for_normal_product_results() {
    let pass_output = run_rusty_drill("fn main() {}\n");
    let try_again_output = run_rusty_drill("fn main(){}\n");

    assert!(pass_output.status.success());
    assert!(try_again_output.status.success());
    assert_eq!(String::from_utf8(pass_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(try_again_output.stderr).unwrap(), "");
}
