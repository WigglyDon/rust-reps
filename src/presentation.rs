use crate::catalog;

pub const SECTION_RULE: &str = "----------------";

pub fn print_app_header() {
    println!("Rust Reps");
    println!("{}", SECTION_RULE);
    println!();
}

pub fn print_help_text() {
    print_app_header();
    println!("Usage:");
    println!("  rust-reps");
    println!("  rust-reps plain");
    println!("  rust-reps [DRILL_ID]");
    println!("  rust-reps play");
    println!("  rust-reps lab");
    println!("  rust-reps --list");
    println!("  rust-reps --help");
    println!("  rust-reps --version");
    println!();
    println!("Default:");
    println!("  Starts fullscreen TUI in an interactive terminal.");
    println!("  Uses plain practice mode when stdin/stdout are piped.");
    println!();
    println!("Lab:");
    println!("  rust-reps lab starts a fullscreen client stress lab in an interactive terminal.");
    println!("  It is not curriculum and does not appear in --list.");
    println!();
}

pub fn print_practice_header() {
    print_app_header();
    println!("Practice");
    println!("Exact typing practice.");
    println!("Answers must match exactly.");
    println!("Type :help for commands and :q to quit.");
    println!();
}

pub fn print_one_shot_prompt(drill: &catalog::Drill) {
    print_app_header();
    println!("Drill {}", drill.id);
    println!("Prompt:");
    println!();
    println!("{}", drill.expected);
    println!();
    print!(">");
}

pub fn print_drill_list() {
    print_app_header();
    println!("Available drills:");
    for drill in catalog::drills() {
        println!("{}  {}", drill.id, drill.expected);
    }
}

pub fn print_drill_list_body() {
    println!("Available drills:");
    for drill in catalog::drills() {
        println!("{}  {}", drill.id, drill.expected);
    }
}

pub fn print_practice_commands() {
    println!("Commands:");
    println!(":help    show commands");
    println!(":status  show session status");
    println!(":list    show drills");
    println!(":skip    skip current drill");
    println!(":q       quit");
    println!(":quit    quit");
}

pub fn print_unknown_practice_command() {
    println!("UNKNOWN COMMAND");
    println!("Type :help for commands.");
}

pub fn print_answer_pass() {
    println!("PASS");
}

pub fn print_answer_try_again(expected: &str) {
    println!("TRY AGAIN");
    println!("Expected:");
    println!("{}", expected);
}

pub fn print_skip() {
    println!("SKIP");
}

pub fn print_quit(passed: usize, skipped: usize, attempts: usize, total_drills: usize) {
    println!("QUIT");
    print_summary(passed, skipped, attempts, total_drills);
}

pub fn print_complete(passed: usize, skipped: usize, attempts: usize, total_drills: usize) {
    println!("COMPLETE");
    print_summary(passed, skipped, attempts, total_drills);
}

pub fn print_summary(passed: usize, skipped: usize, attempts: usize, total_drills: usize) {
    println!("Summary:");
    println!("Passed: {}/{}", passed, total_drills);
    println!("Skipped: {}", skipped);
    println!("Attempts: {}", attempts);
}
