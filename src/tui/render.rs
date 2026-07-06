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
            Constraint::Length(5),
            Constraint::Min(2),
            Constraint::Length(7),
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
        Paragraph::new(Text::from(as_lines(stat_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Session")),
        chunks[2],
    );

    frame.render_widget(
        Paragraph::new("F1 or ? for command panel")
            .block(Block::default().borders(Borders::ALL).title("Mode")),
        chunks[3],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(output_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Output")),
        chunks[4],
    );
}

fn as_lines(values: Vec<String>) -> Vec<Line<'static>> {
    values.into_iter().map(Line::from).collect()
}

fn header_lines(app: &TuiApp) -> Vec<String> {
    let mut lines = vec![
        "Rust Reps".to_string(),
        "Type-based typing practice".to_string(),
    ];

    if app.completed() {
        lines.push("Session complete • Press Esc or Ctrl+C to quit".to_string());
    } else {
        lines.push("Press Esc or Ctrl+C to quit".to_string());
    }

    lines
}

fn drill_lines(app: &TuiApp) -> Vec<String> {
    let drill = app.active_drill();
    vec![
        format!("Drill {}/{}", drill.id, app.session().total_drills()),
        String::new(),
        drill.expected.to_string(),
        String::new(),
        format!("> {}", app.input()),
    ]
}

fn stat_lines(app: &TuiApp) -> Vec<String> {
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

fn output_lines(app: &TuiApp) -> Vec<String> {
    let mut lines = app.status_lines().iter().cloned().collect::<Vec<_>>();
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

#[cfg(test)]
fn render_lines_for_snapshot(app: &TuiApp) -> Vec<String> {
    [
        header_lines(app),
        drill_lines(app),
        stat_lines(app),
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
        assert!(lines.iter().any(|line| line == "PASS"));
        assert!(lines.iter().any(|line| line == "Commands:"));
    }

    #[test]
    fn render_snapshot_includes_expected_and_stats_values() {
        let app = TuiApp::new();
        let lines = render_lines_for_snapshot(&app);
        assert!(lines.iter().any(|line| line == "Skipped: 0"));
        assert!(lines.iter().any(|line| line == "Attempts: 0"));
        assert!(lines.iter().any(|line| line.contains("Drill 001/12")));
    }
}
