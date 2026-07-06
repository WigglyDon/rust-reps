use std::env;
use std::io::{self, Write};
use std::process;

struct Drill {
    id: &'static str,
    expected: &'static str,
}

enum DrillCommand {
    Practice,
    Run(&'static Drill),
    List,
    Help,
    Version,
}

struct PracticeStats {
    passed: usize,
    skipped: usize,
    attempts: usize,
}

const DRILLS: &[Drill] = &[
    Drill {
        id: "001",
        expected: "fn main() {}",
    },
    Drill {
        id: "002",
        expected: "println!(\"Rusty Drill\");",
    },
    Drill {
        id: "003",
        expected: "let answer = 42;",
    },
    Drill {
        id: "004",
        expected: "let name = \"Rusty Drill\";",
    },
    Drill {
        id: "005",
        expected: "let mut score = 0;",
    },
    Drill {
        id: "006",
        expected: "score += 1;",
    },
    Drill {
        id: "007",
        expected: "let passed = true;",
    },
    Drill {
        id: "008",
        expected: "let score: i32 = 0;",
    },
    Drill {
        id: "009",
        expected: "let scores = [1, 2, 3];",
    },
    Drill {
        id: "010",
        expected: "fn add_one(x: i32) -> i32 { x + 1 }",
    },
    Drill {
        id: "011",
        expected: "if score > 0 {}",
    },
    Drill {
        id: "012",
        expected: "while score < 10 {}",
    },
];

fn trim_line_ending(input: &str) -> &str {
    if input.ends_with('\n') {
        let without_lf = &input[..input.len() - 1];
        if without_lf.ends_with('\r') {
            &without_lf[..without_lf.len() - 1]
        } else {
            without_lf
        }
    } else {
        input
    }
}

fn check_answer(answer: &str, expected: &str) -> bool {
    trim_line_ending(answer) == expected
}

fn find_drill(id: &str) -> Option<&'static Drill> {
    DRILLS.iter().find(|drill| drill.id == id)
}

fn select_command(args: &[&str]) -> Result<DrillCommand, String> {
    if args.len() > 1 {
        return Err("expected at most one argument".to_string());
    }

    let id = args.first().copied().unwrap_or("play");
    if id == "play" {
        return Ok(DrillCommand::Practice);
    }
    if id == "--list" {
        return Ok(DrillCommand::List);
    }
    if id == "--help" || id == "-h" {
        return Ok(DrillCommand::Help);
    }
    if id == "--version" {
        return Ok(DrillCommand::Version);
    }
    if id.starts_with('-') {
        return Err(format!("unknown argument: {}", id));
    }

    find_drill(id)
        .map(DrillCommand::Run)
        .ok_or_else(|| format!("unknown drill id: {}", id))
}

fn print_prompt(drill: &Drill) {
    println!("Rusty Drill");
    println!();
    println!("Drill {}", drill.id);
    println!("Type exactly:");
    println!();
    println!("{}", drill.expected);
    println!();
    print!(">");
}

fn print_practice_header() {
    println!("Rusty Drill");
    println!();
    println!("Practice");
    println!("Type :help for commands.");
    println!();
}

fn print_practice_prompt(drill: &Drill) {
    println!("Drill {}/{:03}", drill.id, DRILLS.len());
    println!("Type exactly:");
    println!();
    println!("{}", drill.expected);
    println!();
    print!(">");
}

fn print_drill_list() {
    println!("Rusty Drill");
    println!();
    print_drill_catalog_body();
}

fn print_drill_catalog_body() {
    println!("Available drills:");
    for drill in DRILLS {
        println!("{}  {}", drill.id, drill.expected);
    }
}

