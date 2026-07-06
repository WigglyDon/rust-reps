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
        .constraints([Constraint::Min(12), Constraint::Length(4)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(rep_lines(app))))
            .block(Block::default().borders(Borders::ALL).title("Rep")),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(as_lines(footer_lines(app)))),
        chunks[1],
    );
}

fn as_lines(values: Vec<String>) -> Vec<Line<'static>> {
    values.into_iter().map(Line::from).collect()
}

fn rep_lines(app: &TuiApp) -> Vec<String> {
    let drill = app.active_drill();
    let drill_label = if app.completed() {
        "Complete".to_string()
    } else {
        format!("Drill {}/{}", drill.id, app.session().total_drills())
    };
    let mut lines = vec![
        drill_label,
        String::new(),
        "Prompt".to_string(),
        drill.prompt.to_string(),
        String::new(),
        "Workbox".to_string(),
        input_line(app),
        String::new(),
        "Feedback".to_string(),
    ];
    lines.extend(status_lines(app));
    lines
}

fn status_lines(app: &TuiApp) -> Vec<String> {
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
fn render_lines_for_snapshot(app: &TuiApp) -> Vec<String> {
    [rep_lines(app), footer_lines(app)].concat()
}

#[cfg(test)]
mod tests {
    use super::render_lines_for_snapshot;
    use crate::tui::{TuiApp, UiIntent};

    #[test]
    fn render_snapshot_keeps_drill_input_and_status_in_primary_rep() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);

        let lines = render_lines_for_snapshot(&app);
        let drill_position = lines
            .iter()
            .position(|line| line.contains("Drill 002/12"))
            .expect("expected drill in rep snapshot");
        let expected_position = lines
            .iter()
            .position(|line| line == "Prompt")
            .expect("expected Prompt label in rep snapshot");
        let prompt_value_position = lines
            .iter()
            .position(|line| line.contains("println!(\"Rust Reps\");"))
            .expect("expected prompt in rep snapshot");
        let workbox_position = lines
            .iter()
            .position(|line| line == "Workbox")
            .expect("expected Workbox label in rep snapshot");
        let input_position = lines
            .iter()
            .position(|line| line == "> |  copy prompt here")
            .expect("expected input caret in rep snapshot");
        let feedback_position = lines
            .iter()
            .position(|line| line == "Feedback")
            .expect("expected Feedback label in rep snapshot");
        let status_position = lines
            .iter()
            .position(|line| line == ">> PASS")
            .expect("expected PASS in rep snapshot");

        assert!(drill_position < expected_position);
        assert!(expected_position < prompt_value_position);
        assert!(prompt_value_position < workbox_position);
        assert!(workbox_position < input_position);
        assert!(input_position < feedback_position);
        assert!(feedback_position < status_position);
        assert!(!lines.iter().any(|line| line == "Commands:"));
    }

    #[test]
    fn render_snapshot_uses_task_language_by_default() {
        let app = TuiApp::new();
        let lines = render_lines_for_snapshot(&app);
        assert!(!lines.iter().any(|line| line.contains("READY")));
        assert!(lines.iter().any(|line| line == "Prompt"));
        assert!(lines.iter().any(|line| line == "Workbox"));
        assert!(lines.iter().any(|line| line == "Feedback"));
        assert!(lines.iter().any(|line| line == "Copy the prompt exactly."));
        assert!(lines.iter().any(|line| line == "> |  copy prompt here"));
    }

    #[test]
    fn render_snapshot_uses_compact_footer_by_default() {
        let app = TuiApp::new();
        let lines = render_lines_for_snapshot(&app);
        assert!(
            lines
                .iter()
                .any(|line| line == "Passed 0/12 | Skipped 0 | Attempts 0")
        );
        assert!(
            lines
                .iter()
                .any(|line| line == "Enter submit | Esc quit | ? help")
        );
        assert!(!lines.iter().any(|line| line == "Backspace: delete char"));
    }

    #[test]
    fn render_snapshot_shows_typed_input_text() {
        let mut app = TuiApp::new();
        for ch in "hello".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        let lines = render_lines_for_snapshot(&app);

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

    #[test]
    fn render_snapshot_makes_unknown_command_clear() {
        let mut app = TuiApp::new();
        for ch in ":unknown".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);
        let lines = render_lines_for_snapshot(&app);

        assert!(lines.iter().any(|line| line == ">> UNKNOWN COMMAND"));
        assert!(lines.iter().any(|line| line == "Type :help for commands."));
    }

    #[test]
    fn render_snapshot_makes_pass_prominent_and_moves_to_next_drill() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);
        let lines = render_lines_for_snapshot(&app);

        assert!(lines.iter().any(|line| line == ">> PASS"));
        assert!(lines.iter().any(|line| line.contains("Drill 002/12")));
        assert!(
            lines
                .iter()
                .any(|line| line.contains("println!(\"Rust Reps\");"))
        );
        assert!(lines.iter().any(|line| line == "> |  copy prompt here"));
    }
}
