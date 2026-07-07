use std::time::{Duration, Instant};

use crate::engine;

mod editor;

pub use editor::{CursorPosition, EditorAction, EditorBuffer, EditorView, EditorViewport};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientAction {
    TypeChar(char),
    Backspace,
    InsertNewline,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveLineStart,
    MoveLineEnd,
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

pub const COUNTER_REWARD_ATTACK_DURATION: Duration = Duration::from_millis(120);
pub const COUNTER_REWARD_HOLD_DURATION: Duration = Duration::from_millis(180);
pub const COUNTER_REWARD_RELEASE_DURATION: Duration = Duration::from_millis(1_000);
pub const COUNTER_REWARD_DURATION: Duration = Duration::from_millis(1_300);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CounterRewardFrame {
    pub active: bool,
    pub intensity: u8,
}

impl CounterRewardFrame {
    fn idle() -> Self {
        Self {
            active: false,
            intensity: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CounterRewardAnimation {
    started_at: Option<Instant>,
    duration: Duration,
}

impl CounterRewardAnimation {
    fn new() -> Self {
        Self {
            started_at: None,
            duration: COUNTER_REWARD_DURATION,
        }
    }

    fn start(&mut self, now: Instant) {
        self.started_at = Some(now);
    }

    fn clear(&mut self) {
        self.started_at = None;
    }

    fn settle(&mut self, now: Instant) {
        if !self.frame_at(now).active {
            self.clear();
        }
    }

    fn frame_at(self, now: Instant) -> CounterRewardFrame {
        let Some(started_at) = self.started_at else {
            return CounterRewardFrame::idle();
        };
        let elapsed = now.saturating_duration_since(started_at);
        if elapsed >= self.duration {
            return CounterRewardFrame::idle();
        }

        CounterRewardFrame {
            active: true,
            intensity: reward_intensity_at(elapsed),
        }
    }
}

fn reward_intensity_at(elapsed: Duration) -> u8 {
    if elapsed >= COUNTER_REWARD_DURATION {
        return 0;
    }

    let release_starts_at = COUNTER_REWARD_ATTACK_DURATION + COUNTER_REWARD_HOLD_DURATION;
    let intensity = if elapsed < COUNTER_REWARD_ATTACK_DURATION {
        ease_out_cubic(duration_progress(elapsed, COUNTER_REWARD_ATTACK_DURATION))
    } else if elapsed < release_starts_at {
        1.0
    } else {
        let release_elapsed = elapsed.saturating_sub(release_starts_at);
        1.0 - smoothstep(duration_progress(
            release_elapsed,
            COUNTER_REWARD_RELEASE_DURATION,
        ))
    };

    (intensity * 100.0).round().clamp(0.0, 100.0) as u8
}

fn duration_progress(elapsed: Duration, duration: Duration) -> f32 {
    if duration.is_zero() {
        return 1.0;
    }

    (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0)
}

fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress).powi(3)
}

fn smoothstep(progress: f32) -> f32 {
    progress * progress * (3.0 - (2.0 * progress))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientUiState {
    editor: EditorBuffer,
    cursor_visible: bool,
    help_open: bool,
    status_lines: Vec<String>,
    counter_reward: CounterRewardAnimation,
}

impl ClientUiState {
    pub fn new() -> Self {
        Self {
            editor: EditorBuffer::new(),
            cursor_visible: true,
            help_open: false,
            status_lines: Vec::new(),
            counter_reward: CounterRewardAnimation::new(),
        }
    }

    pub fn editor_view(&self) -> EditorView {
        self.editor.view()
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

    pub fn counter_reward_frame_at(&self, now: Instant) -> CounterRewardFrame {
        self.counter_reward.frame_at(now)
    }

    pub fn counter_reward_active_at(&self, now: Instant) -> bool {
        self.counter_reward_frame_at(now).active
    }

    pub fn apply_action(&mut self, action: ClientAction) {
        match action {
            ClientAction::TypeChar(ch) => {
                self.editor.apply_action(EditorAction::InsertChar(ch));
                self.show_cursor();
            }
            ClientAction::Backspace => {
                self.editor.apply_action(EditorAction::Backspace);
                self.show_cursor();
            }
            ClientAction::InsertNewline => {
                self.editor.apply_action(EditorAction::InsertNewline);
                self.show_cursor();
            }
            ClientAction::MoveCursorLeft => {
                self.editor.apply_action(EditorAction::MoveLeft);
                self.show_cursor();
            }
            ClientAction::MoveCursorRight => {
                self.editor.apply_action(EditorAction::MoveRight);
                self.show_cursor();
            }
            ClientAction::MoveCursorUp => {
                self.editor.apply_action(EditorAction::MoveUp);
                self.show_cursor();
            }
            ClientAction::MoveCursorDown => {
                self.editor.apply_action(EditorAction::MoveDown);
                self.show_cursor();
            }
            ClientAction::MoveLineStart => {
                self.editor.apply_action(EditorAction::MoveLineStart);
                self.show_cursor();
            }
            ClientAction::MoveLineEnd => {
                self.editor.apply_action(EditorAction::MoveLineEnd);
                self.show_cursor();
            }
            ClientAction::ToggleHelp => {
                self.help_open = !self.help_open;
                self.show_cursor();
            }
            ClientAction::CursorTick => {
                self.cursor_visible = !self.cursor_visible;
            }
            ClientAction::Submit | ClientAction::Quit | ClientAction::Ignore => {}
        }
    }

    pub fn take_draft_input(&mut self) -> String {
        self.show_cursor();
        let input = self.editor.text();
        self.editor.clear();
        input
    }

    pub fn set_status_lines(&mut self, status_lines: Vec<String>) {
        self.status_lines = status_lines;
    }

    pub fn show_cursor(&mut self) {
        self.cursor_visible = true;
    }

    pub fn start_counter_reward(&mut self, now: Instant) {
        self.counter_reward.start(now);
    }

    pub fn clear_counter_reward(&mut self) {
        self.counter_reward.clear();
    }

    pub fn settle_counter_reward(&mut self, now: Instant) {
        self.counter_reward.settle(now);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientFrame {
    pub game_window: GameWindowFrame,
    pub status_chrome: StatusChromeFrame,
    pub completed_reps_this_run: usize,
    pub counter_reward: CounterRewardFrame,
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
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
    pub viewport: EditorViewport,
    pub input_tokens: Vec<CodeToken>,
    pub line_tokens: Vec<Vec<CodeToken>>,
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

#[cfg(test)]
pub fn project_client_frame(session: &engine::Session, ui_state: &ClientUiState) -> ClientFrame {
    project_client_frame_at(session, ui_state, Instant::now())
}

pub fn project_client_frame_at(
    session: &engine::Session,
    ui_state: &ClientUiState,
    now: Instant,
) -> ClientFrame {
    let snapshot = session.snapshot();
    let exercise = snapshot
        .exercise
        .unwrap_or_else(engine::first_exercise_packet);
    let editor_view = ui_state.editor_view();
    let input = editor_view.text.clone();
    let line_tokens = editor_view
        .lines
        .iter()
        .map(|line| tokenize_rust_line(line))
        .collect();
    let counter_reward = ui_state.counter_reward_frame_at(now);

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
                lines: editor_view.lines,
                cursor: editor_view.cursor,
                viewport: editor_view.viewport,
                line_tokens,
                cursor_visible: ui_state.cursor_visible(),
            },
        },
        status_chrome: StatusChromeFrame {
            lines: status_lines(ui_state.status_lines()),
        },
        completed_reps_this_run: snapshot.stats.passed,
        counter_reward,
        counter_reward_active: counter_reward.active,
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
        "i32" | "u32" | "usize" | "bool" | "String" | "str" | "char" | "Option"
    )
}

fn help_lines(open: bool) -> Vec<String> {
    if open {
        vec![
            "Commands:".to_string(),
            "Enter newline".to_string(),
            "Ctrl+Enter submit".to_string(),
            "Esc quit".to_string(),
            "? help".to_string(),
        ]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{
        COUNTER_REWARD_ATTACK_DURATION, COUNTER_REWARD_DURATION, COUNTER_REWARD_HOLD_DURATION,
        COUNTER_REWARD_RELEASE_DURATION, ClientAction, ClientUiState, CodeTokenKind,
        CursorPosition, StatusTone, project_client_frame, project_client_frame_at,
        tokenize_rust_line,
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

    fn test_now() -> Instant {
        Instant::now()
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
    fn tokenizer_highlights_mut_binding_array_and_control_flow_lines() {
        for (line, expected_tokens) in [
            (
                "let mut score = 0;",
                vec![
                    (CodeTokenKind::Keyword, "let"),
                    (CodeTokenKind::Keyword, "mut"),
                    (CodeTokenKind::Identifier, "score"),
                    (CodeTokenKind::Number, "0"),
                ],
            ),
            (
                "let scores = [1, 2, 3];",
                vec![
                    (CodeTokenKind::Keyword, "let"),
                    (CodeTokenKind::Identifier, "scores"),
                    (CodeTokenKind::Number, "1"),
                    (CodeTokenKind::Number, "2"),
                    (CodeTokenKind::Number, "3"),
                ],
            ),
            (
                "if score > 0 {}",
                vec![
                    (CodeTokenKind::Keyword, "if"),
                    (CodeTokenKind::Identifier, "score"),
                    (CodeTokenKind::Number, "0"),
                ],
            ),
            (
                "while score < 10 {}",
                vec![
                    (CodeTokenKind::Keyword, "while"),
                    (CodeTokenKind::Identifier, "score"),
                    (CodeTokenKind::Number, "10"),
                ],
            ),
        ] {
            let tokens = tokenize_rust_line(line);
            assert_eq!(token_text(&tokens), line);
            for (kind, text) in expected_tokens {
                assert!(
                    tokens
                        .iter()
                        .any(|token| token.kind == kind && token.text == text),
                    "expected token {kind:?} {text:?} in {line:?}"
                );
            }
        }
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
    fn tokenizer_highlights_comment_references_and_generic_type_names() {
        let comment_tokens = tokenize_rust_line("// comment");
        assert_eq!(token_text(&comment_tokens), "// comment");
        assert_eq!(comment_tokens[0].kind, CodeTokenKind::Comment);

        let reference_tokens = tokenize_rust_line("&str");
        assert_eq!(token_text(&reference_tokens), "&str");
        assert!(
            reference_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Punctuation && token.text == "&")
        );
        assert!(
            reference_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "str")
        );

        let generic_tokens = tokenize_rust_line("Option<String>");
        assert_eq!(token_text(&generic_tokens), "Option<String>");
        assert!(
            generic_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "Option")
        );
        assert!(
            generic_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "String")
        );
    }

    #[test]
    fn tokenizer_preserves_exact_text_when_concatenated() {
        for line in [
            "fn main() {}",
            "let score: i32 = 0;",
            "let mut score = 0;",
            "println!(\"Rust Reps\");",
            "let scores = [1, 2, 3];",
            "if score > 0 {}",
            "while score < 10 {}",
            "// note",
            "&str",
            "Option<String>",
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
        let now = test_now();

        let frame = project_client_frame_at(&session, &ui_state, now);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.counter_reward.intensity, 0);
        assert!(!frame.counter_reward_active);
    }

    #[test]
    fn reward_attack_reaches_high_intensity_quickly() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        assert!(!ui_state.counter_reward_active_at(now));
        ui_state.start_counter_reward(now);

        let start = ui_state.counter_reward_frame_at(now);
        let quick_hit = ui_state.counter_reward_frame_at(now + COUNTER_REWARD_ATTACK_DURATION / 2);
        let attack_end = ui_state.counter_reward_frame_at(now + COUNTER_REWARD_ATTACK_DURATION);

        assert!(start.active);
        assert_eq!(start.intensity, 0);
        assert!(quick_hit.active);
        assert!(quick_hit.intensity >= 80);
        assert!(attack_end.active);
        assert_eq!(attack_end.intensity, 100);
    }

    #[test]
    fn reward_holds_peak_briefly_then_releases_slowly() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();
        ui_state.start_counter_reward(now);

        let hold_mid = ui_state.counter_reward_frame_at(
            now + COUNTER_REWARD_ATTACK_DURATION + (COUNTER_REWARD_HOLD_DURATION / 2),
        );
        let release_starts_at = COUNTER_REWARD_ATTACK_DURATION + COUNTER_REWARD_HOLD_DURATION;
        let early_release =
            ui_state.counter_reward_frame_at(now + release_starts_at + Duration::from_millis(250));
        let late_release =
            ui_state.counter_reward_frame_at(now + release_starts_at + Duration::from_millis(800));

        assert_eq!(hold_mid.intensity, 100);
        assert!(COUNTER_REWARD_RELEASE_DURATION > COUNTER_REWARD_ATTACK_DURATION);
        assert!(early_release.active);
        assert!(late_release.active);
        assert!(early_release.intensity < hold_mid.intensity);
        assert!(early_release.intensity > late_release.intensity);
        assert!(late_release.intensity > 0);
    }

    #[test]
    fn reward_animation_expires_after_duration() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        ui_state.start_counter_reward(now);
        assert!(ui_state.counter_reward_active_at(now + Duration::from_millis(1)));
        assert!(!ui_state.counter_reward_active_at(now + COUNTER_REWARD_DURATION));

        ui_state.settle_counter_reward(now + COUNTER_REWARD_DURATION);
        assert!(!ui_state.counter_reward_active_at(now + COUNTER_REWARD_DURATION));
    }

    #[test]
    fn counter_reward_state_can_be_cleared_without_session_truth() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        ui_state.start_counter_reward(now);
        assert!(ui_state.counter_reward_active_at(now));

        ui_state.clear_counter_reward();
        assert!(!ui_state.counter_reward_active_at(now));
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
        let now = test_now();
        let frame = project_client_frame_at(&fresh_session, &ui_state, now);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.counter_reward.intensity, 0);
        assert!(!frame.counter_reward_active);
    }

    #[test]
    fn frame_projection_includes_workbox_input_and_cursor_state() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        ui_state.apply_action(ClientAction::InsertNewline);
        ui_state.apply_action(ClientAction::TypeChar('y'));
        ui_state.apply_action(ClientAction::CursorTick);

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.game_window.workbox.input, "x\ny");
        assert_eq!(frame.game_window.workbox.lines, vec!["x", "y"]);
        assert_eq!(
            frame
                .game_window
                .workbox
                .line_tokens
                .iter()
                .map(|tokens| token_text(tokens))
                .collect::<Vec<_>>(),
            vec!["x".to_string(), "y".to_string()]
        );
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
        ui_state.apply_action(ClientAction::InsertNewline);
        ui_state.apply_action(ClientAction::TypeChar('b'));
        ui_state.apply_action(ClientAction::Backspace);
        ui_state.apply_action(ClientAction::ToggleHelp);
        ui_state.apply_action(ClientAction::CursorTick);

        assert_eq!(ui_state.editor_view().text, "a\n");
        assert!(ui_state.help_open());
        assert!(!ui_state.cursor_visible());
    }

    #[test]
    fn client_action_insert_newline_updates_editor_buffer() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('a'));
        ui_state.apply_action(ClientAction::InsertNewline);
        ui_state.apply_action(ClientAction::TypeChar('b'));

        let view = ui_state.editor_view();
        assert_eq!(view.text, "a\nb");
        assert_eq!(view.lines, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(view.cursor, CursorPosition { line: 1, column: 1 });
    }

    #[test]
    fn client_action_move_left_and_right_change_editor_cursor() {
        let mut ui_state = ClientUiState::new();
        for ch in "abc".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }

        ui_state.apply_action(ClientAction::MoveCursorLeft);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 2 }
        );
        ui_state.apply_action(ClientAction::MoveCursorRight);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 3 }
        );
    }

    #[test]
    fn printable_char_inserts_at_cursor() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('a'));
        ui_state.apply_action(ClientAction::TypeChar('c'));
        ui_state.apply_action(ClientAction::MoveCursorLeft);

        ui_state.apply_action(ClientAction::TypeChar('b'));

        assert_eq!(ui_state.editor_view().text, "abc");
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 2 }
        );
    }

    #[test]
    fn backspace_deletes_before_cursor() {
        let mut ui_state = ClientUiState::new();
        for ch in "abc".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }
        ui_state.apply_action(ClientAction::MoveCursorLeft);

        ui_state.apply_action(ClientAction::Backspace);

        assert_eq!(ui_state.editor_view().text, "ac");
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );
    }

    #[test]
    fn cursor_navigation_clamps_safely_on_single_line_editor() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));

        ui_state.apply_action(ClientAction::MoveCursorLeft);
        ui_state.apply_action(ClientAction::MoveCursorLeft);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 0 }
        );

        ui_state.apply_action(ClientAction::MoveCursorRight);
        ui_state.apply_action(ClientAction::MoveCursorRight);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );

        ui_state.apply_action(ClientAction::MoveCursorUp);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );
        ui_state.apply_action(ClientAction::MoveCursorDown);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );
    }

    #[test]
    fn home_and_end_move_to_line_boundaries() {
        let mut ui_state = ClientUiState::new();
        for ch in "abc".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }

        ui_state.apply_action(ClientAction::MoveLineStart);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 0 }
        );
        ui_state.apply_action(ClientAction::MoveLineEnd);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 3 }
        );
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