fn print_help() {
    println!("Rusty Drill");
    println!();
    println!("Usage:");
    println!("  rusty-drill");
    println!("  rusty-drill [DRILL_ID]");
    println!("  rusty-drill play");
    println!("  rusty-drill --list");
    println!("  rusty-drill --help");
    println!("  rusty-drill --version");
    println!();
    println!("Default:");
    println!("  Starts a practice session.");
    println!();
    println!("Current drills:");
    for drill in DRILLS {
        println!("{}  {}", drill.id, drill.expected);
    }
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_summary(stats: &PracticeStats) {
    println!("Summary:");
    println!("Passed: {}/{}", stats.passed, DRILLS.len());
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

fn print_practice_status(current_drill: &Drill, stats: &PracticeStats) {
    println!("Status:");
    println!("Current: {}/{:03}", current_drill.id, DRILLS.len());
    println!("Passed: {}/{}", stats.passed, DRILLS.len());
    println!("Skipped: {}", stats.skipped);
    println!("Attempts: {}", stats.attempts);
}

fn flush_stdout_or_exit() {
    if let Err(err) = io::stdout().flush() {
        eprintln!("I/O error: {}", err);
        process::exit(1);
    }
}

fn read_line_or_exit(input: &mut String) -> usize {
    match io::stdin().read_line(input) {
        Ok(bytes_read) => bytes_read,
        Err(err) => {
            eprintln!("I/O error: {}", err);
            process::exit(1);
        }
    }
}

fn print_answer_result(answer: &str, expected: &str) {
    if check_answer(answer, expected) {
        println!("PASS");
    } else {
        println!("TRY AGAIN");
        println!("Expected:");
        println!("{}", expected);
    }
}

fn run_one_shot_drill(drill: &Drill) {
    print_prompt(drill);
    flush_stdout_or_exit();

    let mut input = String::new();
    read_line_or_exit(&mut input);
    print_answer_result(&input, drill.expected);
}

fn run_practice_session() {
    print_practice_header();

    let mut stats = PracticeStats {
        passed: 0,
        skipped: 0,
        attempts: 0,
    };
    let mut index = 0;
    while index < DRILLS.len() {
        let drill = &DRILLS[index];
        print_practice_prompt(drill);
        flush_stdout_or_exit();

        let mut input = String::new();
        if read_line_or_exit(&mut input) == 0 {
            println!("QUIT");
            print_summary(&stats);
            return;
        }

        let answer = trim_line_ending(&input);
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
            print_practice_status(drill, &stats);
            println!();
            continue;
        }
        if answer == ":list" {
            print_drill_catalog_body();
            println!();
            continue;
        }
        if answer == ":skip" {
            stats.skipped += 1;
            println!("SKIP");
            index += 1;
            if index < DRILLS.len() {
                println!();
            }
            continue;
        }

        stats.attempts += 1;
        if check_answer(&input, drill.expected) {
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

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let command = match select_command(&arg_refs) {
        Ok(command) => command,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };

    let drill = match command {
        DrillCommand::Practice => {
            run_practice_session();
            return;
        }
        DrillCommand::Run(drill) => drill,
        DrillCommand::List => {
            print_drill_list();
            return;
        }
        DrillCommand::Help => {
            print_help();
            return;
        }
        DrillCommand::Version => {
            print_version();
            return;
        }
    };

    run_one_shot_drill(drill);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_answer_passes() {
        let answer = "fn main() {}";
        assert!(check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn wrong_answer_fails() {
        let answer = "fn main(){}";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn trailing_lf_is_ignored() {
        let answer = "fn main() {}\n";
        assert!(check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn trailing_crlf_is_ignored() {
        let answer = "fn main() {}\r\n";
        assert!(check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn internal_spacing_differences_fail() {
        let answer = "fn  main() {}";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn default_selection_returns_practice() {
        match select_command(&[]).unwrap() {
            DrillCommand::Practice => {}
            DrillCommand::Run(_) => panic!("expected practice command"),
            DrillCommand::List => panic!("expected practice command"),
            DrillCommand::Help => panic!("expected practice command"),
            DrillCommand::Version => panic!("expected practice command"),
        }
    }

    #[test]
    fn play_arg_selects_practice() {
        match select_command(&["play"]).unwrap() {
            DrillCommand::Practice => {}
            DrillCommand::Run(_) => panic!("expected practice command"),
            DrillCommand::List => panic!("expected practice command"),
            DrillCommand::Help => panic!("expected practice command"),
            DrillCommand::Version => panic!("expected practice command"),
        }
    }

    #[test]
    fn selecting_001_returns_drill_001() {
        match select_command(&["001"]).unwrap() {
            DrillCommand::Run(drill) => {
                assert_eq!(drill.id, "001");
                assert_eq!(drill.expected, "fn main() {}");
            }
            DrillCommand::Practice => panic!("expected Drill 001"),
            DrillCommand::List => panic!("expected Drill 001"),
            DrillCommand::Help => panic!("expected Drill 001"),
            DrillCommand::Version => panic!("expected Drill 001"),
        }
    }

    #[test]
    fn selecting_012_returns_drill_012() {
        match select_command(&["012"]).unwrap() {
            DrillCommand::Run(drill) => {
                assert_eq!(drill.id, "012");
                assert_eq!(drill.expected, "while score < 10 {}");
            }
            DrillCommand::Practice => panic!("expected Drill 012"),
            DrillCommand::List => panic!("expected Drill 012"),
            DrillCommand::Help => panic!("expected Drill 012"),
            DrillCommand::Version => panic!("expected Drill 012"),
        }
    }

    #[test]
    fn list_arg_selects_list_command() {
        match select_command(&["--list"]).unwrap() {
            DrillCommand::List => {}
            DrillCommand::Practice => panic!("expected list command"),
            DrillCommand::Run(_) => panic!("expected list command"),
            DrillCommand::Help => panic!("expected list command"),
            DrillCommand::Version => panic!("expected list command"),
        }
    }

    #[test]
    fn help_arg_selects_help_command() {
        match select_command(&["--help"]).unwrap() {
            DrillCommand::Help => {}
            DrillCommand::Practice => panic!("expected help command"),
            DrillCommand::Run(_) => panic!("expected help command"),
            DrillCommand::List => panic!("expected help command"),
            DrillCommand::Version => panic!("expected help command"),
        }
    }

    #[test]
    fn short_help_arg_selects_help_command() {
        match select_command(&["-h"]).unwrap() {
            DrillCommand::Help => {}
            DrillCommand::Practice => panic!("expected help command"),
            DrillCommand::Run(_) => panic!("expected help command"),
            DrillCommand::List => panic!("expected help command"),
            DrillCommand::Version => panic!("expected help command"),
        }
    }

    #[test]
    fn version_arg_selects_version_command() {
        match select_command(&["--version"]).unwrap() {
            DrillCommand::Version => {}
            DrillCommand::Practice => panic!("expected version command"),
            DrillCommand::Run(_) => panic!("expected version command"),
            DrillCommand::List => panic!("expected version command"),
            DrillCommand::Help => panic!("expected version command"),
        }
    }

    #[test]
    fn unknown_id_errors() {
        let err = select_command(&["999"]).err().unwrap();
        assert_eq!(err, "unknown drill id: 999");
    }

    #[test]
    fn unknown_long_option_errors() {
        let err = select_command(&["--wat"]).err().unwrap();
        assert_eq!(err, "unknown argument: --wat");
    }

    #[test]
    fn unknown_short_option_errors() {
        let err = select_command(&["-x"]).err().unwrap();
        assert_eq!(err, "unknown argument: -x");
    }

    #[test]
    fn too_many_args_errors() {
        let err = select_command(&["--help", "extra"]).err().unwrap();
        assert_eq!(err, "expected at most one argument");
    }

    #[test]
    fn too_many_version_args_errors() {
        let err = select_command(&["--version", "extra"]).err().unwrap();
        assert_eq!(err, "expected at most one argument");
    }

    #[test]
    fn catalog_contains_expected_twelve_drill_ids_in_order() {
        let ids: Vec<&str> = DRILLS.iter().map(|drill| drill.id).collect();
        assert_eq!(
            ids,
            vec![
                "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
            ]
        );
    }
}
