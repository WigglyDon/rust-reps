use crate::engine;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientAction {
    TypeChar(char),
    Backspace,
    Submit,
    Quit,
    ToggleHelp,
    CursorTick,
    Ignore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeTokenKind {
    Keyword,
    Type,
    Number,
    StringLiteral,
    Macro,
    Identifier,
    Punctuation,
    Whitespace,
    Comment,
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeToken {
    pub kind: CodeTokenKind,
    pub text: String,
}

impl CodeToken {
    fn new(kind: CodeTokenKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CounterRewardPhase {
    IdleDot,
    Rise1,
    Rise2,
    PeakCheck,
    HoldCheck,
    Fall1,
    Fall2,
    SettleDot,
}

impl CounterRewardPhase {
    pub fn is_active(self) -> bool {
        self != Self::IdleDot
    }

    fn next(self) -> Self {
        match self {
            Self::IdleDot => Self::IdleDot,
            Self::Rise1 => Self::Rise2,
            Self::Rise2 => Self::PeakCheck,
            Self::PeakCheck => Self::HoldCheck,
            Self::HoldCheck => Self::Fall1,
            Self::Fall1 => Self::Fall2,
            Self::Fall2 => Self::SettleDot,
            Self::SettleDot => Self::IdleDot,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientUiState {
    draft_input: String,
    cursor_visible: bool,
    help_open: bool,
    status_lines: Vec<String>,
    counter_reward_phase: CounterRewardPhase,
}

impl ClientUiState {
    pub fn new() -> Self {
        Self {
            draft_input: String::new(),
            cursor_visible: true,
            help_open: false,
            status_lines: Vec::new(),
            counter_reward_phase: CounterRewardPhase::IdleDot,
        }
    }

    pub fn draft_input(&self) -> &str {
        &self.draft_input
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn help_open(&self) -> bool {
        self.help_open
    }

    pub fn status_lines(&self) -> &[String] {
        &self.status_lines
    }

    pub fn counter_reward_phase(&self) -> CounterRewardPhase {
        self.counter_reward_phase
    }

    pub fn counter_reward_active(&self) -> bool {
        self.counter_reward_phase.is_active()
    }

    pub fn apply_action(&mut self, action: ClientAction) {
        match action {
            ClientAction::TypeChar(ch) => {
                self.draft_input.push(ch);
                self.show_cursor();
            }
            ClientAction::Backspace => {
                self.draft_input.pop();
                self.show_cursor();
            }
            ClientAction::ToggleHelp => {
                self.help_open = !self.help_open;
                self.show_cursor();
            }
            ClientAction::CursorTick => {
                self.cursor_visible = !self.cursor_visible;
                self.advance_counter_reward();
            }
            ClientAction::Submit | ClientAction::Quit | ClientAction::Ignore => {}
        }
    }

    pub fn take_draft_input(&mut self) -> String {
        self.show_cursor();
        std::mem::take(&mut self.draft_input)
    }

    pub fn set_status_lines(&mut self, status_lines: Vec<String>) {
        self.status_lines = status_lines;
    }

    pub fn show_cursor(&mut self) {
        self.cursor_visible = true;
    }

    pub fn start_counter_reward(&mut self) {
        self.counter_reward_phase = CounterRewardPhase::Rise1;
    }

    pub fn clear_counter_reward(&mut self) {
        self.counter_reward_phase = CounterRewardPhase::IdleDot;
    }

    fn advance_counter_reward(&mut self) {
        self.counter_reward_phase = self.counter_reward_phase.next();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientFrame {
    pub game_window: GameWindowFrame,
    pub status_chrome: StatusChromeFrame,
    pub completed_reps_this_run: usize,
    pub counter_reward_phase: CounterRewardPhase,
    pub counter_reward_active: bool,
    pub help_chrome: HelpChromeFrame,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameWindowFrame {
    pub problem: ProblemView,
    pub workbox: WorkboxView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemView {
    pub instruction: String,
    pub prompt: String,
    pub prompt_tokens: Vec<CodeToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkboxView {
    pub input: String,
    pub input_tokens: Vec<CodeToken>,
    pub cursor_visible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusChromeFrame {
    pub lines: Vec<StatusLine>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLine {
    pub tone: StatusTone,
    pub text: String,
    pub code_tokens: Vec<CodeToken>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusTone {
    Pass,
    Complete,
    TryAgain,
    ExpectedLabel,
    UnknownCommand,
    Warning,
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpChromeFrame {
    pub open: bool,
    pub lines: Vec<String>,
}

pub fn project_client_frame(session: &engine::Session, ui_state: &ClientUiState) -> ClientFrame {
    let snapshot = session.snapshot();
    let exercise = snapshot
        .exercise
        .unwrap_or_else(engine::first_exercise_packet);
    let input = ui_state.draft_input().to_string();

    ClientFrame {
        game_window: GameWindowFrame {
            problem: ProblemView {
                instruction: "do this:".to_string(),
                prompt: exercise.prompt.to_string(),
                prompt_tokens: tokenize_rust_line(exercise.prompt),
            },
            workbox: WorkboxView {
                input_tokens: tokenize_rust_line(&input),
                input,
                cursor_visible: ui_state.cursor_visible(),
            },
        },
        status_chrome: StatusChromeFrame {
            lines: status_lines(ui_state.status_lines()),
        },
        completed_reps_this_run: snapshot.stats.passed,
        counter_reward_phase: ui_state.counter_reward_phase(),
        counter_reward_active: ui_state.counter_reward_active(),
        help_chrome: HelpChromeFrame {
            open: ui_state.help_open(),
            lines: help_lines(ui_state.help_open()),
        },
    }
}

pub fn tokenize_rust_line(line: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let ch = chars[index];

        if ch.is_whitespace() {
            let start = index;
            while index < chars.len() && chars[index].is_whitespace() {
                index += 1;
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::Whitespace,
                chars[start..index].iter().collect::<String>(),
            ));
        } else if ch == '/' && chars.get(index + 1) == Some(&'/') {
            tokens.push(CodeToken::new(
                CodeTokenKind::Comment,
                chars[index..].iter().collect::<String>(),
            ));
            break;
        } else if ch == '"' {
            let start = index;
            index += 1;
            let mut escaped = false;
            while index < chars.len() {
                let current = chars[index];
                index += 1;
                if escaped {
                    escaped = false;
                } else if current == '\\' {
                    escaped = true;
                } else if current == '"' {
                    break;
                }
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::StringLiteral,
                chars[start..index].iter().collect::<String>(),
            ));
        } else if ch.is_ascii_digit() {
            let start = index;
            index += 1;
            while index < chars.len()
                && (chars[index].is_ascii_alphanumeric() || chars[index] == '_')
            {
                index += 1;
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::Number,
                chars[start..index].iter().collect::<String>(),
            ));
        } else if is_identifier_start(ch) {
            let start = index;
            index += 1;
            while index < chars.len() && is_identifier_continue(chars[index]) {
                index += 1;
            }
            let identifier: String = chars[start..index].iter().collect();
            if chars.get(index) == Some(&'!') {
                index += 1;
                tokens.push(CodeToken::new(
                    CodeTokenKind::Macro,
                    chars[start..index].iter().collect::<String>(),
                ));
            } else if is_keyword(&identifier) {
                tokens.push(CodeToken::new(CodeTokenKind::Keyword, identifier));
            } else if is_type_name(&identifier) {
                tokens.push(CodeToken::new(CodeTokenKind::Type, identifier));
            } else {
                tokens.push(CodeToken::new(CodeTokenKind::Identifier, identifier));
            }
        } else if ch.is_ascii_punctuation() {
            let start = index;
            index += 1;
            while index < chars.len()
                && chars[index].is_ascii_punctuation()
                && chars[index] != '"'
                && !(chars[index] == '/' && chars.get(index + 1) == Some(&'/'))
                && !is_identifier_start(chars[index])
            {
                index += 1;
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::Punctuation,
                chars[start..index].iter().collect::<String>(),
            ));
        } else {
            tokens.push(CodeToken::new(CodeTokenKind::Plain, ch.to_string()));
            index += 1;
        }
    }

    tokens
}

fn status_lines(lines: &[String]) -> Vec<StatusLine> {
    let mut projected = Vec::new();
    let mut previous_was_expected_label = false;

    for line in lines {
        let code_tokens = if previous_was_expected_label {
            tokenize_rust_line(line)
        } else {
            Vec::new()
        };
        let tone = if previous_was_expected_label {
            StatusTone::Plain
        } else {
            status_tone(line)
        };

        projected.push(StatusLine {
            tone,
            text: line.to_string(),
            code_tokens,
        });
        previous_was_expected_label = is_expected_label(line);
    }

    projected
}

fn status_tone(line: &str) -> StatusTone {
    match line {
        "PASS" => StatusTone::Pass,
        "COMPLETE" => StatusTone::Complete,
        "TRY AGAIN" | "not quite" => StatusTone::TryAgain,
        "Expected:" | "expected:" => StatusTone::ExpectedLabel,
        "UNKNOWN COMMAND" => StatusTone::UnknownCommand,
        "SKIP" | "QUIT" | "STATUS" => StatusTone::Warning,
        _ => StatusTone::Plain,
    }
}

fn is_expected_label(line: &str) -> bool {
    matches!(line, "Expected:" | "expected:")
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

fn is_keyword(identifier: &str) -> bool {
    matches!(
        identifier,
        "fn" | "let"
            | "mut"
            | "if"
            | "else"
            | "while"
            | "for"
            | "loop"
            | "match"
            | "return"
            | "true"
            | "false"
    )
}

fn is_type_name(identifier: &str) -> bool {
    matches!(
        identifier,
        "i32" | "u32" | "usize" | "bool" | "String" | "str" | "char"
    )
}

fn help_lines(open: bool) -> Vec<String> {
    if open {
        vec![
            "Commands:".to_string(),
            "Backspace: delete char".to_string(),
            "Enter: submit input".to_string(),
            "Esc / Ctrl+C: quit".to_string(),
            "F1 or ?: toggle this panel".to_string(),
        ]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClientAction, ClientUiState, CodeTokenKind, CounterRewardPhase, StatusTone,
        project_client_frame, tokenize_rust_line,
    };
    use crate::engine;

    fn token_text(tokens: &[super::CodeToken]) -> String {
        tokens
            .iter()
            .map(|token| token.text.as_str())
            .collect::<String>()
    }

    fn token_kinds_and_text(tokens: &[super::CodeToken]) -> Vec<(CodeTokenKind, String)> {
        tokens
            .iter()
            .map(|token| (token.kind.clone(), token.text.clone()))
            .collect()
    }

    #[test]
    fn tokenizer_highlights_main_function_line() {
        let tokens = tokenize_rust_line("fn main() {}");

        assert_eq!(token_text(&tokens), "fn main() {}");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Keyword && token.text == "fn")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Identifier && token.text == "main")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Punctuation && token.text.contains('{'))
        );
    }

    #[test]
    fn tokenizer_highlights_let_type_and_number() {
        let tokens = tokenize_rust_line("let score: i32 = 0;");

        assert_eq!(token_text(&tokens), "let score: i32 = 0;");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Keyword && token.text == "let")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "i32")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number && token.text == "0")
        );
    }

    #[test]
    fn tokenizer_highlights_macro_and_string_literal() {
        let tokens = tokenize_rust_line("println!(\"Rust Reps\");");

        assert_eq!(token_text(&tokens), "println!(\"Rust Reps\");");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Macro && token.text == "println!")
        );
        assert!(tokens.iter().any(
            |token| token.kind == CodeTokenKind::StringLiteral && token.text == "\"Rust Reps\""
        ));
    }

    #[test]
    fn tokenizer_preserves_exact_text_when_concatenated() {
        for line in [
            "fn main() {}",
            "let score: i32 = 0;",
            "println!(\"Rust Reps\");",
            "if score > 0 {}",
            "// note",
        ] {
            assert_eq!(token_text(&tokenize_rust_line(line)), line);
        }
    }

    #[test]
    fn tokenizer_handles_unknown_identifiers_safely() {
        let tokens = tokenize_rust_line("mystery_value @");

        assert_eq!(token_text(&tokens), "mystery_value @");
        assert_eq!(
            token_kinds_and_text(&tokens)[0],
            (CodeTokenKind::Identifier, "mystery_value".to_string())
        );
    }

    #[test]
    fn frame_projection_includes_problem_prompt() {
        let session = engine::Session::new();
        let ui_state = ClientUiState::new();

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.game_window.problem.instruction, "do this:");
        assert_eq!(frame.game_window.problem.prompt, "fn main() {}");
        assert_eq!(
            token_text(&frame.game_window.problem.prompt_tokens),
            "fn main() {}"
        );
    }

    #[test]
    fn initial_client_frame_counter_is_zero() {
        let session = engine::Session::new();
        let ui_state = ClientUiState::new();

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.counter_reward_phase, CounterRewardPhase::IdleDot);
        assert!(!frame.counter_reward_active);
    }

    #[test]
    fn counter_reward_phase_progresses_through_full_pulse_sequence() {
        let mut ui_state = ClientUiState::new();

        assert_eq!(ui_state.counter_reward_phase(), CounterRewardPhase::IdleDot);
        assert!(!ui_state.counter_reward_active());
        ui_state.start_counter_reward();
        assert_eq!(ui_state.counter_reward_phase(), CounterRewardPhase::Rise1);
        assert!(ui_state.counter_reward_active());

        for phase in [
            CounterRewardPhase::Rise2,
            CounterRewardPhase::PeakCheck,
            CounterRewardPhase::HoldCheck,
            CounterRewardPhase::Fall1,
            CounterRewardPhase::Fall2,
            CounterRewardPhase::SettleDot,
        ] {
            ui_state.apply_action(ClientAction::CursorTick);
            assert_eq!(ui_state.counter_reward_phase(), phase);
            assert!(ui_state.counter_reward_active());
        }

        ui_state.apply_action(ClientAction::CursorTick);
        assert_eq!(ui_state.counter_reward_phase(), CounterRewardPhase::IdleDot);
        assert!(!ui_state.counter_reward_active());
    }

    #[test]
    fn counter_reward_state_can_be_cleared_without_session_truth() {
        let mut ui_state = ClientUiState::new();

        ui_state.start_counter_reward();
        assert!(ui_state.counter_reward_active());

        ui_state.clear_counter_reward();
        assert!(!ui_state.counter_reward_active());
    }

    #[test]
    fn correct_answer_increments_counter_to_one() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        assert!(matches!(
            session.submit_answer("fn main() {}"),
            engine::SubmitResult::Correct
        ));
        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 1);
        assert_eq!(frame.game_window.problem.prompt, "println!(\"Rust Reps\");");
    }

    #[test]
    fn multiple_correct_answers_increment_counter_one_two_three() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        for (answer, expected_counter) in [
            ("fn main() {}", 1),
            ("println!(\"Rust Reps\");", 2),
            ("let answer = 42;", 3),
        ] {
            assert!(matches!(
                session.submit_answer(answer),
                engine::SubmitResult::Correct
            ));
            let frame = project_client_frame(&session, &ui_state);
            assert_eq!(frame.completed_reps_this_run, expected_counter);
        }
    }

    #[test]
    fn wrong_answer_does_not_increment_counter() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        assert!(matches!(
            session.submit_answer("wrong"),
            engine::SubmitResult::TryAgain
        ));
        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 0);
    }

    #[test]
    fn skip_does_not_increment_counter() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        let _ = session.skip();
        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.game_window.problem.prompt, "println!(\"Rust Reps\");");
    }

    #[test]
    fn new_session_starts_counter_at_zero() {
        let mut completed_session = engine::Session::new();
        assert!(matches!(
            completed_session.submit_answer("fn main() {}"),
            engine::SubmitResult::Correct
        ));

        let fresh_session = engine::Session::new();
        let ui_state = ClientUiState::new();
        let frame = project_client_frame(&fresh_session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.counter_reward_phase, CounterRewardPhase::IdleDot);
        assert!(!frame.counter_reward_active);
    }

    #[test]
    fn frame_projection_includes_workbox_input_and_cursor_state() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        ui_state.apply_action(ClientAction::CursorTick);

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.game_window.workbox.input, "x");
        assert_eq!(token_text(&frame.game_window.workbox.input_tokens), "x");
        assert!(!frame.game_window.workbox.cursor_visible);
    }

    #[test]
    fn frame_projection_keeps_status_result_outside_game_window() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.set_status_lines(vec![
            "not quite".to_string(),
            "expected:".to_string(),
            "fn main() {}".to_string(),
        ]);

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(
            frame
                .status_chrome
                .lines
                .iter()
                .map(|line| (
                    &line.tone,
                    line.text.as_str(),
                    token_text(&line.code_tokens)
                ))
                .collect::<Vec<_>>(),
            vec![
                (&StatusTone::TryAgain, "not quite", String::new()),
                (&StatusTone::ExpectedLabel, "expected:", String::new()),
                (
                    &StatusTone::Plain,
                    "fn main() {}",
                    "fn main() {}".to_string()
                ),
            ]
        );
        assert_eq!(frame.game_window.problem.prompt, "fn main() {}");
        assert_eq!(frame.game_window.workbox.input, "");
    }

    #[test]
    fn ui_state_handles_type_backspace_help_and_cursor_tick() {
        let mut ui_state = ClientUiState::new();

        ui_state.apply_action(ClientAction::TypeChar('a'));
        ui_state.apply_action(ClientAction::TypeChar('b'));
        ui_state.apply_action(ClientAction::Backspace);
        ui_state.apply_action(ClientAction::ToggleHelp);
        ui_state.apply_action(ClientAction::CursorTick);

        assert_eq!(ui_state.draft_input(), "a");
        assert!(ui_state.help_open());
        assert!(!ui_state.cursor_visible());
    }

    #[test]
    fn client_public_api_stays_terminal_free() {
        let public_source = include_str!("mod.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected client public source");

        for forbidden in [
            format!("{}{}", "rata", "tui"),
            format!("{}{}", "cross", "term"),
        ] {
            assert!(
                !public_source.contains(&forbidden),
                "client public API should not mention terminal crate {forbidden}"
            );
        }
    }
}
