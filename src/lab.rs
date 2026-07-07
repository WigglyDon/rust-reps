use crate::engine::{ExerciseKind, ExercisePacket, ExerciseValidator};

pub const LONG_PROBLEM_ID: &str = "LAB-001";
pub const MULTILINE_SOLUTION_ID: &str = "LAB-002";
pub const SYNTAX_SAMPLER_ID: &str = "LAB-003";
pub const LAYOUT_PRESSURE_ID: &str = "LAB-004";

pub const LONG_PROBLEM_PROMPT: &str = concat!(
    "task:\n",
    "Build the smallest Rust entry point that still satisfies the lab.\n",
    "\n",
    "requirements:\n",
    "- keep the answer tiny\n",
    "- type it exactly\n",
    "- use this fixture to test problem scrolling\n",
    "- keep the editor and rail fixed while scrolling\n",
    "\n",
    "examples:\n",
    "input: none\n",
    "output: none\n",
    "\n",
    "constraints:\n",
    "- no extra whitespace\n",
    "- no comments in the answer\n",
    "- no hidden validation\n",
    "\n",
    "edge cases:\n",
    "- PageDown should reveal this line\n",
    "- PageUp should return to the task header\n",
    "- the command rail must not move\n",
    "\n",
    "answer shape:\n",
    "fn main() {}"
);

pub const MULTILINE_SOLUTION_PROMPT: &str = concat!(
    "task:\n",
    "Write a tiny multiline function.\n",
    "\n",
    "requirements:\n",
    "- function name: add_one\n",
    "- argument: x: i32\n",
    "- return type: i32\n",
    "- body returns x + 1\n",
    "\n",
    "purpose:\n",
    "This fixture tests multiline editor rendering and exact multiline submit."
);

pub const MULTILINE_SOLUTION_EXPECTED: &str =
    concat!("fn add_one(x: i32) -> i32 {\n", "    x + 1\n", "}");

pub const SYNTAX_SAMPLER_PROMPT: &str = concat!(
    "task:\n",
    "Copy the syntax sampler exactly.\n",
    "\n",
    "coverage:\n",
    "fn let mut i32 usize bool String &str Vec<T> Option<T> Result<T, E>\n",
    "match Some None Ok Err println! format! strings chars numbers comments\n",
    "\n",
    "answer:"
);

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

pub const LAYOUT_PRESSURE_PROMPT: &str = concat!(
    "task:\n",
    "Implement a clamp helper while this problem keeps overflowing.\n",
    "\n",
    "requirements:\n",
    "- accept score: i32\n",
    "- return 0 when score is below zero\n",
    "- return 100 when score is above one hundred\n",
    "- otherwise return score unchanged\n",
    "\n",
    "examples:\n",
    "clamp_score(-1) == 0\n",
    "clamp_score(40) == 40\n",
    "clamp_score(120) == 100\n",
    "\n",
    "constraints:\n",
    "- use if / else if / else\n",
    "- keep the multiline answer exactly shaped\n",
    "- scroll the problem while the editor stays fixed\n",
    "\n",
    "edge cases:\n",
    "- negative scores\n",
    "- zero\n",
    "- upper bound\n",
    "- scores above upper bound\n",
    "\n",
    "answer:"
);

pub const LAYOUT_PRESSURE_EXPECTED: &str = concat!(
    "fn clamp_score(score: i32) -> i32 {\n",
    "    if score < 0 {\n",
    "        0\n",
    "    } else if score > 100 {\n",
    "        100\n",
    "    } else {\n",
    "        score\n",
    "    }\n",
    "}"
);

const LAB_TOTAL: usize = 4;

static LAB_EXERCISES: [ExercisePacket; LAB_TOTAL] = [
    ExercisePacket {
        id: LONG_PROBLEM_ID,
        position: 1,
        total: LAB_TOTAL,
        prompt: LONG_PROBLEM_PROMPT,
        expected_answer: "fn main() {}",
        instruction: "Client stress lab:",
        kind: ExerciseKind::TypeExact,
        validator: ExerciseValidator::Exact,
    },
    ExercisePacket {
        id: MULTILINE_SOLUTION_ID,
        position: 2,
        total: LAB_TOTAL,
        prompt: MULTILINE_SOLUTION_PROMPT,
        expected_answer: MULTILINE_SOLUTION_EXPECTED,
        instruction: "Client stress lab:",
        kind: ExerciseKind::TypeExact,
        validator: ExerciseValidator::Exact,
    },
    ExercisePacket {
        id: SYNTAX_SAMPLER_ID,
        position: 3,
        total: LAB_TOTAL,
        prompt: SYNTAX_SAMPLER_PROMPT,
        expected_answer: SYNTAX_SAMPLER_EXPECTED,
        instruction: "Client stress lab:",
        kind: ExerciseKind::TypeExact,
        validator: ExerciseValidator::Exact,
    },
    ExercisePacket {
        id: LAYOUT_PRESSURE_ID,
        position: 4,
        total: LAB_TOTAL,
        prompt: LAYOUT_PRESSURE_PROMPT,
        expected_answer: LAYOUT_PRESSURE_EXPECTED,
        instruction: "Client stress lab:",
        kind: ExerciseKind::TypeExact,
        validator: ExerciseValidator::Exact,
    },
];

pub fn exercise_packets() -> &'static [ExercisePacket] {
    &LAB_EXERCISES
}

#[cfg(test)]
fn find_exercise(id: &str) -> Option<ExercisePacket> {
    exercise_packets()
        .iter()
        .find(|exercise| exercise.id == id)
        .copied()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        LAYOUT_PRESSURE_ID, LONG_PROBLEM_ID, MULTILINE_SOLUTION_EXPECTED, MULTILINE_SOLUTION_ID,
        SYNTAX_SAMPLER_EXPECTED, SYNTAX_SAMPLER_ID, exercise_packets, find_exercise,
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
    fn lab_fixtures_are_small_unique_and_engine_compatible() {
        let packets = exercise_packets();
        let mut ids = BTreeSet::new();

        assert_eq!(packets.len(), 4);
        for (index, packet) in packets.iter().enumerate() {
            assert!(ids.insert(packet.id), "duplicate lab id {}", packet.id);
            assert_eq!(packet.position, index + 1);
            assert_eq!(packet.total, packets.len());
            assert!(!packet.prompt.trim().is_empty());
            assert!(!packet.expected_answer.trim().is_empty());
            assert_eq!(packet.kind, ExerciseKind::TypeExact);
            assert_eq!(packet.validator, ExerciseValidator::Exact);
        }
        assert!(find_exercise(LONG_PROBLEM_ID).is_some());
        assert!(find_exercise(LAYOUT_PRESSURE_ID).is_some());
    }

    #[test]
    fn lab_fixtures_cover_scroll_multiline_and_syntax_pressure() {
        let long_problem = find_exercise(LONG_PROBLEM_ID).expect("expected long problem fixture");
        let multiline = find_exercise(MULTILINE_SOLUTION_ID).expect("expected multiline fixture");
        let syntax = find_exercise(SYNTAX_SAMPLER_ID).expect("expected syntax fixture");

        assert!(long_problem.prompt.lines().count() > 12);
        assert!(multiline.expected_answer.contains('\n'));
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
    fn syntax_sampler_lines_tokenize_without_losing_text() {
        let syntax = find_exercise(SYNTAX_SAMPLER_ID).expect("expected syntax fixture");
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
