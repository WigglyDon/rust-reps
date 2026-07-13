use std::io::{self, Write};

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
    println!("Deterministic Rust practice.");
    println!("Follow the task instruction.");
    println!("Type :help for commands and :q to quit.");
    println!();
}

pub fn write_task_prompt<W: Write>(
    writer: &mut W,
    instruction: &str,
    stimulus: &str,
) -> io::Result<()> {
    writeln!(writer, "{instruction}")?;
    if !stimulus.is_empty() {
        writeln!(writer, "{stimulus}")?;
    }
    Ok(())
}

pub fn print_drill_list() {
    print_app_header();
    println!("Available drills:");
    for drill in catalog::drills() {
        if drill.stimulus.is_empty() {
            println!("{}", drill.id);
        } else {
            println!("{}  {}", drill.id, drill.stimulus);
        }
    }
}

pub fn print_drill_list_body() {
    println!("Available drills:");
    for drill in catalog::drills() {
        if drill.stimulus.is_empty() {
            println!("{}", drill.id);
        } else {
            println!("{}  {}", drill.id, drill.stimulus);
        }
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

pub fn print_answer_try_again(answer_feedback: &str) {
    println!("TRY AGAIN");
    println!("Expected:");
    println!("{}", answer_feedback);
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

#[cfg(test)]
mod tests {
    use super::write_task_prompt;

    struct ProjectionFixture {
        instruction: &'static str,
        stimulus: &'static str,
        expected: &'static str,
    }

    const FIXTURES: [ProjectionFixture; 5] = [
        ProjectionFixture {
            instruction: "Copy this line exactly: Type the complete function exactly.",
            stimulus: "fn five() -> i32 { 5 }",
            expected: "fn five() -> i32 { 5 }",
        },
        ProjectionFixture {
            instruction: "Complete the missing return value: Type only the code replacing ___.",
            stimulus: "fn five() -> i32 { ___ }",
            expected: "fn five() -> i32 { 5 }",
        },
        ProjectionFixture {
            instruction: "Write a Rust function named five with no parameters that returns the i32 value 5 using a tail expression: Submit the complete function.",
            stimulus: "",
            expected: "fn five() -> i32 { 5 }",
        },
        ProjectionFixture {
            instruction: "Repair the return-type syntax: Submit the complete corrected function.",
            stimulus: "fn five(): i32 { 5 }",
            expected: "fn five() -> i32 { 5 }",
        },
        ProjectionFixture {
            instruction: "Rewrite the function to use a tail expression: Submit the complete transformed function.",
            stimulus: "fn five() -> i32 { return 5; }",
            expected: "fn five() -> i32 { 5 }",
        },
    ];

    fn render(fixture: &ProjectionFixture) -> String {
        let mut output = Vec::new();
        write_task_prompt(&mut output, fixture.instruction, fixture.stimulus)
            .expect("task prompt must write");
        String::from_utf8(output).expect("task prompt must be UTF-8")
    }

    #[test]
    fn five_operation_shapes_render_instruction_and_stimulus_without_answer_exposure() {
        let rendered = FIXTURES.iter().map(render).collect::<Vec<_>>();

        assert_eq!(
            rendered[0],
            "Copy this line exactly: Type the complete function exactly.\nfn five() -> i32 { 5 }\n"
        );
        assert_eq!(
            rendered[1],
            "Complete the missing return value: Type only the code replacing ___.\nfn five() -> i32 { ___ }\n"
        );
        assert!(rendered[1].contains("___"));
        assert!(!rendered[1].contains(FIXTURES[1].expected));
        assert_eq!(
            rendered[2],
            "Write a Rust function named five with no parameters that returns the i32 value 5 using a tail expression: Submit the complete function.\n"
        );
        assert!(!rendered[2].contains(FIXTURES[2].expected));
        assert_eq!(
            rendered[3],
            "Repair the return-type syntax: Submit the complete corrected function.\nfn five(): i32 { 5 }\n"
        );
        assert!(rendered[3].contains("(): i32"));
        assert!(!rendered[3].contains(FIXTURES[3].expected));
        assert_eq!(
            rendered[4],
            "Rewrite the function to use a tail expression: Submit the complete transformed function.\nfn five() -> i32 { return 5; }\n"
        );
        assert!(rendered[4].contains("return 5;"));
        assert!(!rendered[4].contains(FIXTURES[4].expected));
    }

    #[test]
    fn task_writer_preserves_unicode_multiline_and_empty_stimulus_bytes() {
        let mut unicode = Vec::new();
        write_task_prompt(&mut unicode, "Скопіюйте точно:", "fn five() -> i32 { 5 }")
            .expect("Unicode task prompt must write");
        assert_eq!(
            String::from_utf8(unicode).expect("Unicode prompt must be UTF-8"),
            "Скопіюйте точно:\nfn five() -> i32 { 5 }\n"
        );

        let mut multiline = Vec::new();
        write_task_prompt(
            &mut multiline,
            "Transform exactly:",
            "line one\n\nline three",
        )
        .expect("multiline task prompt must write");
        assert_eq!(
            String::from_utf8(multiline).expect("multiline prompt must be UTF-8"),
            "Transform exactly:\nline one\n\nline three\n"
        );

        let mut empty = Vec::new();
        write_task_prompt(&mut empty, "Produce exactly:", "")
            .expect("empty-stimulus prompt must write");
        assert_eq!(
            String::from_utf8(empty).expect("empty prompt must be UTF-8"),
            "Produce exactly:\n"
        );
    }
}
