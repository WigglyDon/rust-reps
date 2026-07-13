#![allow(dead_code)]

use std::{
    collections::VecDeque,
    io::{Read, Write},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const RUSTFMT_COMMAND: &str = "rustfmt";
const REAL_TOOL_TIMEOUT_MS: u64 = 900;
const REAL_TOOL_POLL_INTERVAL_MS: u64 = 10;
const REAL_TOOL_OUTPUT_LIMIT_BYTES: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolKind {
    Formatter,
    Compiler,
    Linter,
    TestHarness,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolWorkingDir {
    None,
    Isolated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolInvocation {
    pub label: String,
    pub args: Vec<String>,
}

impl ToolInvocation {
    pub fn label(label: &str) -> Self {
        Self {
            label: label.to_string(),
            args: Vec::new(),
        }
    }

    pub fn with_arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolRequest {
    pub tool: ToolKind,
    pub invocation: ToolInvocation,
    pub stdin: String,
    pub working_dir: ToolWorkingDir,
}

impl ToolRequest {
    pub fn new(tool: ToolKind, label: &str) -> Self {
        Self {
            tool,
            invocation: ToolInvocation::label(label),
            stdin: String::new(),
            working_dir: ToolWorkingDir::None,
        }
    }

    pub fn with_arg(mut self, arg: &str) -> Self {
        self.invocation = self.invocation.with_arg(arg);
        self
    }

    pub fn with_stdin(mut self, stdin: &str) -> Self {
        self.stdin = stdin.to_string();
        self
    }

    pub fn with_working_dir(mut self, working_dir: ToolWorkingDir) -> Self {
        self.working_dir = working_dir;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedOutput {
    pub text: String,
    pub truncated: bool,
}

impl CapturedOutput {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            truncated: false,
        }
    }

    pub fn truncated(text: &str) -> Self {
        Self {
            text: text.to_string(),
            truncated: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolStatus {
    Success,
    Nonzero(i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolOutput {
    pub status: ToolStatus,
    pub stdout: CapturedOutput,
    pub stderr: CapturedOutput,
}

impl ToolOutput {
    pub fn success(stdout: &str, stderr: &str) -> Self {
        Self {
            status: ToolStatus::Success,
            stdout: CapturedOutput::new(stdout),
            stderr: CapturedOutput::new(stderr),
        }
    }

    pub fn nonzero(code: i32, stdout: &str, stderr: &str) -> Self {
        Self {
            status: ToolStatus::Nonzero(code),
            stdout: CapturedOutput::new(stdout),
            stderr: CapturedOutput::new(stderr),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolOutputStream {
    Stdout,
    Stderr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolFailure {
    Unavailable { tool: ToolKind },
    Timeout { tool: ToolKind },
    InvalidInvocation { reason: String },
    OutputLimitExceeded { stream: ToolOutputStream },
    SetupFailed { reason: String },
    NoConfiguredOutcome,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolResult {
    Output(ToolOutput),
    Failure(ToolFailure),
}

impl ToolResult {
    pub fn success(stdout: &str, stderr: &str) -> Self {
        Self::Output(ToolOutput::success(stdout, stderr))
    }

    pub fn nonzero(code: i32, stdout: &str, stderr: &str) -> Self {
        Self::Output(ToolOutput::nonzero(code, stdout, stderr))
    }
}

pub trait ToolRunner {
    fn run(&mut self, request: &ToolRequest) -> ToolResult;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FakeToolRunner {
    outcomes: VecDeque<ToolResult>,
    requests: Vec<ToolRequest>,
}

impl FakeToolRunner {
    pub fn new(outcomes: Vec<ToolResult>) -> Self {
        Self {
            outcomes: outcomes.into(),
            requests: Vec::new(),
        }
    }

    pub fn run(&mut self, request: &ToolRequest) -> ToolResult {
        self.requests.push(request.clone());
        self.outcomes
            .pop_front()
            .unwrap_or(ToolResult::Failure(ToolFailure::NoConfiguredOutcome))
    }

    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    pub fn requests(&self) -> &[ToolRequest] {
        &self.requests
    }
}

impl ToolRunner for FakeToolRunner {
    fn run(&mut self, request: &ToolRequest) -> ToolResult {
        FakeToolRunner::run(self, request)
    }
}

#[derive(Debug, Clone)]
pub struct RealToolRunner {
    formatter_command: &'static str,
    timeout: Duration,
    output_limit_bytes: usize,
}

impl RealToolRunner {
    pub fn new() -> Self {
        Self {
            formatter_command: RUSTFMT_COMMAND,
            timeout: Duration::from_millis(REAL_TOOL_TIMEOUT_MS),
            output_limit_bytes: REAL_TOOL_OUTPUT_LIMIT_BYTES,
        }
    }

    #[cfg(test)]
    fn with_fixed_formatter_command_for_test(formatter_command: &'static str) -> Self {
        Self {
            formatter_command,
            ..Self::new()
        }
    }

    #[cfg(test)]
    fn with_timeout_for_test(timeout: Duration) -> Self {
        Self {
            timeout,
            ..Self::new()
        }
    }

    #[cfg(test)]
    fn with_output_limit_for_test(output_limit_bytes: usize) -> Self {
        Self {
            output_limit_bytes,
            ..Self::new()
        }
    }

    fn validate_request(&self, request: &ToolRequest) -> Result<(), ToolFailure> {
        if request.tool != ToolKind::Formatter {
            return Err(ToolFailure::InvalidInvocation {
                reason: "real runner only supports formatter requests".to_string(),
            });
        }

        if request.working_dir != ToolWorkingDir::None {
            return Err(ToolFailure::InvalidInvocation {
                reason: "real runner does not support custom working directories".to_string(),
            });
        }

        if request.invocation.args.len() != 2
            || request.invocation.args[0] != "--emit"
            || request.invocation.args[1] != "stdout"
        {
            return Err(ToolFailure::InvalidInvocation {
                reason: "real runner only supports rustfmt --emit stdout".to_string(),
            });
        }

        Ok(())
    }

    fn run_request(&self, request: &ToolRequest) -> ToolResult {
        if let Err(failure) = self.validate_request(request) {
            return ToolResult::Failure(failure);
        }

        let mut child = match Command::new(self.formatter_command)
            .arg("--emit")
            .arg("stdout")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return ToolResult::Failure(ToolFailure::Unavailable { tool: request.tool });
            }
            Err(error) => {
                return ToolResult::Failure(ToolFailure::SetupFailed {
                    reason: error.to_string(),
                });
            }
        };

        if self.timeout.is_zero() {
            stop_child(&mut child);
            return ToolResult::Failure(ToolFailure::Timeout { tool: request.tool });
        }

        let Some(mut stdin) = child.stdin.take() else {
            stop_child(&mut child);
            return ToolResult::Failure(ToolFailure::SetupFailed {
                reason: "failed to open rustfmt stdin".to_string(),
            });
        };

        if let Err(error) = stdin.write_all(request.stdin.as_bytes()) {
            stop_child(&mut child);
            return ToolResult::Failure(ToolFailure::SetupFailed {
                reason: error.to_string(),
            });
        }
        drop(stdin);

        let started_at = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    let stdout = match read_limited(child.stdout.take(), self.output_limit_bytes) {
                        Ok(output) => output,
                        Err(failure) => return ToolResult::Failure(failure),
                    };
                    if stdout.truncated {
                        return ToolResult::Failure(ToolFailure::OutputLimitExceeded {
                            stream: ToolOutputStream::Stdout,
                        });
                    }

                    let stderr = match read_limited(child.stderr.take(), self.output_limit_bytes) {
                        Ok(output) => output,
                        Err(failure) => return ToolResult::Failure(failure),
                    };
                    if stderr.truncated {
                        return ToolResult::Failure(ToolFailure::OutputLimitExceeded {
                            stream: ToolOutputStream::Stderr,
                        });
                    }

                    let tool_status = if status.success() {
                        ToolStatus::Success
                    } else {
                        ToolStatus::Nonzero(status.code().unwrap_or(-1))
                    };

                    return ToolResult::Output(ToolOutput {
                        status: tool_status,
                        stdout,
                        stderr,
                    });
                }
                Ok(None) if started_at.elapsed() >= self.timeout => {
                    stop_child(&mut child);
                    return ToolResult::Failure(ToolFailure::Timeout { tool: request.tool });
                }
                Ok(None) => {
                    thread::sleep(Duration::from_millis(REAL_TOOL_POLL_INTERVAL_MS));
                }
                Err(error) => {
                    stop_child(&mut child);
                    return ToolResult::Failure(ToolFailure::SetupFailed {
                        reason: error.to_string(),
                    });
                }
            }
        }
    }
}

impl Default for RealToolRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRunner for RealToolRunner {
    fn run(&mut self, request: &ToolRequest) -> ToolResult {
        self.run_request(request)
    }
}

fn stop_child(child: &mut std::process::Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn read_limited<R: Read>(
    pipe: Option<R>,
    output_limit_bytes: usize,
) -> Result<CapturedOutput, ToolFailure> {
    let Some(mut pipe) = pipe else {
        return Ok(CapturedOutput::new(""));
    };

    let mut bytes = Vec::new();
    let mut buffer = [0; 4096];

    loop {
        let read = pipe
            .read(&mut buffer)
            .map_err(|error| ToolFailure::SetupFailed {
                reason: error.to_string(),
            })?;

        if read == 0 {
            break;
        }

        let remaining = output_limit_bytes.saturating_sub(bytes.len());
        if read > remaining {
            bytes.extend_from_slice(&buffer[..remaining]);
            return Ok(captured_from_bytes(bytes, true));
        }

        bytes.extend_from_slice(&buffer[..read]);
    }

    Ok(captured_from_bytes(bytes, false))
}

fn captured_from_bytes(bytes: Vec<u8>, truncated: bool) -> CapturedOutput {
    CapturedOutput {
        text: String::from_utf8_lossy(&bytes).into_owned(),
        truncated,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CapturedOutput, FakeToolRunner, RealToolRunner, ToolFailure, ToolKind, ToolOutput,
        ToolOutputStream, ToolRequest, ToolResult, ToolRunner, ToolStatus, ToolWorkingDir,
    };
    use std::time::Duration;

    fn request() -> ToolRequest {
        ToolRequest::new(ToolKind::Formatter, "format")
            .with_arg("--check-shape")
            .with_stdin("fn main() {}")
            .with_working_dir(ToolWorkingDir::Isolated)
    }

    fn rustfmt_request(source: &str) -> ToolRequest {
        ToolRequest::new(ToolKind::Formatter, "rustfmt")
            .with_arg("--emit")
            .with_arg("stdout")
            .with_stdin(source)
    }

    #[test]
    fn fake_success_returns_captured_output_exactly() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::success("ok\n", "note\n")]);

        assert_eq!(
            runner.run(&request()),
            ToolResult::Output(ToolOutput {
                status: ToolStatus::Success,
                stdout: CapturedOutput::new("ok\n"),
                stderr: CapturedOutput::new("note\n"),
            })
        );
    }

    #[test]
    fn fake_nonzero_returns_status_and_captured_output_exactly() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::nonzero(2, "partial\n", "error\n")]);

        assert_eq!(
            runner.run(&request()),
            ToolResult::Output(ToolOutput {
                status: ToolStatus::Nonzero(2),
                stdout: CapturedOutput::new("partial\n"),
                stderr: CapturedOutput::new("error\n"),
            })
        );
    }

    #[test]
    fn fake_unavailable_failure_is_represented_exactly() {
        let expected = ToolFailure::Unavailable {
            tool: ToolKind::Compiler,
        };
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(expected.clone())]);

        assert_eq!(runner.run(&request()), ToolResult::Failure(expected));
    }

    #[test]
    fn fake_timeout_failure_is_represented_exactly() {
        let expected = ToolFailure::Timeout {
            tool: ToolKind::TestHarness,
        };
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(expected.clone())]);

        assert_eq!(runner.run(&request()), ToolResult::Failure(expected));
    }

    #[test]
    fn fake_invalid_invocation_failure_is_represented_exactly() {
        let expected = ToolFailure::InvalidInvocation {
            reason: "missing input".to_string(),
        };
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(expected.clone())]);

        assert_eq!(runner.run(&request()), ToolResult::Failure(expected));
    }

    #[test]
    fn fake_output_limit_failure_is_represented_exactly() {
        let expected = ToolFailure::OutputLimitExceeded {
            stream: ToolOutputStream::Stderr,
        };
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(expected.clone())]);

        assert_eq!(runner.run(&request()), ToolResult::Failure(expected));
    }

    #[test]
    fn fake_setup_failure_is_represented_exactly() {
        let expected = ToolFailure::SetupFailed {
            reason: "workspace unavailable".to_string(),
        };
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(expected.clone())]);

        assert_eq!(runner.run(&request()), ToolResult::Failure(expected));
    }

    #[test]
    fn fake_runner_consumes_outcomes_deterministically() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("first", ""),
            ToolResult::nonzero(1, "second", "err"),
        ]);

        assert_eq!(runner.run(&request()), ToolResult::success("first", ""));
        assert_eq!(
            runner.run(&request()),
            ToolResult::nonzero(1, "second", "err")
        );
        assert!(runner.is_empty());
    }

    #[test]
    fn fake_runner_records_requests_deterministically() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("first", ""),
            ToolResult::success("second", ""),
        ]);
        let first = ToolRequest::new(ToolKind::Formatter, "first").with_stdin("submitted");
        let second = ToolRequest::new(ToolKind::Formatter, "second").with_stdin("expected");

        let _ = runner.run(&first);
        let _ = runner.run(&second);

        assert_eq!(runner.requests(), &[first, second]);
    }

    #[test]
    fn missing_fake_outcome_still_records_request() {
        let mut runner = FakeToolRunner::new(Vec::new());
        let request = ToolRequest::new(ToolKind::Formatter, "missing").with_stdin("input");

        assert_eq!(
            runner.run(&request),
            ToolResult::Failure(ToolFailure::NoConfiguredOutcome)
        );
        assert_eq!(runner.requests(), &[request]);
    }

    #[test]
    fn fake_runner_reports_no_configured_outcome_deterministically() {
        let mut runner = FakeToolRunner::new(Vec::new());

        assert_eq!(
            runner.run(&request()),
            ToolResult::Failure(ToolFailure::NoConfiguredOutcome)
        );
    }

    #[test]
    fn captured_output_can_record_truncation_without_trimming_text() {
        assert_eq!(
            CapturedOutput::truncated("line 1\nline 2"),
            CapturedOutput {
                text: "line 1\nline 2".to_string(),
                truncated: true,
            }
        );
    }

    #[test]
    fn real_runner_uses_fixed_rustfmt_command_by_default() {
        let runner = RealToolRunner::new();

        assert_eq!(runner.formatter_command, "rustfmt");
    }

    #[test]
    fn real_runner_rejects_non_formatter_requests() {
        let mut runner = RealToolRunner::new();

        for tool in [ToolKind::Compiler, ToolKind::Linter, ToolKind::TestHarness] {
            assert!(matches!(
                runner.run(
                    &ToolRequest::new(tool, "not-rustfmt")
                        .with_arg("--emit")
                        .with_arg("stdout")
                        .with_stdin("fn main() {}")
                ),
                ToolResult::Failure(ToolFailure::InvalidInvocation { .. })
            ));
        }
    }

    #[test]
    fn real_runner_rejects_custom_working_directories() {
        let mut runner = RealToolRunner::new();

        assert!(matches!(
            runner.run(&rustfmt_request("fn main() {}").with_working_dir(ToolWorkingDir::Isolated)),
            ToolResult::Failure(ToolFailure::InvalidInvocation { .. })
        ));
    }

    #[test]
    fn real_runner_rejects_non_rustfmt_invocations() {
        let mut runner = RealToolRunner::new();

        assert!(matches!(
            runner.run(&ToolRequest::new(ToolKind::Formatter, "format").with_stdin("fn main() {}")),
            ToolResult::Failure(ToolFailure::InvalidInvocation { .. })
        ));
    }

    #[test]
    fn real_runner_reports_unavailable_fixed_formatter() {
        let mut runner =
            RealToolRunner::with_fixed_formatter_command_for_test("rust-reps-missing-rustfmt-tool");

        assert_eq!(
            runner.run(&rustfmt_request("fn main() {}")),
            ToolResult::Failure(ToolFailure::Unavailable {
                tool: ToolKind::Formatter
            })
        );
    }

    #[test]
    fn real_runner_formats_valid_rust_when_rustfmt_is_available() {
        let mut runner = RealToolRunner::new();

        match runner.run(&rustfmt_request("fn main(){ }")) {
            ToolResult::Failure(ToolFailure::Unavailable { .. }) => {}
            ToolResult::Output(output) => {
                assert_eq!(output.status, ToolStatus::Success);
                assert_eq!(output.stdout.text, "fn main() {}\n");
                assert!(!output.stdout.truncated);
                assert!(!output.stderr.truncated);
            }
            other => panic!("expected rustfmt output or unavailable, got {other:?}"),
        }
    }

    #[test]
    fn real_runner_classifies_invalid_rust_as_nonzero_when_rustfmt_is_available() {
        let mut runner = RealToolRunner::new();

        match runner.run(&rustfmt_request("let = ;")) {
            ToolResult::Failure(ToolFailure::Unavailable { .. }) => {}
            ToolResult::Output(output) => {
                assert!(matches!(output.status, ToolStatus::Nonzero(_)));
                assert!(!output.stderr.text.is_empty());
            }
            other => panic!("expected rustfmt nonzero output or unavailable, got {other:?}"),
        }
    }

    #[test]
    fn real_runner_classifies_output_limit() {
        let mut runner = RealToolRunner::with_output_limit_for_test(1);

        match runner.run(&rustfmt_request("fn main() {}")) {
            ToolResult::Failure(ToolFailure::Unavailable { .. }) => {}
            ToolResult::Failure(ToolFailure::OutputLimitExceeded {
                stream: ToolOutputStream::Stdout,
            }) => {}
            other => panic!("expected output-limit failure or unavailable, got {other:?}"),
        }
    }

    #[test]
    fn real_runner_classifies_timeout() {
        let mut runner = RealToolRunner::with_timeout_for_test(Duration::ZERO);

        match runner.run(&rustfmt_request("fn main() {}")) {
            ToolResult::Failure(ToolFailure::Unavailable { .. }) => {}
            ToolResult::Failure(ToolFailure::Timeout {
                tool: ToolKind::Formatter,
            }) => {}
            other => panic!("expected timeout failure or unavailable, got {other:?}"),
        }
    }

    #[test]
    fn tool_runner_source_does_not_expose_unapproved_execution() {
        let source = include_str!("tool_runner.rs");
        let forbidden = [
            format!("{}{}{}", "sh", " ", "-c"),
            format!("{}{}", "ba", "sh"),
            format!("{}{}", "car", "go"),
            format!("{}{}", "rust", "c"),
            format!("{}{}", "clip", "py"),
        ];

        for needle in forbidden {
            assert!(
                !source.contains(&needle),
                "tool runner must not contain {needle}"
            );
        }
    }

    #[test]
    fn runtime_product_modules_do_not_reference_fake_tool_runner() {
        let engine_source = include_str!("engine/mod.rs")
            .split("\n#[cfg(test)]\nmod tests")
            .next()
            .expect("production engine source must exist");
        let product_sources = [
            engine_source,
            include_str!("practice.rs"),
            include_str!("formatter.rs"),
            include_str!("lab.rs"),
            include_str!("cli.rs"),
            include_str!("client/mod.rs"),
            include_str!("tui/app.rs"),
        ];

        for source in product_sources {
            assert!(!source.contains("tool_runner"));
            assert!(!source.contains("RealToolRunner"));
        }
    }
}
