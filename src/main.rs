use std::env;
use std::io::{self, Write};
use std::process;

struct Drill {
    id: &'static str,
    expected: &'static str,
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

fn select_drill(args: &[&str]) -> Result<&'static Drill, String> {
    if args.len() > 1 {
        return Err("expected at most one drill id".to_string());
    }

    let id = args.first().copied().unwrap_or("001");
    find_drill(id).ok_or_else(|| format!("unknown drill id: {}", id))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let drill = match select_drill(&arg_refs) {
        Ok(drill) => drill,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };

    println!("Rusty Drill");
    println!();
    println!("Drill {}", drill.id);
    println!("Type exactly:");
    println!();
    println!("{}", drill.expected);
    println!();
    print!(">");

    if let Err(err) = io::stdout().flush() {
        eprintln!("I/O error: {}", err);
        process::exit(1);
    }

    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        eprintln!("I/O error: {}", err);
        process::exit(1);
    }

    if check_answer(&input, drill.expected) {
        println!("PASS");
    } else {
        println!("TRY AGAIN");
        println!("Expected:");
        println!("{}", drill.expected);
    }
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
    fn default_selection_returns_drill_001() {
        let drill = select_drill(&[]).unwrap();
        assert_eq!(drill.id, "001");
        assert_eq!(drill.expected, "fn main() {}");
    }

    #[test]
    fn selecting_002_returns_drill_002() {
        let drill = select_drill(&["002"]).unwrap();
        assert_eq!(drill.id, "002");
        assert_eq!(drill.expected, "println!(\"Rusty Drill\");");
    }

    #[test]
    fn unknown_id_errors() {
        let err = select_drill(&["999"]).err().unwrap();
        assert_eq!(err, "unknown drill id: 999");
    }

    #[test]
    fn too_many_args_errors() {
        let err = select_drill(&["001", "extra"]).err().unwrap();
        assert_eq!(err, "expected at most one drill id");
    }
}
