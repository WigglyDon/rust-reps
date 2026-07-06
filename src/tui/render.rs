use crate::tui::app::TuiApp;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp) {
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
        Paragraph::new(Text::from(as_lines(problem_lines(app)))),
        game_chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(workbox_lines(app)))),
        game_chunks[1],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(status_chrome_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Status")),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(footer_lines(app)))),
        chunks[2],
    );
}

fn as_lines(values: Vec<String>) -> Vec<Line<'static>> {
    values.into_iter().map(Line::from).collect()
}

fn problem_lines(app: &TuiApp) -> Vec<String> {
    let drill = app.active_drill();
    let drill_label = if app.completed() {
        "Complete".to_string()
    } else {
        format!("Drill {}/{}", drill.id, app.session().total_drills())
    };

    vec![
        "Problem".to_string(),
        drill_label,
        String::new(),
        drill.prompt.to_string(),
    ]
}

fn workbox_lines(app: &TuiApp) -> Vec<String> {
    vec!["Workbox".to_string(), String::new(), input_line(app)]
}

fn status_chrome_lines(app: &TuiApp) -> Vec<String> {
    let mut lines = app
        .status_lines()
        .iter()
        .enumerate()
        .map(|(index, line)| emphasize_status_line(index, line))
        .collect::<Vec<_>>();
    if app.show_help_panel() {
        lines.extend([
            "Commands:".to_string(),
            "Backspace: delete char".to_string(),
            "Enter: submit input".to_string(),
            "Esc / Ctrl+C: quit".to_string(),
            "F1 or ?: toggle this panel".to_string(),
        ]);
    }
    lines
}

fn input_line(app: &TuiApp) -> String {
    if app.input().is_empty() {
        "> |  copy prompt here".to_string()
    } else {
        format!("> {}|", app.input())
    }
}

fn footer_lines(app: &TuiApp) -> Vec<String> {
    vec![
        format!(
            "Passed {}/{} | Skipped {} | Attempts {}",
            app.session().passed(),
            app.session().total_drills(),
            app.session().skipped(),
            app.session().attempts()
        ),
        "Enter submit | Esc quit | ? help".to_string(),
    ]
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

#[cfg(test)]
struct RenderSnapshot {
    game: Vec<String>,
    status: Vec<String>,
    footer: Vec<String>,
}

#[cfg(test)]
fn render_snapshot(app: &TuiApp) -> RenderSnapshot {
    RenderSnapshot {
        game: [problem_lines(app), workbox_lines(app)].concat(),
        status: status_chrome_lines(app),
        footer: footer_lines(app),
    }
}

#[cfg(test)]
mod tests {
    use super::render_snapshot;
    use crate::tui::{TuiApp, UiIntent};

    #[test]
    fn render_snapshot_keeps_only_problem_and_workbox_in_game_window() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app);
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
            .position(|line| line == "> |  copy prompt here")
            .expect("expected input caret in game snapshot");

        assert!(problem_position < drill_position);
        assert!(drill_position < prompt_position);
        assert!(prompt_position < workbox_position);
        assert!(workbox_position < input_position);
        assert!(!snapshot.game.iter().any(|line| line == "Feedback"));
        assert!(!snapshot.game.iter().any(|line| line.contains("PASS")));
        assert!(!snapshot.game.iter().any(|line| line.contains("TRY AGAIN")));
    }

    #[test]
    fn render_snapshot_keeps_status_outside_game_window() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app);
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
                .any(|line| line == "Copy the prompt exactly.")
        );
    }

    #[test]
    fn render_snapshot_keeps_stats_and_help_in_footer_chrome() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app);
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
            app.handle_intent(UiIntent::Append(ch));
        }
        let snapshot = render_snapshot(&app);

        assert!(snapshot.game.iter().any(|line| line == "> hello|"));
    }

    #[test]
    fn render_snapshot_shows_help_only_when_toggled() {
        let mut app = TuiApp::new();
        let default_snapshot = render_snapshot(&app);
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

        app.handle_intent(UiIntent::ToggleHelp);
        let help_snapshot = render_snapshot(&app);
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
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);
        let snapshot = render_snapshot(&app);

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
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);
        let snapshot = render_snapshot(&app);

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
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);
        let snapshot = render_snapshot(&app);

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
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line == "> |  copy prompt here")
        );
        assert!(!snapshot.game.iter().any(|line| line == ">> PASS"));
    }
}
