use crate::{answer, catalog, curriculum};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseKind {
    TypeExact,
}

impl ExerciseKind {
    fn from_task_type(task_type: curriculum::TaskType) -> Self {
        match task_type {
            curriculum::TaskType::TypeExact => Self::TypeExact,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseValidator {
    Exact,
}

impl ExerciseValidator {
    fn from_validator(validator: curriculum::Validator) -> Self {
        match validator {
            curriculum::Validator::Exact => Self::Exact,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExercisePacket {
    pub id: &'static str,
    pub position: usize,
    pub total: usize,
    pub prompt: &'static str,
    pub expected_answer: &'static str,
    pub instruction: &'static str,
    pub kind: ExerciseKind,
    pub validator: ExerciseValidator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitResult {
    Correct,
    TryAgain,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionSnapshot {
    pub exercise: Option<ExercisePacket>,
    pub stats: SessionStats,
    pub is_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExerciseSource {
    Catalog,
    Packets(&'static [ExercisePacket]),
}

impl ExerciseSource {
    fn len(self) -> usize {
        match self {
            Self::Catalog => catalog::drills().len(),
            Self::Packets(packets) => packets.len(),
        }
    }

    fn get(self, index: usize) -> Option<ExercisePacket> {
        match self {
            Self::Catalog => catalog_exercise_packet_at(index),
            Self::Packets(packets) => packets.get(index).copied(),
        }
    }
}

pub struct Session {
    source: ExerciseSource,
    index: usize,
    passed: usize,
    skipped: usize,
    attempts: usize,
}

impl Session {
    pub fn new() -> Self {
        Self {
            source: ExerciseSource::Catalog,
            index: 0,
            passed: 0,
            skipped: 0,
            attempts: 0,
        }
    }

    pub fn from_exercise_packets(packets: &'static [ExercisePacket]) -> Self {
        Self {
            source: ExerciseSource::Packets(packets),
            index: 0,
            passed: 0,
            skipped: 0,
            attempts: 0,
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

    pub fn is_complete(&self) -> bool {
        self.index >= self.total_drills()
    }

    pub fn current_exercise(&self) -> Option<ExercisePacket> {
        self.source.get(self.index)
    }

    pub fn first_exercise_packet(&self) -> ExercisePacket {
        self.source.get(0).unwrap_or_else(first_exercise_packet)
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

        self.attempts += 1;
        if answer::check_answer(input, exercise.expected_answer) {
            self.passed += 1;
            self.index += 1;
            return SubmitResult::Correct;
        }

        SubmitResult::TryAgain
    }

    pub fn skip(&mut self) -> SkipResult {
        if self.current_exercise().is_none() {
            return SkipResult::NoActiveDrill;
        }

        self.skipped += 1;
        self.index += 1;
        SkipResult::Skipped
    }
}

pub fn exercise_packets() -> Vec<ExercisePacket> {
    (0..catalog::drills().len())
        .filter_map(catalog_exercise_packet_at)
        .collect()
}

pub fn first_exercise_packet() -> ExercisePacket {
    catalog_exercise_packet_at(0).expect("catalog contains at least one exercise")
}

fn catalog_exercise_packet_at(index: usize) -> Option<ExercisePacket> {
    let drill = catalog::drills().get(index)?;
    Some(ExercisePacket {
        id: drill.id,
        position: index + 1,
        total: catalog::drills().len(),
        prompt: drill.prompt,
        expected_answer: drill.expected,
        instruction: "Copy this line exactly:",
        kind: ExerciseKind::from_task_type(drill.task_type),
        validator: ExerciseValidator::from_validator(drill.validator),
    })
}

#[cfg(test)]
mod tests {
    use super::{ExerciseKind, ExerciseValidator, Session, SkipResult, SubmitResult};
    use crate::lab;

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
    fn lab_session_can_be_built_from_static_packets() {
        let session = Session::from_exercise_packets(lab::exercise_packets());

        assert_eq!(
            session
                .current_exercise()
                .expect("expected first lab exercise")
                .id,
            "LAB-001"
        );
        assert_eq!(session.total_drills(), lab::exercise_packets().len());
        assert_eq!(session.stats().total_drills, lab::exercise_packets().len());
        assert!(!session.is_complete());
    }

    #[test]
    fn lab_multiline_answer_uses_exact_answer_rules() {
        let mut session = Session::from_exercise_packets(lab::exercise_packets());
        assert_eq!(session.skip(), SkipResult::Skipped);

        assert_eq!(
            session.submit_answer(lab::MULTILINE_SOLUTION_EXPECTED),
            SubmitResult::Correct
        );
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 1);
    }

    #[test]
    fn lab_multiline_answer_allows_one_final_line_ending_only() {
        let mut one_final_lf = Session::from_exercise_packets(lab::exercise_packets());
        assert_eq!(one_final_lf.skip(), SkipResult::Skipped);
        assert_eq!(
            one_final_lf.submit_answer(&format!("{}\n", lab::MULTILINE_SOLUTION_EXPECTED)),
            SubmitResult::Correct
        );

        let mut extra_blank_line = Session::from_exercise_packets(lab::exercise_packets());
        assert_eq!(extra_blank_line.skip(), SkipResult::Skipped);
        assert_eq!(
            extra_blank_line.submit_answer(&format!("{}\n\n", lab::MULTILINE_SOLUTION_EXPECTED)),
            SubmitResult::TryAgain
        );
        assert_eq!(
            extra_blank_line
                .current_exercise()
                .expect("expected lab multiline exercise")
                .id,
            lab::MULTILINE_SOLUTION_ID
        );
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
                session.submit_answer(exercise.expected_answer),
                SubmitResult::Correct
            );
        }

        assert_eq!(session.passed(), 12);
        assert_eq!(session.attempts(), 12);
        assert!(session.is_complete());
        assert!(session.current_exercise().is_none());
    }

    #[test]
    fn completion_after_final_skip() {
        let mut session = Session::new();

        while session.current_exercise().is_some() {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }

        assert_eq!(session.skipped(), 12);
        assert_eq!(session.attempts(), 0);
        assert!(session.is_complete());
        assert!(session.current_exercise().is_none());
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
        assert_eq!(snapshot.stats.total_drills, 12);
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
        assert_eq!(exercise.total, 12);
        assert_eq!(exercise.prompt, "fn main() {}");
        assert_eq!(exercise.expected_answer, "fn main() {}");
        assert_eq!(exercise.instruction, "Copy this line exactly:");
        assert_eq!(exercise.kind, ExerciseKind::TypeExact);
        assert_eq!(exercise.validator, ExerciseValidator::Exact);
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
                session.submit_answer(exercise.expected_answer),
                SubmitResult::Correct
            );
        }

        let snapshot = session.snapshot();

        assert!(snapshot.is_complete);
        assert!(snapshot.exercise.is_none());
        assert_eq!(snapshot.stats.passed, 12);
        assert_eq!(snapshot.stats.total_drills, 12);
    }

    #[test]
    fn answer_exactness_comes_from_answer_module() {
        let mut session = Session::new();

        assert_eq!(
            session.submit_answer("fn main() {} \n"),
            SubmitResult::TryAgain
        );
        assert_eq!(session.attempts(), 1);
        assert_eq!(
            session.submit_answer("fn main() {}\r\n"),
            SubmitResult::Correct
        );
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 2);
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
