use crate::{client::ClientFrame, tui::app::TuiApp};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp) {
    let client_frame = app.frame();
    render_client_frame(frame, &client_frame);
}

fn render_client_frame(frame: &mut Frame<'_>, client_frame: &ClientFrame) {
    let feedback_lines = feedback_chrome_lines(client_frame);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),
            Constraint::Length(feedback_height(&feedback_lines)),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let game_block = Block::default().borders(Borders::ALL);
    let game_area = chunks[0];
    let game_body = game_area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });
    let game_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(game_body);

    frame.render_widget(game_block, game_area);

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(problem_lines(client_frame)))),
        game_chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(workbox_lines(client_frame)))),
        game_chunks[1],
    );

    if !feedback_lines.is_empty() {
        frame.render_widget(
            Paragraph::new(Text::from(as_lines(feedback_lines))),
            chunks[1],
        );
    }

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(bottom_chrome_lines(client_frame))))
            .alignment(Alignment::Center),
        chunks[2],
    );
}

fn as_lines(values: Vec<String>) -> Vec<Line<'static>> {
    values.into_iter().map(Line::from).collect()
}

fn problem_lines(client_frame: &ClientFrame) -> Vec<String> {
    vec![
        client_frame.game_window.problem.instruction.clone(),
        client_frame.game_window.problem.prompt.clone(),
    ]
}

fn workbox_lines(client_frame: &ClientFrame) -> Vec<String> {
    vec![input_line(client_frame)]
}

fn feedback_chrome_lines(client_frame: &ClientFrame) -> Vec<String> {
    let mut lines = client_frame.status_chrome.lines.clone();
    lines.extend(client_frame.help_chrome.lines.clone());
    lines
}

fn feedback_height(lines: &[String]) -> u16 {
    lines.len().min(6) as u16
}

fn input_line(client_frame: &ClientFrame) -> String {
    let cursor = if client_frame.game_window.workbox.cursor_visible {
        "|"
    } else {
        " "
    };
    if client_frame.game_window.workbox.input.is_empty() {
        format!("> {}", cursor)
    } else {
        format!("> {}{}", client_frame.game_window.workbox.input, cursor)
    }
}

fn bottom_chrome_lines(client_frame: &ClientFrame) -> Vec<String> {
    vec![format!("● {}", client_frame.completed_reps_this_run)]
}

#[cfg(test)]
struct RenderSnapshot {
    game: Vec<String>,
    feedback: Vec<String>,
    bottom_chrome: Vec<String>,
}

#[cfg(test)]
fn render_snapshot(client_frame: &ClientFrame) -> RenderSnapshot {
    RenderSnapshot {
        game: [problem_lines(client_frame), workbox_lines(client_frame)].concat(),
        feedback: feedback_chrome_lines(client_frame),
        bottom_chrome: bottom_chrome_lines(client_frame),
    }
}

#[cfg(test)]
mod tests {
    use super::render_snapshot;
    use crate::{client::ClientAction, tui::TuiApp};

