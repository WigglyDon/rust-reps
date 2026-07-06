use crate::tui::app::TuiApp;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(2),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(header_lines(app)))).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Rust Reps TUI"),
        ),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(drill_lines(app)))).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Drill"),
        ),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(session_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Session")),
        chunks[2],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(input_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Input")),
        chunks[3],
    );

    frame.render_widget(
        Paragraph::new("? / F1 help").block(Block::default().borders(Borders::ALL).title("Mode")),
        chunks[4],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(output_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Output")),
        chunks[5],
    );
}

fn as_lines(values: Vec<String>) -> Vec<Line<'static>> {
    values.into_iter().map(Line::from).collect()
}

fn header_lines(app: &TuiApp) -> Vec<String> {
    let state = if app.completed() {
        "Complete"
    } else {
        "Practice"
    };
    vec!["Rust Reps".to_string(), state.to_string()]
}

fn drill_lines(app: &TuiApp) -> Vec<String> {
    let drill = app.active_drill();
    vec![
        format!("Drill {}/{}", drill.id, app.session().total_drills()),
        String::new(),
        drill.expected.to_string(),
    ]
}

fn session_lines(app: &TuiApp) -> Vec<String> {
    vec![
        format!(
            "Passed: {}/{}",
            app.session().passed(),
            app.session().total_drills()
        ),
        format!("Skipped: {}", app.session().skipped()),
        format!("Attempts: {}", app.session().attempts()),
    ]
}

fn input_lines(app: &TuiApp) -> Vec<String> {
    let input_line = if app.input().is_empty() {
        "> |  type answer".to_string()
    } else {
        format!("> {}|", app.input())
    };

    vec!["Input focus".to_string(), input_line]
}

fn output_lines(app: &TuiApp) -> Vec<String> {
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

fn emphasize_status_line(index: usize, line: &str) -> String {
    if index == 0 {
        match line {
            "PASS" | "TRY AGAIN" | "UNKNOWN COMMAND" | "COMPLETE" | "QUIT" | "STATUS" | "READY"
            | "SKIP" => format!(">> {}", line),
            _ => line.to_string(),
        }
    } else {
        line.to_string()
    }
}

#[cfg(test)]
fn render_lines_for_snapshot(app: &TuiApp) -> Vec<String> {
    [
        header_lines(app),
        drill_lines(app),
        session_lines(app),
        input_lines(app),
        output_lines(app),
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::render_lines_for_snapshot;
    use crate::tui::{TuiApp, UiIntent};

    #[test]
    fn render_snapshot_contains_title_drill_stats_and_last_status() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);

        let lines = render_lines_for_snapshot(&app);
        assert!(lines.iter().any(|line| line == "Rust Reps"));
        assert!(lines.iter().any(|line| line.contains("Drill 002/12")));
        assert!(
            lines
                .iter()
                .any(|line| line.contains("println!(\"Rust Reps\");"))
        );
        assert!(lines.iter().any(|line| line == "Passed: 1/12"));
        assert!(lines.iter().any(|line| line == ">> PASS"));
        assert!(!lines.iter().any(|line| line == "Commands:"));
        assert!(lines.iter().any(|line| line == "Input focus"));
        assert!(lines.iter().any(|line| line == "> |  type answer"));
    }

    #[test]
    fn render_snapshot_includes_expected_and_stats_values() {
        let app = TuiApp::new();
        let lines = render_lines_for_snapshot(&app);
        assert!(lines.iter().any(|line| line == "Skipped: 0"));
        assert!(lines.iter().any(|line| line == "Attempts: 0"));
        assert!(lines.iter().any(|line| line.contains("Drill 001/12")));
        assert!(lines.iter().any(|line| line == "> |  type answer"));
        assert!(lines.iter().any(|line| line == "Input focus"));
        assert!(lines.iter().any(|line| line == ">> READY"));
    }

    #[test]
    fn render_snapshot_shows_typed_input_text() {
        let mut app = TuiApp::new();
        for ch in "hello".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        let lines = render_lines_for_snapshot(&app);

        assert!(lines.iter().any(|line| line == "Input focus"));
        assert!(lines.iter().any(|line| line == "> hello|"));
    }

    #[test]
    fn render_snapshot_shows_help_only_when_toggled() {
        let mut app = TuiApp::new();
        let default_lines = render_lines_for_snapshot(&app);
        assert!(!default_lines.iter().any(|line| line == "Commands:"));
        assert!(
            !default_lines
                .iter()
                .any(|line| line == "Backspace: delete char")
        );

        app.handle_intent(UiIntent::ToggleHelp);
        let help_lines = render_lines_for_snapshot(&app);
        assert!(help_lines.iter().any(|line| line == "Commands:"));
        assert!(help_lines.iter().any(|line| line == "Enter: submit input"));
    }

    #[test]
    fn render_snapshot_makes_try_again_prominent_with_expected_answer() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);
        let lines = render_lines_for_snapshot(&app);

        assert!(lines.iter().any(|line| line == ">> TRY AGAIN"));
        assert!(lines.iter().any(|line| line == "Expected:"));
        assert!(lines.iter().any(|line| line == "fn main() {}"));
    }
}
