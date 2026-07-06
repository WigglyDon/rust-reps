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
    fn no_newline_passes() {
        let answer = "fn main() {}";
        assert!(check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn leading_space_fails() {
        let answer = " fn main() {}";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn trailing_space_before_newline_fails() {
        let answer = "fn main() {} \n";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn internal_spacing_differences_fail() {
        let answer = "fn  main() {}";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn tab_character_differences_fail() {
        let answer = "fn\tmain() {}";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn case_sensitivity_fails() {
        let answer = "Fn Main() {}";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn missing_punctuation_fails() {
        let answer = "fn main()";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn empty_string_fails() {
        let answer = "";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn blank_line_fails() {
        let answer = "\n";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn multiple_lines_fail() {
        let answer = "fn main() {}\nprintln!(\"Rust Reps\");\n";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn trailing_cr_without_lf_is_not_normalized() {
        let answer = "fn main() {}\r";
        assert!(!check_answer(answer, "fn main() {}"));
    }

    #[test]
    fn trim_line_ending_drops_newline_and_crlf() {
        let answer = "fn main() {}\r\n";
        assert_eq!(trim_line_ending(answer), "fn main() {}");
    }
}