    #[test]
    fn render_snapshot_keeps_only_problem_and_workbox_in_game_window() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let instruction_position = snapshot
            .game
            .iter()
            .position(|line| line == "do this:")
            .expect("expected concise problem instruction in game snapshot");
        let prompt_value_position = snapshot
            .game
            .iter()
            .position(|line| line == "fn main() {}")
            .expect("expected prompt in game snapshot");
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == "> |")
            .expect("expected input caret in game snapshot");

        assert!(instruction_position < prompt_value_position);
        assert!(prompt_value_position < input_position);
        assert!(!snapshot.game.iter().any(|line| line == "Problem"));
        assert!(!snapshot.game.iter().any(|line| line == "Workbox"));
        assert!(!snapshot.game.iter().any(|line| line.contains("Drill")));
        assert!(!snapshot.game.iter().any(|line| line == "Feedback"));
        assert!(!snapshot.game.iter().any(|line| line.contains("PASS")));
        assert!(!snapshot.game.iter().any(|line| line.contains("TRY AGAIN")));
    }

    #[test]
    fn render_snapshot_keeps_status_outside_game_window() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let visible_lines: Vec<&String> = snapshot
            .game
            .iter()
            .chain(snapshot.feedback.iter())
            .chain(snapshot.bottom_chrome.iter())
            .collect();
        for forbidden in [
            "Passed",
            "Skipped",
            "Attempts",
            "READY",
            "Enter submit",
            "Esc quit",
            "? help",
        ] {
            assert!(
                !visible_lines.iter().any(|line| line.contains(forbidden)),
                "default render should not contain {forbidden}"
            );
        }
        assert!(snapshot.feedback.is_empty());
    }

    #[test]
    fn render_snapshot_problem_instruction_is_concise() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let descriptor_position = snapshot
            .game
            .iter()
            .position(|line| line == "do this:")
            .expect("expected problem descriptor");
        let prompt_position = snapshot
            .game
            .iter()
            .position(|line| line == "fn main() {}")
            .expect("expected prompt line");

        assert!(descriptor_position < prompt_position);
    }

    #[test]
    fn render_snapshot_contains_bottom_center_counter_text() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        assert_eq!(snapshot.bottom_chrome, vec!["● 0".to_string()]);
        assert!(!snapshot.game.iter().any(|line| line.contains("Passed")));
        assert!(!snapshot.feedback.iter().any(|line| line.contains("Passed")));
    }

    #[test]
    fn render_snapshot_shows_typed_input_text() {
        let mut app = TuiApp::new();
        for ch in "hello".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.game.iter().any(|line| line == "> hello|"));
    }

    #[test]
    fn render_snapshot_hides_cursor_when_cursor_state_is_hidden() {
        let mut app = TuiApp::new();
        app.tick_cursor();
        let frame = app.frame();
        let empty_snapshot = render_snapshot(&frame);
        assert!(empty_snapshot.game.iter().any(|line| line == ">  "));

        app.handle_action(ClientAction::TypeChar('x'));
        app.tick_cursor();
        let frame = app.frame();
        let typed_snapshot = render_snapshot(&frame);
        assert!(typed_snapshot.game.iter().any(|line| line == "> x "));
    }

    #[test]
    fn render_snapshot_shows_help_only_when_toggled() {
        let mut app = TuiApp::new();
        let frame = app.frame();
        let default_snapshot = render_snapshot(&frame);
        assert!(
            !default_snapshot
                .feedback
                .iter()
                .any(|line| line == "Commands:")
        );
        assert!(
            !default_snapshot
                .feedback
                .iter()
                .any(|line| line == "Backspace: delete char")
        );

        app.handle_action(ClientAction::ToggleHelp);
        let frame = app.frame();
        let help_snapshot = render_snapshot(&frame);
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line == "Commands:")
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line == "Enter: submit input")
        );
        assert!(!help_snapshot.game.iter().any(|line| line == "Commands:"));
    }

    #[test]
    fn render_snapshot_makes_try_again_prominent_with_expected_answer() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.feedback.iter().any(|line| line == "TRY AGAIN"));
        assert!(snapshot.feedback.iter().any(|line| line == "Expected:"));
        assert!(snapshot.feedback.iter().any(|line| line == "fn main() {}"));
        assert!(!snapshot.game.iter().any(|line| line == "TRY AGAIN"));
        assert!(!snapshot.game.iter().any(|line| line == "Expected:"));
    }

    #[test]
    fn render_snapshot_makes_unknown_command_clear() {
        let mut app = TuiApp::new();
        for ch in ":unknown".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(
            snapshot
                .feedback
                .iter()
                .any(|line| line == "UNKNOWN COMMAND")
        );
        assert!(
            snapshot
                .feedback
                .iter()
                .any(|line| line == "Type :help for commands.")
        );
        assert!(
            !snapshot
                .game
                .iter()
                .any(|line| line == ">> UNKNOWN COMMAND")
        );
    }

    #[test]
    fn render_snapshot_makes_pass_prominent_and_moves_to_next_drill() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.feedback.iter().any(|line| line == "PASS"));
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line == "println!(\"Rust Reps\");")
        );
        assert!(snapshot.game.iter().any(|line| line == "> |"));
        assert!(snapshot.bottom_chrome.iter().any(|line| line == "● 1"));
        assert!(!snapshot.game.iter().any(|line| line == "PASS"));
    }
}
