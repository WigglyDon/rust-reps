use crate::{answer, curriculum};

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
    UnsupportedValidator,
}

pub fn validate(input: ValidationInput<'_>, spec: ValidatorSpec) -> ValidationOutcome {
    match (spec.kind, spec.answer_rule) {
        (ValidatorKind::Exact, curriculum::AnswerRule::ExactTrimFinalNewline) => {
            if answer::check_answer(input.submitted, input.expected) {
                ValidationOutcome::Correct
            } else {
                ValidationOutcome::Wrong
            }
        }
        _ => ValidationOutcome::UnsupportedValidator,
    }
}

#[cfg(test)]
mod tests {
    use super::{ValidationInput, ValidationOutcome, ValidatorKind, ValidatorSpec, validate};
    use crate::curriculum::{AnswerRule, Validator};

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
    fn catalog_rustfmt_exact_rule_maps_to_inert_rustfmt_exact_spec() {
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
    fn rustfmt_exact_is_representable_but_not_executable_yet() {
        let outcome = validate(
            ValidationInput::new("fn main() {}", "fn main() {}"),
            ValidatorSpec::rustfmt_exact(),
        );

        assert_eq!(outcome, ValidationOutcome::UnsupportedValidator);
    }

    #[test]
    fn unsupported_rustfmt_exact_is_not_classified_as_wrong() {
        let outcome = validate(
            ValidationInput::new("wrong", "fn main() {}"),
            ValidatorSpec::rustfmt_exact(),
        );

        assert_ne!(outcome, ValidationOutcome::Wrong);
        assert_eq!(outcome, ValidationOutcome::UnsupportedValidator);
    }

    #[test]
    fn mismatched_answer_rule_fails_closed() {
        let spec = ValidatorSpec::from_catalog(Validator::Exact, AnswerRule::RustfmtExact);

        assert_eq!(spec.kind, ValidatorKind::Exact);
        assert_eq!(spec.answer_rule, AnswerRule::RustfmtExact);
        assert_eq!(
            validate(ValidationInput::new("fn main() {}", "fn main() {}"), spec),
            ValidationOutcome::UnsupportedValidator
        );
    }

    #[test]
    fn validators_module_stays_disconnected_from_tool_execution() {
        let source = include_str!("validators.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected production validator source");

        for forbidden in [
            format!("{}{}", "tool", "_runner"),
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
