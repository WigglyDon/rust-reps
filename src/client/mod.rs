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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientUiState {
    draft_input: String,
    cursor_visible: bool,
    help_open: bool,
    status_lines: Vec<String>,
}

impl ClientUiState {
    pub fn new() -> Self {
        Self {
            draft_input: String::new(),
            cursor_visible: true,
            help_open: false,
            status_lines: vec!["Waiting for input.".to_string()],
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientFrame {
    pub game_window: GameWindowFrame,
    pub status_chrome: StatusChromeFrame,
    pub progress_chrome: ProgressChromeFrame,
    pub help_chrome: HelpChromeFrame,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameWindowFrame {
    pub problem: ProblemView,
    pub workbox: WorkboxView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemView {
    pub title: String,
    pub progress_label: String,
    pub instruction: String,
    pub prompt: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkboxView {
    pub title: String,
    pub input: String,
    pub cursor_visible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusChromeFrame {
    pub lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressChromeFrame {
    pub line: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpChromeFrame {
    pub default_hint: String,
    pub open: bool,
    pub lines: Vec<String>,
}

pub fn project_client_frame(
    session: &engine::Session,
    ui_state: &ClientUiState,
    completed: bool,
) -> ClientFrame {
    let snapshot = session.snapshot();
    let exercise = snapshot
        .exercise
        .unwrap_or_else(engine::first_exercise_packet);
    let progress_label = if completed || snapshot.is_complete {
        "Complete".to_string()
    } else {
        format!("Drill {}/{}", exercise.id, exercise.total)
    };

    ClientFrame {
        game_window: GameWindowFrame {
            problem: ProblemView {
                title: "Problem".to_string(),
                progress_label,
                instruction: exercise.instruction.to_string(),
                prompt: exercise.prompt.to_string(),
            },
            workbox: WorkboxView {
                title: "Workbox".to_string(),
                input: ui_state.draft_input().to_string(),
                cursor_visible: ui_state.cursor_visible(),
            },
        },
        status_chrome: StatusChromeFrame {
            lines: ui_state
                .status_lines()
                .iter()
                .enumerate()
                .map(|(index, line)| emphasize_status_line(index, line))
                .collect(),
        },
        progress_chrome: ProgressChromeFrame {
            line: format!(
                "Passed {}/{} | Skipped {} | Attempts {}",
                snapshot.stats.passed,
                snapshot.stats.total_drills,
                snapshot.stats.skipped,
                snapshot.stats.attempts
            ),
        },
        help_chrome: HelpChromeFrame {
            default_hint: "Enter submit | Esc quit | ? help".to_string(),
            open: ui_state.help_open(),
            lines: help_lines(ui_state.help_open()),
        },
    }
}

fn emphasize_status_line(index: usize, line: &str) -> String {
    if index == 0 {
        match line {
            "PASS" | "TRY AGAIN" | "UNKNOWN COMMAND" | "COMPLETE" | "QUIT" | "STATUS" | "SKIP" => {
                format!(">> {}", line)
            }
            _ => line.to_string(),
        }
    } else {
        line.to_string()
    }
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
    use super::{ClientAction, ClientUiState, project_client_frame};
    use crate::engine;

    #[test]
    fn frame_projection_includes_problem_prompt() {
        let session = engine::Session::new();
        let ui_state = ClientUiState::new();

        let frame = project_client_frame(&session, &ui_state, false);

        assert_eq!(frame.game_window.problem.title, "Problem");
        assert_eq!(frame.game_window.problem.progress_label, "Drill 001/12");
        assert_eq!(
            frame.game_window.problem.instruction,
            "Copy this line exactly:"
        );
        assert_eq!(frame.game_window.problem.prompt, "fn main() {}");
    }

    #[test]
    fn frame_projection_uses_exercise_packet_position_and_prompt() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        assert!(matches!(
            session.submit_answer("fn main() {}"),
            engine::SubmitResult::Correct
        ));
        let frame = project_client_frame(&session, &ui_state, false);

        assert_eq!(frame.game_window.problem.progress_label, "Drill 002/12");
        assert_eq!(frame.game_window.problem.prompt, "println!(\"Rust Reps\");");
    }

    #[test]
    fn frame_projection_includes_workbox_input_and_cursor_state() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        ui_state.apply_action(ClientAction::CursorTick);

        let frame = project_client_frame(&session, &ui_state, false);

        assert_eq!(frame.game_window.workbox.title, "Workbox");
        assert_eq!(frame.game_window.workbox.input, "x");
        assert!(!frame.game_window.workbox.cursor_visible);
    }

    #[test]
    fn frame_projection_includes_compact_progress_stats() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();
        let event = session.submit_answer("wrong");

        assert!(matches!(event, engine::SubmitResult::TryAgain));
        let frame = project_client_frame(&session, &ui_state, false);

        assert_eq!(
            frame.progress_chrome.line,
            "Passed 0/12 | Skipped 0 | Attempts 1"
        );
    }

    #[test]
    fn frame_projection_keeps_status_result_outside_game_window() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.set_status_lines(vec![
            "TRY AGAIN".to_string(),
            "Expected:".to_string(),
            "fn main() {}".to_string(),
        ]);

        let frame = project_client_frame(&session, &ui_state, false);

        assert_eq!(
            frame.status_chrome.lines,
            vec![
                ">> TRY AGAIN".to_string(),
                "Expected:".to_string(),
                "fn main() {}".to_string(),
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
