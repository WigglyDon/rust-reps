pub fn trim_line_ending(input: &str) -> &str {
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

pub fn check_answer(answer: &str, expected: &str) -> bool {
    trim_line_ending(answer) == expected
}

pub fn print_answer_result(answer: &str, expected: &str) {
    if check_answer(answer, expected) {
        println!("PASS");
    } else {
        println!("TRY AGAIN");
        println!("Expected:");
        println!("{}", expected);
    }
}

#[cfg(test)]
mod tests {
    use super::{check_answer, trim_line_ending};

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
    fn trim_line_ending_drops_newline_and_crlf() {
        let answer = "fn main() {}\r\n";
        assert_eq!(trim_line_ending(answer), "fn main() {}");
    }
}
