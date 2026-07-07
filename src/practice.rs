use std::io::{self, Write};

use crate::{answer, catalog, engine, presentation};

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

fn submit_practice_input(session: &mut engine::Session, input: &str) -> PracticeEvent {
    let command = answer::trim_line_ending(input);

    if command == ":q" || command == ":quit" {
        return PracticeEvent::Quit;
    }
    if command == ":help" {
        return PracticeEvent::Help;
    }
    if command == ":status" {
        return PracticeEvent::Status;
    }
    if command == ":list" {
        return PracticeEvent::List;
    }
    if command == ":skip" {
        let _ = session.skip();
        return PracticeEvent::Skip;
    }
    if command.starts_with(':') {
        return PracticeEvent::UnknownCommand;
    }

    match session.submit_answer(input) {
        engine::SubmitResult::Correct => PracticeEvent::Correct,
        engine::SubmitResult::TryAgain => PracticeEvent::TryAgain,
        engine::SubmitResult::NoActiveDrill => PracticeEvent::Quit,
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

fn print_practice_exercise_prompt(exercise: &engine::ExercisePacket) {
    println!("Drill {}/{:03}", exercise.id, exercise.total);
    println!("Prompt:");
    println!();
    println!("{}", exercise.prompt);
    println!();
    print!(">");
}

fn print_practice_exercise_status(exercise: &engine::ExercisePacket, session: &engine::Session) {
    println!("Status:");
    println!("Current: {}/{:03}", exercise.id, exercise.total);
    println!("Passed: {}/{}", session.passed(), exercise.total);
    println!("Skipped: {}", session.skipped());
    println!("Attempts: {}", session.attempts());
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

    let mut session = engine::Session::new();

    while !session.is_complete() {
        let exercise = session
            .current_exercise()
            .expect("current exercise exists while session active");
        print_practice_exercise_prompt(&exercise);
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

        match submit_practice_input(&mut session, &input) {
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
                print_practice_exercise_status(&exercise, &session);
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
                presentation::print_answer_try_again(exercise.expected_answer.as_ref());
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
    use super::{PracticeEvent, submit_practice_input};
    use crate::{catalog, engine};

    #[test]
    fn correct_answer_advances_and_counts() {
        let mut session = engine::Session::new();

        let exercise = session.current_exercise().expect("expected first exercise");
        let event = submit_practice_input(&mut session, exercise.expected_answer.as_ref());

        assert!(matches!(event, PracticeEvent::Correct));
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
    fn wrong_answer_increments_attempts_without_advancing() {
        let mut session = engine::Session::new();

        let event = submit_practice_input(&mut session, "wrong\n");

        assert!(matches!(event, PracticeEvent::TryAgain));
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 1);
        assert_eq!(session.skipped(), 0);
        assert_eq!(
            session
                .current_exercise()
                .expect("expected still first exercise")
                .id,
            "001"
        );
    }

    #[test]
    fn skip_does_not_count_attempts_and_advances() {
        let mut session = engine::Session::new();

        let event = submit_practice_input(&mut session, ":skip\n");

        assert!(matches!(event, PracticeEvent::Skip));
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
    fn unknown_colon_command_does_not_count_attempts() {
        let mut session = engine::Session::new();

        let event = submit_practice_input(&mut session, ":unknown\n");

        assert!(matches!(event, PracticeEvent::UnknownCommand));
        assert_eq!(session.passed(), 0);
        assert_eq!(session.attempts(), 0);
        assert_eq!(session.skipped(), 0);
    }

    #[test]
    fn final_drill_completion_advances_session() {
        let mut session = engine::Session::new();

        for id in [
            "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
        ] {
            let expected = catalog::find_drill(id).expect("expected drill id").expected;
            let event = submit_practice_input(&mut session, expected);
            assert!(matches!(event, PracticeEvent::Correct));
        }

        assert!(session.is_complete());
        assert_eq!(session.passed(), 12);
        assert_eq!(session.attempts(), 12);
        assert_eq!(session.skipped(), 0);
    }
}
