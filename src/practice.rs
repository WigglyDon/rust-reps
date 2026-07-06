use std::io::{self, Write};

use crate::{answer, catalog};

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

pub fn print_practice_status(current_drill: &catalog::Drill, stats: &PracticeStats) {
    println!("Status:");
    println!(
        "Current: {}/{:03}",
        current_drill.id,
        catalog::drills().len()
    );
    println!("Passed: {}/{}", stats.passed, catalog::drills().len());
    println!("Skipped: {}", stats.skipped);
    println!("Attempts: {}", stats.attempts);
}

fn print_summary(stats: &PracticeStats) {
    println!("Summary:");
    println!("Passed: {}/{}", stats.passed, catalog::drills().len());
    println!("Skipped: {}", stats.skipped);
    println!("Attempts: {}", stats.attempts);
}

fn print_practice_commands() {
    println!("Commands:");
    println!(":help    show commands");
    println!(":status  show session status");
    println!(":list    show drills");
    println!(":skip    skip current drill");
    println!(":q       quit");
    println!(":quit    quit");
}

pub fn run_one_shot_drill(drill: &catalog::Drill) {
    catalog::print_one_shot_prompt(drill);
    flush_stdout_or_exit();

    let mut input = String::new();
    read_line_or_exit(&mut input);
    answer::print_answer_result(&input, drill.expected);
}

pub fn run_practice_session() {
    catalog::print_practice_header();

    let mut stats = PracticeStats::new();
    let mut index = 0;
    while index < catalog::drills().len() {
        let drill = catalog::drills()[index];
        catalog::print_practice_prompt(&drill);
        flush_stdout_or_exit();

        let mut input = String::new();
        if read_line_or_exit(&mut input) == 0 {
            println!("QUIT");
            print_summary(&stats);
            return;
        }

        let answer = answer::trim_line_ending(&input);
        if answer == ":q" || answer == ":quit" {
            println!("QUIT");
            print_summary(&stats);
            return;
        }
        if answer == ":help" {
            print_practice_commands();
            println!();
            continue;
        }
        if answer == ":status" {
            print_practice_status(&drill, &stats);
            println!();
            continue;
        }
        if answer == ":list" {
            catalog::print_drill_catalog_body();
            println!();
            continue;
        }
        if answer == ":skip" {
            stats.skipped += 1;
            println!("SKIP");
            index += 1;
            if index < catalog::drills().len() {
                println!();
            }
            continue;
        }

        stats.attempts += 1;
        if answer::check_answer(&input, drill.expected) {
            stats.passed += 1;
            println!("PASS");
            println!();
            index += 1;
        } else {
            println!("TRY AGAIN");
            println!("Expected:");
            println!("{}", drill.expected);
            println!();
        }
    }

    println!("COMPLETE");
    print_summary(&stats);
}
