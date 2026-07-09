#![allow(dead_code)]

use std::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use super::{
        CapturedOutput, FakeToolRunner, ToolFailure, ToolKind, ToolOutput, ToolOutputStream,
        ToolRequest, ToolResult, ToolStatus, ToolWorkingDir,
    };

    fn request() -> ToolRequest {
        ToolRequest::new(ToolKind::Formatter, "format")
            .with_arg("--check-shape")
            .with_stdin("fn main() {}")
            .with_working_dir(ToolWorkingDir::Isolated)
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
    fn fake_scaffold_source_does_not_contain_process_execution_calls() {
        let source = include_str!("tool_runner.rs");
        let forbidden = [
            format!("{}{}", "std::process::", "Command"),
            format!("{}{}", "Command", "::new"),
        ];

        for needle in forbidden {
            assert!(
                !source.contains(&needle),
                "fake scaffold must not contain {needle}"
            );
        }
    }

    #[test]
    fn runtime_product_modules_do_not_reference_fake_tool_runner() {
        let product_sources = [
            include_str!("engine/mod.rs"),
            include_str!("practice.rs"),
            include_str!("formatter.rs"),
            include_str!("lab.rs"),
            include_str!("cli.rs"),
            include_str!("client/mod.rs"),
            include_str!("tui/app.rs"),
        ];

        for source in product_sources {
            assert!(!source.contains("tool_runner"));
        }
    }
}
