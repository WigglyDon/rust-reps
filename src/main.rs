use std::io::{self, Write};
use std::process;

struct Drill {
    id: &'static str,
    expected: &'static str,
}

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

fn main() {
    let drill = Drill {
        id: "001",
        expected: "fn main() {}",
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
}
