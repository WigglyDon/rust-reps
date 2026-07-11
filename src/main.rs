mod answer;
mod catalog;
mod cli;
mod client;
mod curriculum;
mod engine;
mod formatter;
mod lab;
mod orchestration;
mod practice;
mod presentation;
mod tool_runner;
mod tui;
mod validators;

use std::env;
use std::io::{self, IsTerminal};
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if should_launch_tui_by_default(&args) {
        run_tui_or_exit();
        return;
    }

    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let command = match cli::select_command(&arg_refs) {
        Ok(command) => command,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };

    match command {
        cli::DrillCommand::Practice => {
            practice::run_practice_session(current_product_session_or_exit());
            return;
        }
        cli::DrillCommand::Run(exercise) => practice::run_one_shot_drill(exercise),
        cli::DrillCommand::List => {
            cli::print_drill_list();
            return;
        }
        cli::DrillCommand::Help => {
            cli::print_help();
            return;
        }
        cli::DrillCommand::Version => {
            cli::print_version();
            return;
        }
        cli::DrillCommand::Tui => {
            run_tui_or_exit();
            return;
        }
        cli::DrillCommand::Lab => {
            run_lab_or_exit();
            return;
        }
    }
}

fn should_launch_tui_by_default(args: &[String]) -> bool {
    should_launch_tui_for(
        args.is_empty(),
        io::stdin().is_terminal(),
        io::stdout().is_terminal(),
    )
}

fn should_launch_tui_for(no_args: bool, stdin_is_terminal: bool, stdout_is_terminal: bool) -> bool {
    no_args && stdin_is_terminal && stdout_is_terminal
}

fn run_tui_or_exit() {
    if let Err(err) = tui::run_tui_with_session(current_product_session_or_exit()) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}

