use crate::{client::ClientFrame, tui::app::TuiApp};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp) {
    let client_frame = app.frame();
    render_client_frame(frame, &client_frame);
}

fn render_client_frame(frame: &mut Frame<'_>, client_frame: &ClientFrame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),
            Constraint::Length(7),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let game_block = Block::default().borders(Borders::ALL).title("Game");
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

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(status_chrome_lines(client_frame))))
            .block(Block::default().borders(Borders::ALL).title("Status")),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(footer_lines(client_frame)))),
        chunks[2],
    );
}

fn as_lines(values: Vec<String>) -> Vec<Line<'static>> {
    values.into_iter().map(Line::from).collect()
}

fn problem_lines(client_frame: &ClientFrame) -> Vec<String> {
    vec![
        client_frame.game_window.problem.title.clone(),
        client_frame.game_window.problem.progress_label.clone(),
        String::new(),
        client_frame.game_window.problem.instruction.clone(),
        client_frame.game_window.problem.prompt.clone(),
    ]
}

fn workbox_lines(client_frame: &ClientFrame) -> Vec<String> {
    vec![
        client_frame.game_window.workbox.title.clone(),
        String::new(),
        input_line(client_frame),
    ]
}

fn status_chrome_lines(client_frame: &ClientFrame) -> Vec<String> {
    let mut lines = client_frame.status_chrome.lines.clone();
    lines.extend(client_frame.help_chrome.lines.clone());
    lines
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

fn footer_lines(client_frame: &ClientFrame) -> Vec<String> {
    vec![
        client_frame.progress_chrome.line.clone(),
        client_frame.help_chrome.default_hint.clone(),
    ]
}

#[cfg(test)]
struct RenderSnapshot {
    game: Vec<String>,
    status: Vec<String>,
    footer: Vec<String>,
}

#[cfg(test)]
fn render_snapshot(client_frame: &ClientFrame) -> RenderSnapshot {
    RenderSnapshot {
        game: [problem_lines(client_frame), workbox_lines(client_frame)].concat(),
        status: status_chrome_lines(client_frame),
        footer: footer_lines(client_frame),
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
        let problem_position = snapshot
            .game
            .iter()
            .position(|line| line == "Problem")
            .expect("expected Problem label in game snapshot");
        let drill_position = snapshot
            .game
            .iter()
            .position(|line| line.contains("Drill 001/12"))
            .expect("expected drill in game snapshot");
        let prompt_position = snapshot
            .game
            .iter()
            .position(|line| line == "Copy this line exactly:")
            .expect("expected problem descriptor in game snapshot");
        let prompt_value_position = snapshot
            .game
            .iter()
            .position(|line| line == "fn main() {}")
            .expect("expected prompt in game snapshot");
        let workbox_position = snapshot
            .game
            .iter()
            .position(|line| line == "Workbox")
            .expect("expected Workbox label in game snapshot");
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == "> |")
            .expect("expected input caret in game snapshot");

        assert!(problem_position < drill_position);
        assert!(drill_position < prompt_position);
        assert!(prompt_position < prompt_value_position);
        assert!(prompt_value_position < workbox_position);
        assert!(workbox_position < input_position);
        assert!(!snapshot.game.iter().any(|line| line == "Feedback"));
        assert!(!snapshot.game.iter().any(|line| line.contains("PASS")));
        assert!(!snapshot.game.iter().any(|line| line.contains("TRY AGAIN")));
    }

    #[test]
    fn render_snapshot_keeps_status_outside_game_window() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        assert!(!snapshot.game.iter().any(|line| line.contains("READY")));
        assert!(
            !snapshot
                .game
                .iter()
                .any(|line| line == "Copy the prompt exactly.")
        );
        assert!(
            snapshot
                .status
                .iter()
                .any(|line| line == "Waiting for input.")
        );
    }

    #[test]
    fn render_snapshot_problem_descriptor_precedes_prompt_line() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let descriptor_position = snapshot
            .game
            .iter()
            .position(|line| line == "Copy this line exactly:")
            .expect("expected problem descriptor");
        let prompt_position = snapshot
            .game
            .iter()
            .position(|line| line == "fn main() {}")
            .expect("expected prompt line");

        assert!(descriptor_position < prompt_position);
    }

    #[test]
    fn render_snapshot_keeps_stats_and_help_in_footer_chrome() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        assert!(
            snapshot
                .footer
                .iter()
                .any(|line| line == "Passed 0/12 | Skipped 0 | Attempts 0")
        );
        assert!(
            snapshot
                .footer
                .iter()
                .any(|line| line == "Enter submit | Esc quit | ? help")
        );
        assert!(
            !snapshot
                .game
                .iter()
                .any(|line| line.contains("Passed 0/12"))
        );
        assert!(!snapshot.status.iter().any(|line| line == "Commands:"));
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
                .status
                .iter()
                .any(|line| line == "Commands:")
        );
        assert!(
            !default_snapshot
                .status
                .iter()
                .any(|line| line == "Backspace: delete char")
        );

        app.handle_action(ClientAction::ToggleHelp);
        let frame = app.frame();
        let help_snapshot = render_snapshot(&frame);
        assert!(help_snapshot.status.iter().any(|line| line == "Commands:"));
        assert!(
            help_snapshot
                .status
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

        assert!(snapshot.status.iter().any(|line| line == ">> TRY AGAIN"));
        assert!(snapshot.status.iter().any(|line| line == "Expected:"));
        assert!(snapshot.status.iter().any(|line| line == "fn main() {}"));
        assert!(!snapshot.game.iter().any(|line| line == ">> TRY AGAIN"));
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
                .status
                .iter()
                .any(|line| line == ">> UNKNOWN COMMAND")
        );
        assert!(
            snapshot
                .status
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

        assert!(snapshot.status.iter().any(|line| line == ">> PASS"));
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line.contains("Drill 002/12"))
        );
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line.contains("println!(\"Rust Reps\");"))
        );
        assert!(snapshot.game.iter().any(|line| line == "> |"));
        assert!(!snapshot.game.iter().any(|line| line == ">> PASS"));
    }
}
