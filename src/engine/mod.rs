use crate::{answer, catalog};

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

#[derive(Clone, Copy)]
pub struct SessionSnapshot {
    pub current_drill: Option<&'static catalog::Drill>,
    pub stats: SessionStats,
    pub is_complete: bool,
}

pub struct Session {
    index: usize,
    passed: usize,
    skipped: usize,
    attempts: usize,
}

impl Session {
    pub fn new() -> Self {
        Self {
            index: 0,
            passed: 0,
            skipped: 0,
            attempts: 0,
        }
    }

    pub fn total_drills(&self) -> usize {
        catalog::drills().len()
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

    pub fn current_drill(&self) -> Option<&'static catalog::Drill> {
        catalog::drills().get(self.index)
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
            current_drill: self.current_drill(),
            stats: self.stats(),
            is_complete: self.is_complete(),
        }
    }

    pub fn submit_answer(&mut self, input: &str) -> SubmitResult {
        let Some(drill) = self.current_drill() else {
            return SubmitResult::NoActiveDrill;
        };

        self.attempts += 1;
        if answer::check_answer(input, drill.expected) {
            self.passed += 1;
            self.index += 1;
            return SubmitResult::Correct;
        }

        SubmitResult::TryAgain
    }

    pub fn skip(&mut self) -> SkipResult {
        if self.current_drill().is_none() {
            return SkipResult::NoActiveDrill;
        }

        self.skipped += 1;
        self.index += 1;
        SkipResult::Skipped
    }
}

#[cfg(test)]
mod tests {
    use super::{Session, SkipResult, SubmitResult};

    #[test]
    fn new_session_starts_at_drill_001() {
        let session = Session::new();

        assert_eq!(
            session.current_drill().expect("expected first drill").id,
            "001"
        );
        assert_eq!(session.passed(), 0);
        assert_eq!(session.skipped(), 0);
        assert_eq!(session.attempts(), 0);
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
            session.current_drill().expect("expected second drill").id,
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
            session.current_drill().expect("expected first drill").id,
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
            session.current_drill().expect("expected second drill").id,
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

        while let Some(drill) = session.current_drill() {
            assert_eq!(session.submit_answer(drill.expected), SubmitResult::Correct);
        }

        assert_eq!(session.passed(), 12);
        assert_eq!(session.attempts(), 12);
        assert!(session.is_complete());
        assert!(session.current_drill().is_none());
    }

    #[test]
    fn completion_after_final_skip() {
        let mut session = Session::new();

        while session.current_drill().is_some() {
            assert_eq!(session.skip(), SkipResult::Skipped);
        }

        assert_eq!(session.skipped(), 12);
        assert_eq!(session.attempts(), 0);
        assert!(session.is_complete());
        assert!(session.current_drill().is_none());
    }

    #[test]
    fn stats_snapshot_is_accurate() {
        let mut session = Session::new();
        assert_eq!(session.submit_answer("wrong"), SubmitResult::TryAgain);
        assert_eq!(session.skip(), SkipResult::Skipped);

        let snapshot = session.snapshot();

        assert_eq!(
            snapshot.current_drill.expect("expected second drill").id,
            "002"
        );
        assert_eq!(snapshot.stats.passed, 0);
        assert_eq!(snapshot.stats.skipped, 1);
        assert_eq!(snapshot.stats.attempts, 1);
        assert_eq!(snapshot.stats.total_drills, 12);
        assert!(!snapshot.is_complete);
    }

    #[test]
    fn current_drill_access_tracks_position() {
        let mut session = Session::new();

        assert_eq!(
            session.current_drill().expect("expected first drill").id,
            "001"
        );
        assert_eq!(session.submit_answer("fn main() {}"), SubmitResult::Correct);
        assert_eq!(
            session.current_drill().expect("expected second drill").id,
            "002"
        );
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