fn current_product_session_or_exit() -> engine::Session {
    match orchestration::current_product_session() {
        Ok(session) => session,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}

fn run_lab_or_exit() {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        eprintln!("error: lab mode requires an interactive terminal");
        process::exit(1);
    }

    let product_exercises = match orchestration::current_product_exercise_packets() {
        Ok(exercises) => exercises,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };
    if let Err(err) = tui::run_tui_with_session(engine::Session::lab_with_product_exercises(
        product_exercises,
    )) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{answer, catalog, cli};

    #[test]
    fn exact_answer_passes() {
        assert!(answer::check_answer("fn main() {}", "fn main() {}"));
    }

    #[test]
    fn wrong_answer_fails() {
        assert!(!answer::check_answer("fn main(){}", "fn main() {}"));
    }

    #[test]
    fn trailing_lf_is_ignored() {
        assert!(answer::check_answer("fn main() {}\n", "fn main() {}"));
    }

    #[test]
    fn trailing_crlf_is_ignored() {
        assert!(answer::check_answer("fn main() {}\r\n", "fn main() {}"));
    }

    #[test]
    fn internal_spacing_differences_fail() {
        assert!(!answer::check_answer("fn  main() {}", "fn main() {}"));
    }

    #[test]
    fn default_selection_returns_practice() {
        match cli::select_command(&[]).unwrap() {
            cli::DrillCommand::Practice => {}
            cli::DrillCommand::Run(_) => panic!("expected practice command"),
            cli::DrillCommand::List => panic!("expected practice command"),
            cli::DrillCommand::Help => panic!("expected practice command"),
            cli::DrillCommand::Version => panic!("expected practice command"),
            cli::DrillCommand::Tui => panic!("expected practice command"),
            cli::DrillCommand::Lab => panic!("expected practice command"),
        }
    }

    #[test]
    fn interactive_empty_args_selects_tui_default() {
        assert!(super::should_launch_tui_for(true, true, true));
        assert!(!super::should_launch_tui_for(true, false, true));
        assert!(!super::should_launch_tui_for(true, true, false));
        assert!(!super::should_launch_tui_for(false, true, true));
    }

    #[test]
    fn play_arg_selects_practice() {
        match cli::select_command(&["play"]).unwrap() {
            cli::DrillCommand::Practice => {}
            cli::DrillCommand::Run(_) => panic!("expected practice command"),
            cli::DrillCommand::List => panic!("expected practice command"),
            cli::DrillCommand::Help => panic!("expected practice command"),
            cli::DrillCommand::Version => panic!("expected practice command"),
            cli::DrillCommand::Tui => panic!("expected practice command"),
            cli::DrillCommand::Lab => panic!("expected practice command"),
        }
    }

    #[test]
    fn plain_arg_selects_practice() {
        match cli::select_command(&["plain"]).unwrap() {
            cli::DrillCommand::Practice => {}
            cli::DrillCommand::Run(_) => panic!("expected practice command"),
            cli::DrillCommand::List => panic!("expected practice command"),
            cli::DrillCommand::Help => panic!("expected practice command"),
            cli::DrillCommand::Version => panic!("expected practice command"),
            cli::DrillCommand::Tui => panic!("expected practice command"),
            cli::DrillCommand::Lab => panic!("expected practice command"),
        }
    }

    #[test]
    fn selecting_001_returns_drill_001() {
        match cli::select_command(&["001"]).unwrap() {
            cli::DrillCommand::Run(drill) => {
                assert_eq!(drill.id, "001");
                assert_eq!(drill.stimulus, "fn main() {}");
                assert_eq!(drill.expected_answer(), "fn main() {}");
            }
            cli::DrillCommand::Practice => panic!("expected Drill 001"),
            cli::DrillCommand::List => panic!("expected Drill 001"),
            cli::DrillCommand::Help => panic!("expected Drill 001"),
            cli::DrillCommand::Version => panic!("expected Drill 001"),
            cli::DrillCommand::Tui => panic!("expected Drill 001"),
            cli::DrillCommand::Lab => panic!("expected Drill 001"),
        }
    }

    #[test]
    fn selecting_012_returns_drill_012() {
        match cli::select_command(&["012"]).unwrap() {
            cli::DrillCommand::Run(drill) => {
                assert_eq!(drill.id, "012");
                assert_eq!(drill.stimulus, "while score < 10 {}");
                assert_eq!(drill.expected_answer(), "while score < 10 {}");
            }
            cli::DrillCommand::Practice => panic!("expected Drill 012"),
            cli::DrillCommand::List => panic!("expected Drill 012"),
            cli::DrillCommand::Help => panic!("expected Drill 012"),
            cli::DrillCommand::Version => panic!("expected Drill 012"),
            cli::DrillCommand::Tui => panic!("expected Drill 012"),
            cli::DrillCommand::Lab => panic!("expected Drill 012"),
        }
    }

    #[test]
    fn list_arg_selects_list_command() {
        match cli::select_command(&["--list"]).unwrap() {
            cli::DrillCommand::List => {}
            cli::DrillCommand::Practice => panic!("expected list command"),
            cli::DrillCommand::Run(_) => panic!("expected list command"),
            cli::DrillCommand::Help => panic!("expected list command"),
            cli::DrillCommand::Version => panic!("expected list command"),
            cli::DrillCommand::Tui => panic!("expected list command"),
            cli::DrillCommand::Lab => panic!("expected list command"),
        }
    }

    #[test]
    fn help_arg_selects_help_command() {
        match cli::select_command(&["--help"]).unwrap() {
            cli::DrillCommand::Help => {}
            cli::DrillCommand::Practice => panic!("expected help command"),
            cli::DrillCommand::Run(_) => panic!("expected help command"),
            cli::DrillCommand::List => panic!("expected help command"),
            cli::DrillCommand::Version => panic!("expected help command"),
            cli::DrillCommand::Tui => panic!("expected help command"),
            cli::DrillCommand::Lab => panic!("expected help command"),
        }
    }

    #[test]
    fn short_help_arg_selects_help_command() {
        match cli::select_command(&["-h"]).unwrap() {
            cli::DrillCommand::Help => {}
            cli::DrillCommand::Practice => panic!("expected help command"),
            cli::DrillCommand::Run(_) => panic!("expected help command"),
            cli::DrillCommand::List => panic!("expected help command"),
            cli::DrillCommand::Version => panic!("expected help command"),
            cli::DrillCommand::Tui => panic!("expected help command"),
            cli::DrillCommand::Lab => panic!("expected help command"),
        }
    }

    #[test]
    fn version_arg_selects_version_command() {
        match cli::select_command(&["--version"]).unwrap() {
            cli::DrillCommand::Version => {}
            cli::DrillCommand::Practice => panic!("expected version command"),
            cli::DrillCommand::Run(_) => panic!("expected version command"),
            cli::DrillCommand::List => panic!("expected version command"),
            cli::DrillCommand::Help => panic!("expected version command"),
            cli::DrillCommand::Tui => panic!("expected version command"),
            cli::DrillCommand::Lab => panic!("expected version command"),
        }
    }

    #[test]
    fn unknown_id_errors() {
        let err = cli::select_command(&["999"]).err().unwrap();
        assert_eq!(err, "unknown drill id: 999");
    }

    #[test]
    fn unknown_long_option_errors() {
        let err = cli::select_command(&["--wat"]).err().unwrap();
        assert_eq!(err, "unknown argument: --wat");
    }

    #[test]
    fn unknown_short_option_errors() {
        let err = cli::select_command(&["-x"]).err().unwrap();
        assert_eq!(err, "unknown argument: -x");
    }

    #[test]
    fn too_many_args_errors() {
        let err = cli::select_command(&["--help", "extra"]).err().unwrap();
        assert_eq!(err, "expected at most one argument");
    }

    #[test]
    fn too_many_version_args_errors() {
        let err = cli::select_command(&["--version", "extra"]).err().unwrap();
        assert_eq!(err, "expected at most one argument");
    }

    #[test]
    fn tui_arg_selects_tui_command() {
        match cli::select_command(&["tui"]).unwrap() {
            cli::DrillCommand::Tui => {}
            cli::DrillCommand::Practice => panic!("expected tui command"),
            cli::DrillCommand::Run(_) => panic!("expected tui command"),
            cli::DrillCommand::List => panic!("expected tui command"),
            cli::DrillCommand::Help => panic!("expected tui command"),
            cli::DrillCommand::Version => panic!("expected tui command"),
            cli::DrillCommand::Lab => panic!("expected tui command"),
        }
    }

    #[test]
    fn lab_arg_selects_lab_command() {
        match cli::select_command(&["lab"]).unwrap() {
            cli::DrillCommand::Lab => {}
            cli::DrillCommand::Practice => panic!("expected lab command"),
            cli::DrillCommand::Run(_) => panic!("expected lab command"),
            cli::DrillCommand::List => panic!("expected lab command"),
            cli::DrillCommand::Help => panic!("expected lab command"),
            cli::DrillCommand::Version => panic!("expected lab command"),
            cli::DrillCommand::Tui => panic!("expected lab command"),
        }
    }

    #[test]
    fn catalog_contains_expected_twelve_drill_ids_in_order() {
        let ids: Vec<&str> = catalog::drills().iter().map(|drill| drill.id).collect();
        assert_eq!(
            ids,
            vec![
                "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
            ]
        );
    }
}
