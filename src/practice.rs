use std::io::{self, Write};

use crate::{answer, catalog, presentation};

#[derive(Clone, Copy)]
pub enum PracticeEvent {
    Help,
    Status,
    List,
    Skip,
    UnknownCommand,
    Correct,
    TryAgain,
    Quit,
}

pub struct PracticeSession {
    index: usize,
    passed: usize,
    skipped: usize,
    attempts: usize,
}

impl PracticeSession {
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

    pub fn submit_input(&mut self, input: &str) -> PracticeEvent {
        let answer = answer::trim_line_ending(input);

        if answer == ":q" || answer == ":quit" {
            return PracticeEvent::Quit;
        }
        if answer == ":help" {
            return PracticeEvent::Help;
        }
        if answer == ":status" {
            return PracticeEvent::Status;
        }
        if answer == ":list" {
            return PracticeEvent::List;
        }
        if answer == ":skip" {
            self.skipped += 1;
            self.index += 1;
            return PracticeEvent::Skip;
        }
        if answer.starts_with(':') {
            return PracticeEvent::UnknownCommand;
        }

        self.attempts += 1;
        let Some(drill) = self.current_drill() else {
            return PracticeEvent::Quit;
        };

        if answer::check_answer(input, drill.expected) {
            self.passed += 1;
            self.index += 1;
            return PracticeEvent::Correct;
        }

        PracticeEvent::TryAgain
    }
}

fn flush_stdout_or_exit() {
    if let Err(err) = io::stdout().flush() {
        eprintln!("I/O error: {}", err);
        std::process::exit(1);
    }
}

fn read_line_or_exit(input: &mut String) -> usize {
    match io::stdin().read_line(input) {
        Ok(bytes_read) => bytes_read,
        Err(err) => {
            eprintln!("I/O error: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn run_one_shot_drill(drill: &catalog::Drill) {
    presentation::print_one_shot_prompt(drill);
    flush_stdout_or_exit();

    let mut input = String::new();
    read_line_or_exit(&mut input);

    if answer::check_answer(&input, drill.expected) {
        presentation::print_answer_pass();
    } else {
        presentation::print_answer_try_again(drill.expected);
    }
}

pub fn run_practice_session() {
    presentation::print_practice_header();

    let mut session = PracticeSession::new();

    while !session.is_complete() {
        let drill = session
            .current_drill()
            .expect("current drill exists while session active");
        presentation::print_practice_prompt(drill, session.total_drills());
        flush_stdout_or_exit();

        let mut input = String::new();
        if read_line_or_exit(&mut input) == 0 {
            presentation::print_quit(
                session.passed(),
                session.skipped(),
                session.attempts(),
                session.total_drills(),
            );
            return;
        }

        match session.submit_input(&input) {
            PracticeEvent::Quit => {
                presentation::print_quit(
                    session.passed(),
                    session.skipped(),
                    session.attempts(),
                    session.total_drills(),
                );
                return;
            }
            PracticeEvent::Help => {
                presentation::print_practice_commands();
                println!();
            }
            PracticeEvent::Status => {
                presentation::print_practice_status(
                    drill,
                    session.passed(),
                    session.skipped(),
                    session.attempts(),
                    session.total_drills(),
                );
                println!();
            }
            PracticeEvent::List => {
                presentation::print_drill_list_body();
                println!();
            }
            PracticeEvent::Skip => {
                presentation::print_skip();
                if !session.is_complete() {
                    println!();
                }
            }
            PracticeEvent::UnknownCommand => {
                presentation::print_unknown_practice_command();
                println!();
            }
            PracticeEvent::Correct => {
                presentation::print_answer_pass();
                println!();
            }
            PracticeEvent::TryAgain => {
                presentation::print_answer_try_again(drill.expected);
                println!();
            }
        }
    }

    presentation::print_complete(
        session.passed(),
        session.skipped(),
        session.attempts(),
        session.total_drills(),
    );
}

#[cfg(test)]
mod tests {
    use super::{PracticeEvent, PracticeSession};
    use crate::catalog;

    #[test]
    fn correct_answer_advances_and_counts() {
        let mut session = PracticeSession::new();

        let drill = session.current_drill().expect("expected first drill");
        let event = session.submit_input(drill.expected);

        assert!(matches!(event, PracticeEvent::Correct));
        assert_eq!(session.passed(), 1);
        assert_eq!(session.attempts(), 1);
        assert_eq!(
            session.current_drill().expect("expected second drill").id,
            "002"
        );
    }

    #[test]
    fn wrong_answer_increments_attempts_without_advancing() {
        let mut session = PracticeSession::new();

        let event = session.submit_input("wrong\n");

        assert!(matches!(event, PracticeEvent::TryAgain));
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 1);
        assert_eq!(session.skipped(), 0);
        assert_eq!(
            session
                .current_drill()
                .expect("expected still first drill")
                .id,
            "001"
        );
    }

    #[test]
    fn skip_does_not_count_attempts_and_advances() {
        let mut session = PracticeSession::new();

        let event = session.submit_input(":skip\n");

        assert!(matches!(event, PracticeEvent::Skip));
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 0);
        assert_eq!(session.skipped(), 1);
        assert_eq!(
            session.current_drill().expect("expected second drill").id,
            "002"
        );
    }

    #[test]
    fn unknown_colon_command_does_not_count_attempts() {
        let mut session = PracticeSession::new();

        let event = session.submit_input(":unknown\n");

        assert!(matches!(event, PracticeEvent::UnknownCommand));
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 0);
        assert_eq!(session.skipped(), 0);
    }

    #[test]
    fn final_drill_completion_advances_session() {
        let mut session = PracticeSession::new();

        for id in [
            "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
        ] {
            let expected = catalog::find_drill(id).expect("expected drill id").expected;
            let event = session.submit_input(expected);
            assert!(matches!(event, PracticeEvent::Correct));
        }

        assert!(session.is_complete());
        assert_eq!(session.passed(), 12);
        assert_eq!(session.attempts(), 12);
        assert_eq!(session.skipped(), 0);
    }
}
