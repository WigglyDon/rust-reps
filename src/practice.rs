use std::io::{self, Write};

use crate::{answer, catalog, presentation};

pub struct PracticeStats {
    pub passed: usize,
    pub skipped: usize,
    pub attempts: usize,
}

impl PracticeStats {
    fn new() -> Self {
        Self {
            passed: 0,
            skipped: 0,
            attempts: 0,
        }
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

    let mut stats = PracticeStats::new();
    let total_drills = catalog::drills().len();
    let mut index = 0;

    while index < total_drills {
        let drill = catalog::drills()[index];
        presentation::print_practice_prompt(&drill, total_drills);
        flush_stdout_or_exit();

        let mut input = String::new();
        if read_line_or_exit(&mut input) == 0 {
            presentation::print_quit(stats.passed, stats.skipped, stats.attempts, total_drills);
            return;
        }

        let answer = answer::trim_line_ending(&input);
        if answer == ":q" || answer == ":quit" {
            presentation::print_quit(stats.passed, stats.skipped, stats.attempts, total_drills);
            return;
        }
        if answer == ":help" {
            presentation::print_practice_commands();
            println!();
            continue;
        }
        if answer == ":status" {
            presentation::print_practice_status(
                &drill,
                stats.passed,
                stats.skipped,
                stats.attempts,
                total_drills,
            );
            println!();
            continue;
        }
        if answer == ":list" {
            presentation::print_drill_list_body();
            println!();
            continue;
        }
        if answer == ":skip" {
            stats.skipped += 1;
            presentation::print_skip();
            index += 1;
            if index < total_drills {
                println!();
            }
            continue;
        }
        if answer.starts_with(':') {
            presentation::print_unknown_practice_command();
            println!();
            continue;
        }

        stats.attempts += 1;
        if answer::check_answer(&input, drill.expected) {
            stats.passed += 1;
            presentation::print_answer_pass();
            println!();
            index += 1;
        } else {
            presentation::print_answer_try_again(drill.expected);
            println!();
        }
    }

    presentation::print_complete(stats.passed, stats.skipped, stats.attempts, total_drills);
}
