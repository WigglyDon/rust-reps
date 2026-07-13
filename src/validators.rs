use crate::{answer, curriculum, tool_runner};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidatorKind {
    Exact,
    RustfmtExact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatorSpec {
    pub kind: ValidatorKind,
    pub answer_rule: curriculum::AnswerRule,
}

impl ValidatorSpec {
    pub const fn new(kind: ValidatorKind, answer_rule: curriculum::AnswerRule) -> Self {
        Self { kind, answer_rule }
    }

    pub const fn exact_trim_final_newline() -> Self {
        Self::new(
            ValidatorKind::Exact,
            curriculum::AnswerRule::ExactTrimFinalNewline,
        )
    }

    pub const fn rustfmt_exact() -> Self {
        Self::new(
            ValidatorKind::RustfmtExact,
            curriculum::AnswerRule::RustfmtExact,
        )
    }

    pub fn from_catalog(
        validator: curriculum::Validator,
        answer_rule: curriculum::AnswerRule,
    ) -> Self {
        match (validator, answer_rule) {
            (curriculum::Validator::Exact, curriculum::AnswerRule::ExactTrimFinalNewline) => {
                Self::exact_trim_final_newline()
            }
            (curriculum::Validator::RustfmtExact, curriculum::AnswerRule::RustfmtExact) => {
                Self::rustfmt_exact()
            }
            (curriculum::Validator::Exact, _) => Self::new(ValidatorKind::Exact, answer_rule),
            (curriculum::Validator::RustfmtExact, _) => {
                Self::new(ValidatorKind::RustfmtExact, answer_rule)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidationInput<'a> {
    pub submitted: &'a str,
    pub expected: &'a str,
}

impl<'a> ValidationInput<'a> {
    pub const fn new(submitted: &'a str, expected: &'a str) -> Self {
        Self {
            submitted,
            expected,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationOutcome {
    Correct,
    Wrong,
    Unavailable,
}

pub fn validate(input: ValidationInput<'_>, spec: ValidatorSpec) -> ValidationOutcome {
    let mut runner = tool_runner::RealToolRunner::new();
    validate_with_runner(input, spec, &mut runner)
}

pub(crate) fn validate_with_runner<R: tool_runner::ToolRunner>(
    input: ValidationInput<'_>,
    spec: ValidatorSpec,
    runner: &mut R,
) -> ValidationOutcome {
    match (spec.kind, spec.answer_rule) {
        (ValidatorKind::Exact, curriculum::AnswerRule::ExactTrimFinalNewline) => {
            if answer::check_answer(input.submitted, input.expected) {
                ValidationOutcome::Correct
            } else {
                ValidationOutcome::Wrong
            }
        }
        (ValidatorKind::RustfmtExact, curriculum::AnswerRule::RustfmtExact) => {
            match evaluate_rustfmt_exact_with_runner(input, runner) {
                RustfmtExactEvaluation::Correct => ValidationOutcome::Correct,
                RustfmtExactEvaluation::Wrong => ValidationOutcome::Wrong,
                RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolNonzero {
                    phase: RustfmtExactPhase::Submitted,
                    ..
                }) => ValidationOutcome::Wrong,
                RustfmtExactEvaluation::Failure(_) => ValidationOutcome::Unavailable,
            }
        }
        _ => ValidationOutcome::Unavailable,
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustfmtExactPhase {
    Submitted,
    Expected,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustfmtExactFailure {
    ToolUnavailable {
        phase: RustfmtExactPhase,
        tool: tool_runner::ToolKind,
    },
    ToolTimeout {
        phase: RustfmtExactPhase,
        tool: tool_runner::ToolKind,
    },
    ToolNonzero {
        phase: RustfmtExactPhase,
        status: i32,
        stderr: tool_runner::CapturedOutput,
    },
    ToolOutputLimitExceeded {
        phase: RustfmtExactPhase,
        stream: tool_runner::ToolOutputStream,
    },
    InvalidInvocation {
        phase: RustfmtExactPhase,
        reason: String,
    },
    SetupFailed {
        phase: RustfmtExactPhase,
        reason: String,
    },
    NoConfiguredOutcome {
        phase: RustfmtExactPhase,
    },
    InvalidExerciseConfiguration {
        phase: RustfmtExactPhase,
        reason: String,
    },
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustfmtExactEvaluation {
    Correct,
    Wrong,
    Failure(RustfmtExactFailure),
}

#[allow(dead_code)]
pub fn evaluate_rustfmt_exact_with_runner<R: tool_runner::ToolRunner>(
    input: ValidationInput<'_>,
    runner: &mut R,
) -> RustfmtExactEvaluation {
    let submitted =
        match format_rustfmt_exact_phase(RustfmtExactPhase::Submitted, input.submitted, runner) {
            Ok(formatted) => formatted,
            Err(failure) => return RustfmtExactEvaluation::Failure(failure),
        };
    let expected =
        match format_rustfmt_exact_phase(RustfmtExactPhase::Expected, input.expected, runner) {
            Ok(formatted) => formatted,
            Err(failure) => return RustfmtExactEvaluation::Failure(failure),
        };

    if answer::check_answer(&submitted, answer::trim_line_ending(&expected)) {
        RustfmtExactEvaluation::Correct
    } else {
        RustfmtExactEvaluation::Wrong
    }
}

#[allow(dead_code)]
pub fn evaluate_rustfmt_exact_with_fake_runner(
    input: ValidationInput<'_>,
    runner: &mut tool_runner::FakeToolRunner,
) -> RustfmtExactEvaluation {
    evaluate_rustfmt_exact_with_runner(input, runner)
}

#[allow(dead_code)]
fn format_rustfmt_exact_phase<R: tool_runner::ToolRunner>(
    phase: RustfmtExactPhase,
    source: &str,
    runner: &mut R,
) -> Result<String, RustfmtExactFailure> {
    let request = rustfmt_exact_request(phase, source);
    match runner.run(&request) {
        tool_runner::ToolResult::Output(output) => match output.status {
            tool_runner::ToolStatus::Success => {
                if let Some(failure) = output_limit_failure(phase, &output) {
                    return Err(failure);
                }
                Ok(output.stdout.text)
            }
            tool_runner::ToolStatus::Nonzero(status) => {
                if let Some(failure) = output_limit_failure(phase, &output) {
                    return Err(failure);
                }
                Err(RustfmtExactFailure::ToolNonzero {
                    phase,
                    status,
                    stderr: output.stderr,
                })
            }
        },
        tool_runner::ToolResult::Failure(failure) => Err(map_tool_failure(phase, failure)),
    }
}

#[allow(dead_code)]
fn output_limit_failure(
    phase: RustfmtExactPhase,
    output: &tool_runner::ToolOutput,
) -> Option<RustfmtExactFailure> {
    if output.stdout.truncated {
        return Some(RustfmtExactFailure::ToolOutputLimitExceeded {
            phase,
            stream: tool_runner::ToolOutputStream::Stdout,
        });
    }
    if output.stderr.truncated {
        return Some(RustfmtExactFailure::ToolOutputLimitExceeded {
            phase,
            stream: tool_runner::ToolOutputStream::Stderr,
        });
    }
    None
}

#[allow(dead_code)]
fn rustfmt_exact_request(phase: RustfmtExactPhase, source: &str) -> tool_runner::ToolRequest {
    let label = match phase {
        RustfmtExactPhase::Submitted => "rustfmt-exact-submitted",
        RustfmtExactPhase::Expected => "rustfmt-exact-expected",
    };
    tool_runner::ToolRequest::new(tool_runner::ToolKind::Formatter, label)
        .with_arg("--emit")
        .with_arg("stdout")
        .with_stdin(source)
        .with_working_dir(tool_runner::ToolWorkingDir::None)
}

#[allow(dead_code)]
fn map_tool_failure(
    phase: RustfmtExactPhase,
    failure: tool_runner::ToolFailure,
) -> RustfmtExactFailure {
    match failure {
        tool_runner::ToolFailure::Unavailable { tool } => {
            RustfmtExactFailure::ToolUnavailable { phase, tool }
        }
        tool_runner::ToolFailure::Timeout { tool } => {
            RustfmtExactFailure::ToolTimeout { phase, tool }
        }
        tool_runner::ToolFailure::InvalidInvocation { reason } => {
            RustfmtExactFailure::InvalidInvocation { phase, reason }
        }
        tool_runner::ToolFailure::OutputLimitExceeded { stream } => {
            RustfmtExactFailure::ToolOutputLimitExceeded { phase, stream }
        }
        tool_runner::ToolFailure::SetupFailed { reason } => {
            RustfmtExactFailure::SetupFailed { phase, reason }
        }
        tool_runner::ToolFailure::NoConfiguredOutcome => {
            RustfmtExactFailure::NoConfiguredOutcome { phase }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RustfmtExactEvaluation, RustfmtExactFailure, RustfmtExactPhase, ValidationInput,
        ValidationOutcome, ValidatorKind, ValidatorSpec, evaluate_rustfmt_exact_with_fake_runner,
        evaluate_rustfmt_exact_with_runner, validate, validate_with_runner,
    };
    use crate::curriculum::{AnswerRule, Validator};
    use crate::tool_runner::{
        CapturedOutput, FakeToolRunner, RealToolRunner, ToolFailure, ToolKind, ToolOutput,
        ToolOutputStream, ToolRequest, ToolResult, ToolStatus,
    };

    fn validate_exact(submitted: &str, expected: &str) -> ValidationOutcome {
        validate(
            ValidationInput::new(submitted, expected),
            ValidatorSpec::exact_trim_final_newline(),
        )
    }

    #[test]
    fn catalog_exact_rule_maps_to_exact_validator_spec() {
        let spec = ValidatorSpec::from_catalog(Validator::Exact, AnswerRule::ExactTrimFinalNewline);

        assert_eq!(spec.kind, ValidatorKind::Exact);
        assert_eq!(spec.answer_rule, AnswerRule::ExactTrimFinalNewline);
    }

    #[test]
    fn catalog_rustfmt_exact_rule_maps_to_rustfmt_exact_spec() {
        let spec = ValidatorSpec::from_catalog(Validator::RustfmtExact, AnswerRule::RustfmtExact);

        assert_eq!(spec.kind, ValidatorKind::RustfmtExact);
        assert_eq!(spec.answer_rule, AnswerRule::RustfmtExact);
    }

    #[test]
    fn exact_validator_accepts_current_exact_answer() {
        assert_eq!(
            validate_exact("fn main() {}", "fn main() {}"),
            ValidationOutcome::Correct
        );
    }

    #[test]
    fn exact_validator_trims_one_lf_or_crlf_only() {
        assert_eq!(
            validate_exact("fn main() {}\n", "fn main() {}"),
            ValidationOutcome::Correct
        );
        assert_eq!(
            validate_exact("fn main() {}\r\n", "fn main() {}"),
            ValidationOutcome::Correct
        );
        assert_eq!(
            validate_exact("fn main() {}\n\n", "fn main() {}"),
            ValidationOutcome::Wrong
        );
    }

    #[test]
    fn exact_validator_does_not_normalize_whitespace_or_case() {
        assert_eq!(
            validate_exact(" fn main() {}", "fn main() {}"),
            ValidationOutcome::Wrong
        );
        assert_eq!(
            validate_exact("fn main() {} ", "fn main() {}"),
            ValidationOutcome::Wrong
        );
        assert_eq!(
            validate_exact("fn\tmain() {}", "fn main() {}"),
            ValidationOutcome::Wrong
        );
        assert_eq!(
            validate_exact("Fn Main() {}", "fn main() {}"),
            ValidationOutcome::Wrong
        );
    }

    #[test]
    fn exact_validator_does_not_normalize_internal_newlines_or_blank_lines() {
        assert_eq!(
            validate_exact("fn main() {}\nprintln!(\"Rust Reps\");", "fn main() {}"),
            ValidationOutcome::Wrong
        );
        assert_eq!(
            validate_exact("\n", "fn main() {}"),
            ValidationOutcome::Wrong
        );
    }

    #[test]
    fn exact_validator_does_not_normalize_unicode_or_punctuation() {
        assert_eq!(
            validate_exact("println!(\"Ferris\");", "println!(\"Ferris\");"),
            ValidationOutcome::Correct
        );
        assert_eq!(
            validate_exact("println!(\"Ferrís\");", "println!(\"Ferris\");"),
            ValidationOutcome::Wrong
        );
        assert_eq!(
            validate_exact("println!(\"Ferris\")", "println!(\"Ferris\");"),
            ValidationOutcome::Wrong
        );
    }

    #[test]
    fn wrong_answer_is_a_validation_outcome_not_an_error() {
        assert_eq!(
            validate_exact("wrong", "fn main() {}"),
            ValidationOutcome::Wrong
        );
    }

    #[test]
    fn rustfmt_exact_product_dispatch_accepts_equivalent_fake_outputs() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n", ""),
            ToolResult::success("fn main() {}\n", ""),
        ]);
        assert_eq!(
            validate_with_runner(
                ValidationInput::new("fn main(){ }", "fn main() {}"),
                ValidatorSpec::rustfmt_exact(),
                &mut runner,
            ),
            ValidationOutcome::Correct
        );
    }

    #[test]
    fn rustfmt_unavailability_is_not_classified_as_wrong() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(ToolFailure::Unavailable {
            tool: ToolKind::Formatter,
        })]);
        assert_eq!(
            validate_with_runner(
                ValidationInput::new("fn main() {}", "fn main() {}"),
                ValidatorSpec::rustfmt_exact(),
                &mut runner,
            ),
            ValidationOutcome::Unavailable
        );
    }

    #[test]
    fn exact_dispatch_never_invokes_the_formatter_runner() {
        let mut runner = FakeToolRunner::new(Vec::new());

        assert_eq!(
            validate_with_runner(
                ValidationInput::new("5\r\n", "5"),
                ValidatorSpec::exact_trim_final_newline(),
                &mut runner,
            ),
            ValidationOutcome::Correct
        );
        assert!(runner.requests().is_empty());
    }

    #[test]
    fn mismatched_answer_rule_fails_closed() {
        let spec = ValidatorSpec::from_catalog(Validator::Exact, AnswerRule::RustfmtExact);

        assert_eq!(spec.kind, ValidatorKind::Exact);
        assert_eq!(spec.answer_rule, AnswerRule::RustfmtExact);
        assert_eq!(
            validate(ValidationInput::new("fn main() {}", "fn main() {}"), spec),
            ValidationOutcome::Unavailable
        );
    }

    #[test]
    fn fake_rustfmt_exact_success_with_matching_outputs_returns_correct() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n", "submitted note\n"),
            ToolResult::success("fn main() {}\n", "expected note\n"),
        ]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main(){ }\n", "fn main() {}"),
            &mut runner,
        );

        assert_eq!(outcome, RustfmtExactEvaluation::Correct);
    }

    #[test]
    fn fake_rustfmt_exact_success_with_different_outputs_returns_wrong() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n", ""),
            ToolResult::success("fn other() {}\n", ""),
        ]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn other() {}"),
            &mut runner,
        );

        assert_eq!(outcome, RustfmtExactEvaluation::Wrong);
    }

    #[test]
    fn fake_rustfmt_exact_uses_one_final_line_ending_trim_law_for_formatted_outputs() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n\n", ""),
            ToolResult::success("fn main() {}\n", ""),
        ]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_eq!(outcome, RustfmtExactEvaluation::Wrong);
    }

    #[test]
    fn fake_rustfmt_exact_requests_submitted_then_expected_with_stdin() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n", ""),
            ToolResult::success("fn main() {}\n", ""),
        ]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("submitted source", "expected source"),
            &mut runner,
        );

        assert_eq!(outcome, RustfmtExactEvaluation::Correct);
        assert_eq!(
            runner.requests(),
            &[
                ToolRequest::new(ToolKind::Formatter, "rustfmt-exact-submitted")
                    .with_arg("--emit")
                    .with_arg("stdout")
                    .with_stdin("submitted source"),
                ToolRequest::new(ToolKind::Formatter, "rustfmt-exact-expected")
                    .with_arg("--emit")
                    .with_arg("stdout")
                    .with_stdin("expected source"),
            ]
        );
    }

    #[test]
    fn submitted_fake_nonzero_maps_to_non_wrong_failure() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::nonzero(
            2,
            "partial\n",
            "submitted parse error\n",
        )]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("bad submitted", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolNonzero {
                phase: RustfmtExactPhase::Submitted,
                status: 2,
                stderr: CapturedOutput::new("submitted parse error\n"),
            })
        );
    }

    #[test]
    fn expected_fake_nonzero_maps_to_configuration_failure_not_wrong() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n", ""),
            ToolResult::nonzero(2, "partial\n", "expected parse error\n"),
        ]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "bad expected"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolNonzero {
                phase: RustfmtExactPhase::Expected,
                status: 2,
                stderr: CapturedOutput::new("expected parse error\n"),
            })
        );
    }

    #[test]
    fn fake_unavailable_maps_to_non_wrong_failure() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(ToolFailure::Unavailable {
            tool: ToolKind::Formatter,
        })]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolUnavailable {
                phase: RustfmtExactPhase::Submitted,
                tool: ToolKind::Formatter,
            })
        );
    }

    #[test]
    fn fake_timeout_maps_to_non_wrong_failure() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(ToolFailure::Timeout {
            tool: ToolKind::Formatter,
        })]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolTimeout {
                phase: RustfmtExactPhase::Submitted,
                tool: ToolKind::Formatter,
            })
        );
    }

    #[test]
    fn fake_invalid_invocation_maps_to_non_wrong_failure() {
        let mut runner =
            FakeToolRunner::new(vec![ToolResult::Failure(ToolFailure::InvalidInvocation {
                reason: "missing stdin".to_string(),
            })]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::InvalidInvocation {
                phase: RustfmtExactPhase::Submitted,
                reason: "missing stdin".to_string(),
            })
        );
    }

    #[test]
    fn fake_output_limit_maps_to_non_wrong_failure() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(
            ToolFailure::OutputLimitExceeded {
                stream: ToolOutputStream::Stdout,
            },
        )]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolOutputLimitExceeded {
                phase: RustfmtExactPhase::Submitted,
                stream: ToolOutputStream::Stdout,
            })
        );
    }

    #[test]
    fn truncated_fake_output_maps_to_output_limit_failure() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::Output(ToolOutput {
            status: ToolStatus::Success,
            stdout: CapturedOutput::truncated("fn main() {}\n"),
            stderr: CapturedOutput::new(""),
        })]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolOutputLimitExceeded {
                phase: RustfmtExactPhase::Submitted,
                stream: ToolOutputStream::Stdout,
            })
        );
    }

    #[test]
    fn fake_setup_failure_maps_to_non_wrong_failure() {
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(ToolFailure::SetupFailed {
            reason: "scratch unavailable".to_string(),
        })]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::SetupFailed {
                phase: RustfmtExactPhase::Submitted,
                reason: "scratch unavailable".to_string(),
            })
        );
    }

    #[test]
    fn missing_fake_outcome_maps_to_non_wrong_failure() {
        let mut runner = FakeToolRunner::new(Vec::new());

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_ne!(outcome, RustfmtExactEvaluation::Wrong);
        assert_eq!(
            outcome,
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::NoConfiguredOutcome {
                phase: RustfmtExactPhase::Submitted,
            })
        );
    }

    #[test]
    fn success_stderr_is_ignored_by_scaffold_and_not_returned_as_product_output() {
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn main() {}\n", "warning on submitted\n"),
            ToolResult::success("fn main() {}\n", "warning on expected\n"),
        ]);

        let outcome = evaluate_rustfmt_exact_with_fake_runner(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            &mut runner,
        );

        assert_eq!(outcome, RustfmtExactEvaluation::Correct);
    }

    #[test]
    fn real_rustfmt_exact_accepts_differently_formatted_valid_source_when_available() {
        let mut runner = RealToolRunner::new();

        let outcome = evaluate_rustfmt_exact_with_runner(
            ValidationInput::new("fn main(){ }", "fn main() {}"),
            &mut runner,
        );

        match outcome {
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolUnavailable { .. }) => {}
            other => assert_eq!(other, RustfmtExactEvaluation::Correct),
        }
    }

    #[test]
    fn real_rustfmt_exact_rejects_different_valid_source_when_available() {
        let mut runner = RealToolRunner::new();

        let outcome = evaluate_rustfmt_exact_with_runner(
            ValidationInput::new("fn main() {}", "fn other() {}"),
            &mut runner,
        );

        match outcome {
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolUnavailable { .. }) => {}
            other => assert_eq!(other, RustfmtExactEvaluation::Wrong),
        }
    }

    #[test]
    fn real_rustfmt_exact_invalid_submitted_source_is_non_wrong_failure() {
        let mut runner = RealToolRunner::new();

        let outcome = evaluate_rustfmt_exact_with_runner(
            ValidationInput::new("let = ;", "fn main() {}"),
            &mut runner,
        );

        match outcome {
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolUnavailable { .. }) => {}
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolNonzero {
                phase: RustfmtExactPhase::Submitted,
                ..
            }) => {}
            other => panic!("expected submitted non-wrong failure or unavailable, got {other:?}"),
        }
    }

    #[test]
    fn real_rustfmt_exact_invalid_expected_source_is_configuration_failure() {
        let mut runner = RealToolRunner::new();

        let outcome = evaluate_rustfmt_exact_with_runner(
            ValidationInput::new("fn main() {}", "let = ;"),
            &mut runner,
        );

        match outcome {
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolUnavailable { .. }) => {}
            RustfmtExactEvaluation::Failure(RustfmtExactFailure::ToolNonzero {
                phase: RustfmtExactPhase::Expected,
                ..
            }) => {}
            other => panic!("expected expected-side failure or unavailable, got {other:?}"),
        }
    }

    #[test]
    fn product_dispatch_maps_submitted_parse_failure_to_wrong() {
        let mut runner =
            FakeToolRunner::new(vec![ToolResult::nonzero(1, "", "submitted parse error\n")]);
        assert_eq!(
            validate_with_runner(
                ValidationInput::new("let = ;", "fn main() {}"),
                ValidatorSpec::rustfmt_exact(),
                &mut runner,
            ),
            ValidationOutcome::Wrong
        );
    }

    #[test]
    fn client_formatter_and_adapters_do_not_call_answer_validator_internals() {
        let product_sources = [
            include_str!("practice.rs"),
            include_str!("cli.rs"),
            include_str!("lab.rs"),
            include_str!("formatter.rs"),
            include_str!("client/mod.rs"),
            include_str!("tui/app.rs"),
        ];

        for source in product_sources {
            assert!(!source.contains("evaluate_rustfmt_exact_with_fake_runner"));
            assert!(!source.contains("evaluate_rustfmt_exact_with_runner"));
            assert!(!source.contains("RealToolRunner"));
        }
    }

    #[test]
    fn validators_use_only_the_fixed_tool_runner_boundary() {
        let source = include_str!("validators.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected production validator source");

        assert!(source.contains("RealToolRunner::new"));
        for forbidden in [
            format!("{}{}", "format", "ter"),
            format!("{}{}", "std::", "process"),
            format!("{}{}{}", "Com", "mand", "::new"),
            format!("{}{}", "car", "go"),
            format!("{}{}", "rust", "c"),
            format!("{}{}", "clip", "py"),
        ] {
            assert!(
                !source.contains(&forbidden),
                "validators production source must not reference {forbidden}"
            );
        }
    }
}
