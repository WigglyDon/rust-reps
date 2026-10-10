use std::borrow::Cow;

use crate::{curriculum, lab, validators};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseKind {
    TypeExact,
    RustCode,
}

impl ExerciseKind {
    pub(crate) fn from_task_type(task_type: curriculum::TaskType) -> Self {
        match task_type {
            curriculum::TaskType::TypeExact => Self::TypeExact,
            curriculum::TaskType::RustCode => Self::RustCode,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseValidator {
    Exact,
    RustfmtExact,
}

impl ExerciseValidator {
    pub(crate) fn from_validator(validator: curriculum::Validator) -> Self {
        match validator {
            curriculum::Validator::Exact => Self::Exact,
            curriculum::Validator::RustfmtExact => Self::RustfmtExact,
        }
    }

    fn validator(self) -> curriculum::Validator {
        match self {
            Self::Exact => curriculum::Validator::Exact,
            Self::RustfmtExact => curriculum::Validator::RustfmtExact,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseAnswerRule {
    ExactTrimFinalNewline,
    RustfmtExact,
}

impl ExerciseAnswerRule {
    pub(crate) fn from_answer_rule(answer_rule: curriculum::AnswerRule) -> Self {
        match answer_rule {
            curriculum::AnswerRule::ExactTrimFinalNewline => Self::ExactTrimFinalNewline,
            curriculum::AnswerRule::RustfmtExact => Self::RustfmtExact,
        }
    }

    fn answer_rule(self) -> curriculum::AnswerRule {
        match self {
            Self::ExactTrimFinalNewline => curriculum::AnswerRule::ExactTrimFinalNewline,
            Self::RustfmtExact => curriculum::AnswerRule::RustfmtExact,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FormatCapability {
    Disabled,
    Rustfmt,
}

impl FormatCapability {
    pub(crate) fn is_available(self) -> bool {
        matches!(self, Self::Rustfmt)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ExerciseSupportStage {
    Introduce,
    Guided,
    Practice,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ExerciseFocusRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ExerciseSupport {
    pub(crate) stage: ExerciseSupportStage,
    pub(crate) action_header: Cow<'static, str>,
    pub(crate) catalog_label: Cow<'static, str>,
    pub(crate) beginner_brief: Vec<Cow<'static, str>>,
    pub(crate) explanation: Cow<'static, str>,
    pub(crate) worked_example: Option<Cow<'static, str>>,
    pub(crate) focus_ranges: Vec<ExerciseFocusRange>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExercisePacket {
    pub id: Cow<'static, str>,
    pub position: usize,
    pub total: usize,
    pub instruction: Cow<'static, str>,
    pub stimulus: Cow<'static, str>,
    pub(crate) support: Option<ExerciseSupport>,
    expected_answer: Cow<'static, str>,
    pub(crate) format_capability: FormatCapability,
    pub kind: ExerciseKind,
    pub validator: ExerciseValidator,
    pub answer_rule: ExerciseAnswerRule,
}

impl ExercisePacket {
    #[expect(
        clippy::too_many_arguments,
        reason = "packet construction keeps private answers and validation separate from visible task fields"
    )]
    pub(crate) fn new(
        id: Cow<'static, str>,
        position: usize,
        total: usize,
        instruction: Cow<'static, str>,
        stimulus: Cow<'static, str>,
        expected_answer: Cow<'static, str>,
        format_capability: FormatCapability,
        kind: ExerciseKind,
        validator: ExerciseValidator,
        answer_rule: ExerciseAnswerRule,
    ) -> Self {
        Self {
            id,
            position,
            total,
            instruction,
            stimulus,
            support: None,
            expected_answer,
            format_capability,
            kind,
            validator,
            answer_rule,
        }
    }

    pub(crate) fn with_support(mut self, support: ExerciseSupport) -> Self {
        self.support = Some(support);
        self
    }

    fn validator_spec(&self) -> validators::ValidatorSpec {
        validators::ValidatorSpec::from_catalog(
            self.validator.validator(),
            self.answer_rule.answer_rule(),
        )
    }

    #[cfg(test)]
    pub(crate) fn expected_answer(&self) -> &str {
        self.expected_answer.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitResult {
    Correct,
    TryAgain,
    ValidationUnavailable,
    NoActiveDrill,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipResult {
    Skipped,
    NoActiveDrill,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionStats {
    pub passed: usize,
    pub skipped: usize,
    pub attempts: usize,
    pub total_drills: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionSnapshot {
    pub exercise: Option<ExercisePacket>,
    pub stats: SessionStats,
    pub is_complete: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ExerciseSource {
    Product(Vec<ExercisePacket>),
    LoopingProduct {
        looping_exercises: Vec<ExercisePacket>,
        catalog_exercises: Vec<ExercisePacket>,
        round_size: usize,
    },
    Lab {
        product_exercises: Vec<ExercisePacket>,
    },
}

impl ExerciseSource {
    fn len(&self) -> usize {
        match self {
            Self::Product(exercises) => exercises.len(),
            Self::LoopingProduct { round_size, .. } => *round_size,
            Self::Lab { .. } => usize::MAX,
        }
    }

    fn get(&self, index: usize) -> Option<ExercisePacket> {
        match self {
            Self::Product(exercises) => exercises.get(index).cloned(),
            Self::LoopingProduct {
                looping_exercises, ..
            } => looping_exercises
                .get(index % looping_exercises.len())
                .cloned(),
            Self::Lab { .. } => Some(lab::exercise_packet_at(index)),
        }
    }

    fn product_exercises(&self) -> &[ExercisePacket] {
        match self {
            Self::Product(exercises) => exercises,
            Self::LoopingProduct {
                catalog_exercises, ..
            } => catalog_exercises,
            Self::Lab { product_exercises } => product_exercises,
        }
    }

    fn is_endless(&self) -> bool {
        matches!(self, Self::LoopingProduct { .. } | Self::Lab { .. })
    }

    fn round_position(&self, index: usize) -> Option<(usize, usize)> {
        match self {
            Self::LoopingProduct { round_size, .. } => Some((index % *round_size + 1, *round_size)),
            Self::Product(_) | Self::Lab { .. } => None,
        }
    }
}

pub struct Session {
    source: ExerciseSource,
    index: usize,
    passed: usize,
    skipped: usize,
    attempts: usize,
    wrong_answer_feedback: Option<Cow<'static, str>>,
}

impl Session {
    pub(crate) fn from_exercises(exercises: Vec<ExercisePacket>) -> Self {
        Self {
            source: ExerciseSource::Product(exercises),
            index: 0,
            passed: 0,
            skipped: 0,
            attempts: 0,
            wrong_answer_feedback: None,
        }
    }

    pub(crate) fn lab_with_product_exercises(product_exercises: Vec<ExercisePacket>) -> Self {
        Self {
            source: ExerciseSource::Lab { product_exercises },
            index: 0,
            passed: 0,
            skipped: 0,
            attempts: 0,
            wrong_answer_feedback: None,
        }
    }

    pub(crate) fn looping_from_exercises(
        looping_exercises: Vec<ExercisePacket>,
        catalog_exercises: Vec<ExercisePacket>,
        round_size: usize,
    ) -> Self {
        assert!(round_size > 0, "looping round must contain exercises");
        assert!(
            !looping_exercises.is_empty() && looping_exercises.len().is_multiple_of(round_size),
            "looping exercises must contain complete rounds"
        );
        Self {
            source: ExerciseSource::LoopingProduct {
                looping_exercises,
                catalog_exercises,
                round_size,
            },
            index: 0,
            passed: 0,
            skipped: 0,
            attempts: 0,
            wrong_answer_feedback: None,
        }
    }

    #[cfg(test)]
    pub fn new() -> Self {
        Self::from_exercises(
            crate::orchestration::current_product_exercise_packets()
                .expect("test product exercises must resolve"),
        )
    }

    #[cfg(test)]
    pub fn lab() -> Self {
        Self::lab_with_product_exercises(
            crate::orchestration::current_product_exercise_packets()
                .expect("test product exercises must resolve"),
        )
    }

    #[cfg(test)]
    pub fn lab_at(index: usize) -> Self {
        Self {
            source: ExerciseSource::Lab {
                product_exercises: crate::orchestration::current_product_exercise_packets()
                    .expect("test product exercises must resolve"),
            },
            index,
            passed: 0,
            skipped: 0,
            attempts: 0,
            wrong_answer_feedback: None,
        }
    }

    pub fn total_drills(&self) -> usize {
        self.source.len()
    }

    pub fn passed(&self) -> usize {
        self.passed
    }

    pub fn skipped(&self) -> usize {
        self.skipped
    }

    pub fn attempts(&self) -> usize {
        self.attempts
    }

    pub fn product_exercises(&self) -> &[ExercisePacket] {
        self.source.product_exercises()
    }

    pub fn wrong_answer_feedback(&self) -> Option<&str> {
        self.wrong_answer_feedback.as_deref()
    }

    pub fn is_complete(&self) -> bool {
        !self.source.is_endless() && self.index >= self.total_drills()
    }

    pub fn round_position(&self) -> Option<(usize, usize)> {
        self.source.round_position(self.index)
    }

    pub fn current_exercise(&self) -> Option<ExercisePacket> {
        self.source.get(self.index)
    }

    pub fn first_exercise_packet(&self) -> ExercisePacket {
        self.source
            .get(0)
            .expect("session source contains at least one exercise")
    }

    pub fn stats(&self) -> SessionStats {
        SessionStats {
            passed: self.passed,
            skipped: self.skipped,
            attempts: self.attempts,
            total_drills: self.total_drills(),
        }
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        SessionSnapshot {
            exercise: self.current_exercise(),
            stats: self.stats(),
            is_complete: self.is_complete(),
        }
    }

    pub fn submit_answer(&mut self, input: &str) -> SubmitResult {
        let Some(exercise) = self.current_exercise() else {
            return SubmitResult::NoActiveDrill;
        };
        self.wrong_answer_feedback = None;
        let outcome = validators::validate(
            validators::ValidationInput::new(input, exercise.expected_answer.as_ref()),
            exercise.validator_spec(),
        );
        self.apply_validation_outcome(exercise, outcome)
    }

    fn apply_validation_outcome(
        &mut self,
        exercise: ExercisePacket,
        outcome: validators::ValidationOutcome,
    ) -> SubmitResult {
        match outcome {
            validators::ValidationOutcome::Correct => {
                self.attempts += 1;
                self.passed += 1;
                self.advance();
                SubmitResult::Correct
            }
            validators::ValidationOutcome::Wrong => {
                self.attempts += 1;
                self.wrong_answer_feedback = Some(exercise.expected_answer.clone());
                SubmitResult::TryAgain
            }
            validators::ValidationOutcome::Unavailable => SubmitResult::ValidationUnavailable,
        }
    }

    pub fn skip(&mut self) -> SkipResult {
        if self.current_exercise().is_none() {
            return SkipResult::NoActiveDrill;
        }

        self.wrong_answer_feedback = None;
        self.skipped += 1;
        self.advance();
        SkipResult::Skipped
    }

    fn advance(&mut self) {
        self.index = self.index.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExerciseAnswerRule, ExerciseKind, ExerciseValidator, FormatCapability, Session, SkipResult,
        SubmitResult,
    };
    use crate::tool_runner::{FakeToolRunner, ToolFailure, ToolKind, ToolResult};
    use crate::{curriculum, lab, orchestration, validators};

    impl Session {
        fn submit_answer_with_runner<R: crate::tool_runner::ToolRunner>(
            &mut self,
            input: &str,
            runner: &mut R,
        ) -> SubmitResult {
            let Some(exercise) = self.current_exercise() else {
                return SubmitResult::NoActiveDrill;
            };
            self.wrong_answer_feedback = None;
            let outcome = validators::validate_with_runner(
                validators::ValidationInput::new(input, exercise.expected_answer.as_ref()),
                exercise.validator_spec(),
                runner,
            );
            self.apply_validation_outcome(exercise, outcome)
        }
    }

    fn one_task_session(id: &str) -> Session {
        let exercise = orchestration::current_product_exercise_packets()
            .expect("product packets must resolve")
            .into_iter()
            .find(|exercise| exercise.id == id)
            .expect("requested product exercise must exist");
        Session::from_exercises(vec![exercise])
    }

    #[test]
    fn new_session_starts_at_exercise_001() {
        let session = Session::new();

        assert_eq!(
            session
                .current_exercise()
                .expect("expected first exercise")
                .id,
            "001"
        );
        assert_eq!(session.passed(), 0);
        assert_eq!(session.skipped(), 0);
        assert_eq!(session.attempts(), 0);
        assert!(!session.is_complete());
    }

    #[test]
    fn lab_session_starts_from_generated_packet() {
        let session = Session::lab();
        let exercise = session
            .current_exercise()
            .expect("expected first lab exercise");

        assert_eq!(exercise.id, "LAB-000001");
        assert_eq!(exercise.format_capability, FormatCapability::Rustfmt);
        assert_eq!(session.total_drills(), usize::MAX);
        assert_eq!(session.stats().total_drills, usize::MAX);
        assert!(!session.is_complete());
    }

    #[test]
    fn lab_multiline_answer_uses_exact_answer_rules() {
        let mut session = Session::lab();
        for _ in 0..lab::MULTILINE_SOLUTION_FAMILY_INDEX {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }

        assert_eq!(
            session.submit_answer(lab::MULTILINE_SOLUTION_EXPECTED),
            SubmitResult::Correct
        );
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 1);
    }

    #[test]
    fn lab_multiline_answer_allows_one_final_line_ending_only() {
        let mut one_final_lf = Session::lab();
        for _ in 0..lab::MULTILINE_SOLUTION_FAMILY_INDEX {
            assert_eq!(one_final_lf.skip(), SkipResult::Skipped);
        }
        assert_eq!(
            one_final_lf.submit_answer(&format!("{}\n", lab::MULTILINE_SOLUTION_EXPECTED)),
            SubmitResult::Correct
        );

        let mut extra_blank_line = Session::lab();
        for _ in 0..lab::MULTILINE_SOLUTION_FAMILY_INDEX {
            assert_eq!(extra_blank_line.skip(), SkipResult::Skipped);
        }
        assert_eq!(
            extra_blank_line.submit_answer(&format!("{}\n\n", lab::MULTILINE_SOLUTION_EXPECTED)),
            SubmitResult::TryAgain
        );
        assert_eq!(
            extra_blank_line
                .current_exercise()
                .expect("expected lab multiline exercise")
                .id,
            "LAB-000005"
        );
    }

    #[test]
    fn lab_correct_answer_advances_without_completing() {
        let mut session = Session::lab();
        let first = session
            .current_exercise()
            .expect("expected first lab exercise");

        assert_eq!(
            session.submit_answer(first.expected_answer.as_ref()),
            SubmitResult::Correct
        );

        assert_eq!(session.passed(), 1);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected second lab exercise")
                .id,
            "LAB-000002"
        );
        assert!(!session.is_complete());
    }

    #[test]
    fn lab_skip_advances_without_completing() {
        let mut session = Session::lab();

        assert_eq!(session.skip(), SkipResult::Skipped);

        assert_eq!(session.skipped(), 1);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected second lab exercise")
                .id,
            "LAB-000002"
        );
        assert!(!session.is_complete());
    }

    #[test]
    fn lab_advances_past_static_fixture_count_without_ending() {
        let mut session = Session::lab();

        for _ in 0..20 {
            let exercise = session.current_exercise().expect("expected lab exercise");
            assert_eq!(
                session.submit_answer(exercise.expected_answer.as_ref()),
                SubmitResult::Correct
            );
        }

        assert_eq!(session.passed(), 20);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected twenty first lab exercise")
                .id,
            "LAB-000021"
        );
        assert!(!session.is_complete());
    }

    #[test]
    fn correct_answer_counts_attempt_pass_and_advances() {
        let mut session = Session::new();

        let result = session.submit_answer("fn main() {}");

        assert_eq!(result, SubmitResult::Correct);
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 1);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected second exercise")
                .id,
            "002"
        );
    }

    #[test]
    fn wrong_answer_counts_attempt_and_stays_on_same_drill() {
        let mut session = Session::new();

        let result = session.submit_answer("wrong\n");

        assert_eq!(result, SubmitResult::TryAgain);
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 1);
        assert_eq!(session.skipped(), 0);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected first exercise")
                .id,
            "001"
        );
    }

    #[test]
    fn skip_counts_skip_and_advances_without_attempt() {
        let mut session = Session::new();

        let result = session.skip();

        assert_eq!(result, SkipResult::Skipped);
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 0);
        assert_eq!(session.skipped(), 1);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected second exercise")
                .id,
            "002"
        );
    }

    #[test]
    fn plain_help_status_and_list_are_not_engine_transitions() {
        let public_source = include_str!("mod.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected engine public source");

        for command_result in ["Help", "Status", "List"] {
            assert!(
                !public_source.contains(command_result),
                "engine should not expose plain command transition {command_result}"
            );
        }
    }

    #[test]
    fn completion_after_final_correct_answer() {
        let mut session = Session::new();

        while let Some(exercise) = session.current_exercise() {
            assert_eq!(
                session.submit_answer(exercise.expected_answer.as_ref()),
                SubmitResult::Correct
            );
        }

        assert_eq!(session.passed(), 22);
        assert_eq!(session.attempts(), 22);
        assert!(session.is_complete());
        assert!(session.current_exercise().is_none());
    }

    #[test]
    fn completion_after_final_skip() {
        let mut session = Session::new();

        while session.current_exercise().is_some() {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }

        assert_eq!(session.skipped(), 22);
        assert_eq!(session.attempts(), 0);
        assert!(session.is_complete());
        assert!(session.current_exercise().is_none());
    }

    #[test]
    fn looping_correct_at_five_opens_one_of_next_correlated_row() {
        let mut session = orchestration::current_interactive_product_session()
            .expect("interactive product session must resolve");
        for _ in 0..4 {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }
        assert_eq!(session.round_position(), Some((5, 5)));
        assert_eq!(session.current_exercise().unwrap().id, "017");

        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn five() -> i32 {\n    5\n}\n", ""),
            ToolResult::success("fn five() -> i32 {\n    5\n}\n", ""),
        ]);
        assert_eq!(
            session.submit_answer_with_runner("fn five()->i32{5}", &mut runner),
            SubmitResult::Correct
        );

        assert_eq!(session.current_exercise().unwrap().id, "018");
        assert_eq!(session.round_position(), Some((1, 5)));
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 1);
        assert!(!session.is_complete());
    }

    #[test]
    fn looping_skip_at_five_opens_one_of_next_correlated_row_without_attempt() {
        let mut session = orchestration::current_interactive_product_session()
            .expect("interactive product session must resolve");
        for _ in 0..5 {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }

        assert_eq!(session.current_exercise().unwrap().id, "018");
        assert_eq!(session.round_position(), Some((1, 5)));
        assert_eq!(session.skipped(), 5);
        assert_eq!(session.attempts(), 0);
        assert_eq!(session.passed(), 0);
        assert!(!session.is_complete());
    }

    #[test]
    fn looping_wrong_at_five_records_attempt_and_stays_at_five() {
        let mut session = orchestration::current_interactive_product_session()
            .expect("interactive product session must resolve");
        for _ in 0..4 {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }
        let mut runner = FakeToolRunner::new(vec![
            ToolResult::success("fn other() -> i32 {\n    5\n}\n", ""),
            ToolResult::success("fn five() -> i32 {\n    5\n}\n", ""),
        ]);

        assert_eq!(
            session.submit_answer_with_runner("fn other()->i32{5}", &mut runner),
            SubmitResult::TryAgain
        );
        assert_eq!(session.current_exercise().unwrap().id, "017");
        assert_eq!(session.round_position(), Some((5, 5)));
        assert_eq!(session.attempts(), 1);
        assert_eq!(session.passed(), 0);
        assert!(!session.is_complete());
    }

    #[test]
    fn rustfmt_rejected_learner_input_is_wrong_attempt_without_advancing() {
        let mut session = one_task_session("015");
        let mut runner =
            FakeToolRunner::new(vec![ToolResult::nonzero(1, "", "submitted parse error\n")]);

        assert_eq!(
            session.submit_answer_with_runner("fn {", &mut runner),
            SubmitResult::TryAgain
        );
        assert_eq!(session.current_exercise().unwrap().id, "015");
        assert_eq!(session.attempts(), 1);
        assert_eq!(session.passed(), 0);
    }

    #[test]
    fn rustfmt_unavailability_is_not_wrong_or_attempt_evidence() {
        let mut session = one_task_session("015");
        let mut runner = FakeToolRunner::new(vec![ToolResult::Failure(ToolFailure::Unavailable {
            tool: ToolKind::Formatter,
        })]);

        assert_eq!(
            session.submit_answer_with_runner("fn five() -> i32 { 5 }", &mut runner),
            SubmitResult::ValidationUnavailable
        );
        assert_eq!(session.current_exercise().unwrap().id, "015");
        assert_eq!(session.attempts(), 0);
        assert_eq!(session.passed(), 0);
        assert_eq!(session.wrong_answer_feedback(), None);
    }

    #[test]
    fn exact_fill_does_not_invoke_rustfmt_and_keeps_exact_bytes() {
        let mut session = one_task_session("014");
        let mut runner = FakeToolRunner::new(Vec::new());

        assert_eq!(
            session.submit_answer_with_runner("5 ", &mut runner),
            SubmitResult::TryAgain
        );
        assert!(runner.requests().is_empty());
        assert_eq!(session.attempts(), 1);
        assert_eq!(session.current_exercise().unwrap().id, "014");
    }

    #[test]
    fn both_varied_fills_accept_only_the_exact_replacement_token() {
        for (id, token, complete_function) in [
            ("014", "->", "fn five() -> i32 { 5 }"),
            ("019", "->", "fn seven() -> u64 { 7 }"),
        ] {
            for accepted in [
                token.to_owned(),
                format!("{token}\n"),
                format!("{token}\r\n"),
            ] {
                let mut session = one_task_session(id);
                let mut runner = FakeToolRunner::new(Vec::new());

                assert_eq!(
                    session.submit_answer_with_runner(&accepted, &mut runner),
                    SubmitResult::Correct
                );
                assert!(runner.requests().is_empty());
                assert_eq!(session.attempts(), 1);
                assert_eq!(session.passed(), 1);
            }

            for wrong in [format!("{token} "), complete_function.to_owned()] {
                let mut session = one_task_session(id);
                let mut runner = FakeToolRunner::new(Vec::new());

                assert_eq!(
                    session.submit_answer_with_runner(&wrong, &mut runner),
                    SubmitResult::TryAgain
                );
                assert!(runner.requests().is_empty());
                assert_eq!(session.attempts(), 1);
                assert_eq!(session.passed(), 0);
                assert_eq!(session.current_exercise().unwrap().id, id);
            }
        }
    }

    #[test]
    fn stats_snapshot_is_accurate() {
        let mut session = Session::new();
        assert_eq!(session.submit_answer("wrong"), SubmitResult::TryAgain);
        assert_eq!(session.skip(), SkipResult::Skipped);

        let snapshot = session.snapshot();

        assert_eq!(
            snapshot.exercise.expect("expected second exercise").id,
            "002"
        );
        assert_eq!(snapshot.stats.passed, 0);
        assert_eq!(snapshot.stats.skipped, 1);
        assert_eq!(snapshot.stats.attempts, 1);
        assert_eq!(snapshot.stats.total_drills, 22);
        assert!(!snapshot.is_complete);
    }

    #[test]
    fn current_exercise_access_tracks_position() {
        let mut session = Session::new();

        assert_eq!(
            session
                .current_exercise()
                .expect("expected first exercise")
                .id,
            "001"
        );
        assert_eq!(session.submit_answer("fn main() {}"), SubmitResult::Correct);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected second exercise")
                .id,
            "002"
        );
    }

    #[test]
    fn exercise_packet_contains_client_facing_fields() {
        let session = Session::new();

        let exercise = session.current_exercise().expect("expected first exercise");

        assert_eq!(exercise.id, "001");
        assert_eq!(exercise.position, 1);
        assert_eq!(exercise.total, 22);
        assert_eq!(exercise.stimulus, "fn main() {}");
        assert_eq!(exercise.expected_answer, "fn main() {}");
        assert_eq!(exercise.instruction, "Copy this line exactly:");
        assert_eq!(exercise.format_capability, FormatCapability::Disabled);
        assert_eq!(exercise.kind, ExerciseKind::TypeExact);
        assert_eq!(exercise.validator, ExerciseValidator::Exact);
        assert_eq!(
            exercise.answer_rule,
            ExerciseAnswerRule::ExactTrimFinalNewline
        );
    }

    #[test]
    fn format_capability_is_explicit_two_state_transport_without_answer_data() {
        let capabilities = [FormatCapability::Disabled, FormatCapability::Rustfmt];

        assert_eq!(capabilities.len(), 2);
        assert!(!capabilities[0].is_available());
        assert!(capabilities[1].is_available());
    }

    #[test]
    fn rustfmt_exact_marker_maps_to_active_validator_spec() {
        let validator = ExerciseValidator::from_validator(curriculum::Validator::RustfmtExact);
        let answer_rule =
            ExerciseAnswerRule::from_answer_rule(curriculum::AnswerRule::RustfmtExact);
        let spec = validators::ValidatorSpec::from_catalog(
            validator.validator(),
            answer_rule.answer_rule(),
        );

        assert_eq!(validator, ExerciseValidator::RustfmtExact);
        assert_eq!(answer_rule, ExerciseAnswerRule::RustfmtExact);
        assert_eq!(
            validators::validate(
                validators::ValidationInput::new("fn main() {}", "fn main() {}"),
                spec,
            ),
            validators::ValidationOutcome::Correct
        );
        assert_ne!(SubmitResult::ValidationUnavailable, SubmitResult::TryAgain);
    }

    #[test]
    fn exercise_packet_advances_after_correct_answer() {
        let mut session = Session::new();

        assert_eq!(session.submit_answer("fn main() {}"), SubmitResult::Correct);
        let exercise = session
            .current_exercise()
            .expect("expected second exercise");

        assert_eq!(exercise.id, "002");
        assert_eq!(exercise.position, 2);
    }

    #[test]
    fn exercise_packet_does_not_advance_after_wrong_answer() {
        let mut session = Session::new();

        assert_eq!(session.submit_answer("wrong"), SubmitResult::TryAgain);
        let exercise = session.current_exercise().expect("expected first exercise");

        assert_eq!(exercise.id, "001");
        assert_eq!(exercise.position, 1);
    }

    #[test]
    fn exercise_packet_advances_after_skip() {
        let mut session = Session::new();

        assert_eq!(session.skip(), SkipResult::Skipped);
        let exercise = session
            .current_exercise()
            .expect("expected second exercise");

        assert_eq!(exercise.id, "002");
        assert_eq!(exercise.position, 2);
    }

    #[test]
    fn completion_snapshot_has_no_current_exercise() {
        let mut session = Session::new();

        while let Some(exercise) = session.current_exercise() {
            assert_eq!(
                session.submit_answer(exercise.expected_answer.as_ref()),
                SubmitResult::Correct
            );
        }

        let snapshot = session.snapshot();

        assert!(snapshot.is_complete);
        assert!(snapshot.exercise.is_none());
        assert_eq!(snapshot.stats.passed, 22);
        assert_eq!(snapshot.stats.total_drills, 22);
    }

    #[test]
    fn submit_answer_uses_exact_validator_law() {
        let mut session = Session::new();

        assert_eq!(
            session.submit_answer("fn main() {} \n"),
            SubmitResult::TryAgain
        );
        assert_eq!(session.attempts(), 1);
        assert_eq!(
            session.submit_answer("fn main() {}\n\n"),
            SubmitResult::TryAgain
        );
        assert_eq!(session.attempts(), 2);
        assert_eq!(
            session.submit_answer("fn main() {}\r\n"),
            SubmitResult::Correct
        );
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 3);
    }

    #[test]
    fn wrong_answer_feedback_is_engine_owned_and_clears_on_navigation() {
        let mut session = Session::new();

        assert_eq!(session.wrong_answer_feedback(), None);
        assert_eq!(session.submit_answer("wrong"), SubmitResult::TryAgain);
        assert_eq!(session.wrong_answer_feedback(), Some("fn main() {}"));
        assert_eq!(session.skip(), SkipResult::Skipped);
        assert_eq!(session.wrong_answer_feedback(), None);
    }

    #[test]
    fn production_engine_has_no_catalog_guidance_or_copy_instruction_owner() {
        let production_source = include_str!("mod.rs")
            .split("\n#[cfg(test)]\nmod tests")
            .next()
            .expect("expected production engine source");

        for forbidden in [
            "catalog::",
            "find_drill",
            "resolve_guidance",
            "GUIDANCE_PACKS",
            "copy.exact_line",
            "Copy this line exactly:",
        ] {
            assert!(
                !production_source.contains(forbidden),
                "engine must not own product task source {forbidden}"
            );
        }
        assert!(production_source.contains("pub instruction:"));
        assert!(production_source.contains("pub stimulus:"));
        assert!(production_source.contains("expected_answer:"));
        assert!(!production_source.contains("pub expected_answer:"));
    }

    #[test]
    fn engine_public_api_stays_terminal_free() {
        let public_source = include_str!("mod.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected engine public source");

        for forbidden in [
            format!("{}{}", "rata", "tui"),
            format!("{}{}", "cross", "term"),
        ] {
            assert!(
                !public_source.contains(&forbidden),
                "engine public API should not mention terminal crate {forbidden}"
            );
        }
    }
}
