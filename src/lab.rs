use std::borrow::Cow;

use crate::engine::{ExerciseKind, ExercisePacket, ExerciseValidator};

pub const STRESS_FAMILY_COUNT: usize = 9;
#[cfg(test)]
pub const MULTILINE_SOLUTION_FAMILY_INDEX: usize = 4;
#[cfg(test)]
pub const SYNTAX_SAMPLER_FAMILY_INDEX: usize = 6;
pub const LAB_INSTRUCTION: &str = "Client stress lab:";
pub const LAB_TOTAL: usize = usize::MAX;

pub const MULTILINE_SOLUTION_EXPECTED: &str =
    concat!("fn add_one(x: i32) -> i32 {\n", "    x + 1\n", "}");

pub const SYNTAX_SAMPLER_EXPECTED: &str = concat!(
    "fn describe(score: i32, name: &str) -> Result<String, String> {\n",
    "    let mut values: Vec<i32> = vec![score, 10, 20];\n",
    "    let ready: bool = values.len() > 0usize;\n",
    "    let marker: char = 'R';\n",
    "    // sample variants and macros\n",
    "    match values.pop() {\n",
    "        Some(value) if ready => Ok(format!(\"{}:{}:{}\", name, value, marker)),\n",
    "        Some(_) | None => Err(String::from(\"empty\")),\n",
    "    }\n",
    "}"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabStressFamily {
    LongWrappedProblem,
    ManyShortProblemLines,
    DeepIndentedCodeBlock,
    LongUnbrokenToken,
    MultilineExactAnswer,
    AnswerTallerThanEditor,
    SyntaxSampler,
    LayoutPressure,
    TinyControl,
}

const STRESS_FAMILIES: [LabStressFamily; STRESS_FAMILY_COUNT] = [
    LabStressFamily::LongWrappedProblem,
    LabStressFamily::ManyShortProblemLines,
    LabStressFamily::DeepIndentedCodeBlock,
    LabStressFamily::LongUnbrokenToken,
    LabStressFamily::MultilineExactAnswer,
    LabStressFamily::AnswerTallerThanEditor,
    LabStressFamily::SyntaxSampler,
    LabStressFamily::LayoutPressure,
    LabStressFamily::TinyControl,
];

pub fn stress_family_at(index: usize) -> LabStressFamily {
    STRESS_FAMILIES[index % STRESS_FAMILY_COUNT]
}

pub fn exercise_packet_at(index: usize) -> ExercisePacket {
    let number = index.saturating_add(1);
    let family = stress_family_at(index);
    let id = format!("LAB-{number:06}");

    let (prompt, expected_answer) = match family {
        LabStressFamily::LongWrappedProblem => long_wrapped_problem(number),
        LabStressFamily::ManyShortProblemLines => many_short_problem_lines(number),
        LabStressFamily::DeepIndentedCodeBlock => deep_indented_code_block(number),
        LabStressFamily::LongUnbrokenToken => long_unbroken_token(number),
        LabStressFamily::MultilineExactAnswer => multiline_exact_answer(number),
        LabStressFamily::AnswerTallerThanEditor => answer_taller_than_editor(number),
        LabStressFamily::SyntaxSampler => syntax_sampler(number),
        LabStressFamily::LayoutPressure => layout_pressure(number),
        LabStressFamily::TinyControl => tiny_control(number),
    };

    ExercisePacket {
        id: Cow::Owned(id),
        position: number,
        total: LAB_TOTAL,
        prompt: Cow::Owned(prompt),
        expected_answer: Cow::Owned(expected_answer),
        instruction: Cow::Borrowed(LAB_INSTRUCTION),
        kind: ExerciseKind::TypeExact,
        validator: ExerciseValidator::Exact,
    }
}

fn long_wrapped_problem(number: usize) -> (String, String) {
    (
        format!(
            "task:\n\
             Build the smallest Rust entry point for lab case {number} while this sentence stays \
             intentionally long enough to force deterministic wrapping inside a narrow terminal \
             problem pane without moving the editor or command rail.\n\
             \n\
             requirements:\n\
             - keep the answer tiny\n\
             - type it exactly\n\
             - keep overflowing task text bounded inside the task panel\n\
             - keep the editor viewport independent\n\
             \n\
             examples:\n\
             input: none\n\
             output: none\n\
             \n\
             constraints:\n\
             - no extra whitespace\n\
             - no hidden validation\n\
             - this lab problem is generated deterministically from its index\n\
             \n\
             edge cases:\n\
             - long wrapped prose\n\
             - narrow terminal widths\n\
             - reward rail stability\n\
             \n\
             answer shape:\n\
             fn main() {{}}"
        ),
        "fn main() {}".to_string(),
    )
}

fn many_short_problem_lines(number: usize) -> (String, String) {
    let mut prompt =
        format!("task:\nCopy the tiny constant for lab case {number}.\n\nchecklist:\n");
    for line in 1..=28 {
        prompt.push_str(&format!("- visible short line {line:02}\n"));
    }
    prompt.push_str("\nanswer shape:\nlet score: i32 = 1;");
    (prompt, "let score: i32 = 1;".to_string())
}

fn deep_indented_code_block(number: usize) -> (String, String) {
    (
        format!(
            "task:\n\
             Copy a small nested Rust block for lab case {number}.\n\
             \n\
             sample shape:\n\
             fn main() {{\n\
                 if true {{\n\
                     while false {{\n\
                         println!(\"never\");\n\
                     }}\n\
                 }}\n\
             }}\n\
             \n\
             purpose:\n\
             This stresses indentation, gutter alignment, and syntax colors."
        ),
        concat!(
            "fn main() {\n",
            "    if true {\n",
            "        println!(\"nested\");\n",
            "    }\n",
            "}"
        )
        .to_string(),
    )
}

fn long_unbroken_token(number: usize) -> (String, String) {
    (
        format!(
            "task:\n\
             Inspect this long unbroken path for lab case {number}:\n\
             crate::stress::very::long::module::path::that::keeps::going::without::spaces::LAB{number:06}::VALUE\n\
             \n\
             requirement:\n\
             The problem formatter must split this path deterministically instead of smashing \
             the app layout."
        ),
        "let path_ok = true;".to_string(),
    )
}

fn multiline_exact_answer(number: usize) -> (String, String) {
    (
        format!(
            "task:\n\
             Write a tiny multiline function for lab case {number}.\n\
             \n\
             requirements:\n\
             - function name: add_one\n\
             - argument: x: i32\n\
             - return type: i32\n\
             - body returns x + 1\n\
             \n\
             purpose:\n\
             This fixture tests exact multiline submit and failed-attempt preservation."
        ),
        MULTILINE_SOLUTION_EXPECTED.to_string(),
    )
}

fn answer_taller_than_editor(number: usize) -> (String, String) {
    let prompt = format!(
        "task:\n\
         Type a table-shaped function for lab case {number}.\n\
         \n\
         purpose:\n\
         The expected answer is intentionally taller than the normal editor viewport so cursor \
         movement must scroll the editor view without changing the buffer."
    );
    let expected = concat!(
        "fn scores() -> [i32; 8] {\n",
        "    [\n",
        "        1,\n",
        "        2,\n",
        "        3,\n",
        "        4,\n",
        "        5,\n",
        "        6,\n",
        "        7,\n",
        "        8,\n",
        "    ]\n",
        "}"
    )
    .to_string();
    (prompt, expected)
}

fn syntax_sampler(number: usize) -> (String, String) {
    (
        format!(
            "task:\n\
             Copy the syntax sampler for lab case {number}.\n\
             \n\
             coverage:\n\
             fn let mut i32 usize bool String &str Vec<T> Option<T> Result<T, E>\n\
             match Some None Ok Err println! format! strings chars numbers comments\n\
             \n\
             answer:"
        ),
        SYNTAX_SAMPLER_EXPECTED.to_string(),
    )
}

fn layout_pressure(number: usize) -> (String, String) {
    (
        format!(
            "task:\n\
             Implement a clamp helper for lab case {number} while this problem keeps overflowing.\n\
             \n\
             requirements:\n\
             - accept score: i32\n\
             - return 0 when score is below zero\n\
             - return 100 when score is above one hundred\n\
             - otherwise return score unchanged\n\
             \n\
             examples:\n\
             clamp_score(-1) == 0\n\
             clamp_score(40) == 40\n\
             clamp_score(120) == 100\n\
             \n\
             constraints:\n\
             - use if / else if / else\n\
             - keep the multiline answer exactly shaped\n\
             - scroll the problem while the editor stays fixed\n\
             \n\
             edge cases:\n\
             - negative scores\n\
             - zero\n\
             - upper bound\n\
             - scores above upper bound\n\
             \n\
             answer:"
        ),
        concat!(
            "fn clamp_score(score: i32) -> i32 {\n",
            "    if score < 0 {\n",
            "        0\n",
            "    } else if score > 100 {\n",
            "        100\n",
            "    } else {\n",
            "        score\n",
            "    }\n",
            "}"
        )
        .to_string(),
    )
}

fn tiny_control(number: usize) -> (String, String) {
    (
        format!("task:\nTiny control case {number}.\n\nanswer:\ntrue"),
        "true".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        LabStressFamily, MULTILINE_SOLUTION_EXPECTED, MULTILINE_SOLUTION_FAMILY_INDEX,
        STRESS_FAMILY_COUNT, SYNTAX_SAMPLER_EXPECTED, SYNTAX_SAMPLER_FAMILY_INDEX,
        exercise_packet_at, stress_family_at,
    };
    use crate::client::{CodeTokenKind, tokenize_rust_line};
    use crate::engine::{ExerciseKind, ExerciseValidator};

    fn token_text(tokens: &[crate::client::CodeToken]) -> String {
        tokens
            .iter()
            .map(|token| token.text.as_str())
            .collect::<String>()
    }

    #[test]
    fn generated_lab_packets_are_unique_and_engine_compatible() {
        let mut ids = BTreeSet::new();

        for index in 0..(STRESS_FAMILY_COUNT * 3) {
            let packet = exercise_packet_at(index);
            assert!(
                ids.insert(packet.id.clone()),
                "duplicate lab id {}",
                packet.id
            );
            assert_eq!(packet.id.as_ref(), format!("LAB-{:06}", index + 1));
            assert_eq!(packet.position, index + 1);
            assert_eq!(packet.total, usize::MAX);
            assert!(!packet.prompt.trim().is_empty());
            assert!(!packet.expected_answer.trim().is_empty());
            assert_eq!(packet.kind, ExerciseKind::TypeExact);
            assert_eq!(packet.validator, ExerciseValidator::Exact);
        }
    }

    #[test]
    fn stress_matrix_covers_required_families() {
        let families = (0..STRESS_FAMILY_COUNT)
            .map(stress_family_at)
            .collect::<Vec<_>>();

        assert_eq!(families.len(), 9);
        assert!(families.contains(&LabStressFamily::LongWrappedProblem));
        assert!(families.contains(&LabStressFamily::ManyShortProblemLines));
        assert!(families.contains(&LabStressFamily::DeepIndentedCodeBlock));
        assert!(families.contains(&LabStressFamily::LongUnbrokenToken));
        assert!(families.contains(&LabStressFamily::MultilineExactAnswer));
        assert!(families.contains(&LabStressFamily::AnswerTallerThanEditor));
        assert!(families.contains(&LabStressFamily::SyntaxSampler));
        assert!(families.contains(&LabStressFamily::LayoutPressure));
        assert!(families.contains(&LabStressFamily::TinyControl));
    }

    #[test]
    fn lab_fixtures_cover_scroll_multiline_and_syntax_pressure() {
        let long_problem = exercise_packet_at(0);
        let multiline = exercise_packet_at(MULTILINE_SOLUTION_FAMILY_INDEX);
        let syntax = exercise_packet_at(SYNTAX_SAMPLER_FAMILY_INDEX);

        assert!(long_problem.prompt.lines().count() > 20);
        assert!(multiline.expected_answer.contains('\n'));
        assert_eq!(multiline.expected_answer, MULTILINE_SOLUTION_EXPECTED);
        assert!(syntax.prompt.contains("Vec<T>"));
        assert!(syntax.prompt.contains("Result<T, E>"));
        assert!(syntax.expected_answer.contains("match"));
        assert!(syntax.expected_answer.contains("Some(value)"));
        assert!(syntax.expected_answer.contains("None"));
        assert!(syntax.expected_answer.contains("Ok(format!"));
        assert!(syntax.expected_answer.contains("Err(String::from"));
        assert!(syntax.expected_answer.contains("// sample"));
        assert!(SYNTAX_SAMPLER_EXPECTED.contains("'R'"));
        assert!(MULTILINE_SOLUTION_EXPECTED.contains("x + 1"));
    }

    #[test]
    fn lab_prompts_do_not_teach_page_scroll_copy() {
        for index in 0..STRESS_FAMILY_COUNT {
            let packet = exercise_packet_at(index);

            assert!(!packet.prompt.contains("PageDown"));
            assert!(!packet.prompt.contains("PageUp"));
        }
    }

    #[test]
    fn lab_packets_are_deterministic_for_same_index() {
        let first = exercise_packet_at(17);
        let second = exercise_packet_at(17);

        assert_eq!(first.id, second.id);
        assert_eq!(first.prompt, second.prompt);
        assert_eq!(first.expected_answer, second.expected_answer);
    }

    #[test]
    fn syntax_sampler_lines_tokenize_without_losing_text() {
        let syntax = exercise_packet_at(SYNTAX_SAMPLER_FAMILY_INDEX);
        let combined = format!("{}\n{}", syntax.prompt, syntax.expected_answer);
        let mut saw_keyword = false;
        let mut saw_type = false;
        let mut saw_number = false;
        let mut saw_string = false;
        let mut saw_macro = false;
        let mut saw_comment = false;

        for line in combined.lines() {
            let tokens = tokenize_rust_line(line);
            assert_eq!(token_text(&tokens), line);
            saw_keyword |= tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Keyword);
            saw_type |= tokens.iter().any(|token| token.kind == CodeTokenKind::Type);
            saw_number |= tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number);
            saw_string |= tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::StringLiteral);
            saw_macro |= tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Macro);
            saw_comment |= tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Comment);
        }

        assert!(saw_keyword);
        assert!(saw_type);
        assert!(saw_number);
        assert!(saw_string);
        assert!(saw_macro);
        assert!(saw_comment);
    }
}
