use crate::{
    client::{ClientFrame, CodeToken, CodeTokenKind, StatusLine, StatusTone},
    tui::app::TuiApp,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone, Copy)]
struct TuiTheme {
    background: Color,
    panel: Color,
    border: Color,
    text: Color,
    muted: Color,
    keyword: Color,
    type_name: Color,
    number: Color,
    string: Color,
    macro_name: Color,
    punctuation: Color,
    success: Color,
    success_bright: Color,
    danger: Color,
    warning: Color,
    caret: Color,
}

fn theme() -> TuiTheme {
    TuiTheme {
        background: Color::Rgb(8, 10, 14),
        panel: Color::Rgb(12, 15, 21),
        border: Color::Rgb(62, 70, 84),
        text: Color::Rgb(232, 237, 246),
        muted: Color::Rgb(119, 129, 148),
        keyword: Color::Rgb(126, 169, 255),
        type_name: Color::Rgb(106, 220, 190),
        number: Color::Rgb(236, 196, 126),
        string: Color::Rgb(157, 218, 139),
        macro_name: Color::Rgb(200, 168, 255),
        punctuation: Color::Rgb(150, 159, 176),
        success: Color::Rgb(88, 224, 135),
        success_bright: Color::Rgb(158, 255, 187),
        danger: Color::Rgb(255, 92, 105),
        warning: Color::Rgb(235, 178, 86),
        caret: Color::Rgb(116, 241, 157),
    }
}

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp) {
    let client_frame = app.frame();
    render_client_frame(frame, &client_frame);
}

fn render_client_frame(frame: &mut Frame<'_>, client_frame: &ClientFrame) {
    let theme = theme();
    let feedback_lines = feedback_chrome_lines(client_frame);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),
            Constraint::Length(feedback_height(&feedback_lines)),
            Constraint::Length(1),
        ])
        .split(frame.area());

    frame.render_widget(Block::default().style(base_style()), frame.area());

    let game_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.panel))
        .border_style(border_style());
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
        Paragraph::new(Text::from(problem_lines(client_frame))).style(base_style()),
        game_chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(workbox_lines(client_frame))).style(base_style()),
        game_chunks[1],
    );

    if !feedback_lines.is_empty() {
        frame.render_widget(
            Paragraph::new(Text::from(feedback_lines)).style(base_style()),
            chunks[1],
        );
    }

    frame.render_widget(
        Paragraph::new(Text::from(bottom_chrome_lines(client_frame)))
            .alignment(Alignment::Center)
            .style(base_style()),
        chunks[2],
    );
}

fn base_style() -> Style {
    Style::default().fg(theme().text).bg(theme().background)
}

fn border_style() -> Style {
    Style::default().fg(theme().border).bg(theme().panel)
}

fn problem_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled(
            client_frame.game_window.problem.instruction.clone(),
            muted_style(),
        )),
        Line::from(code_spans(&client_frame.game_window.problem.prompt_tokens)),
    ]
}

fn workbox_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![Line::from(input_spans(client_frame))]
}

fn feedback_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    let mut lines = client_frame.status_chrome.lines.clone();
    lines.extend(
        client_frame
            .help_chrome
            .lines
            .iter()
            .map(|line| StatusLine {
                tone: StatusTone::Plain,
                text: line.to_string(),
                code_tokens: Vec::new(),
            }),
    );
    lines.into_iter().map(feedback_line).collect()
}

fn feedback_height(lines: &[Line<'static>]) -> u16 {
    lines.len().min(6) as u16
}

fn input_spans(client_frame: &ClientFrame) -> Vec<Span<'static>> {
    let cursor = if client_frame.game_window.workbox.cursor_visible {
        "|"
    } else {
        " "
    };
    let mut spans = vec![Span::styled("> ", prompt_marker_style())];
    spans.extend(code_spans(&client_frame.game_window.workbox.input_tokens));
    spans.push(Span::styled(cursor.to_string(), caret_style()));
    spans
}

fn feedback_line(status_line: StatusLine) -> Line<'static> {
    if !status_line.code_tokens.is_empty() {
        return Line::from(code_spans(&status_line.code_tokens));
    }

    Line::from(Span::styled(
        status_line.text,
        status_tone_style(status_line.tone),
    ))
}

fn bottom_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![Line::from(vec![
        Span::styled("●", counter_dot_style(client_frame.counter_pulse_active)),
        Span::styled(
            format!(" {}", client_frame.completed_reps_this_run),
            counter_number_style(),
        ),
    ])]
}

fn code_spans(tokens: &[CodeToken]) -> Vec<Span<'static>> {
    tokens
        .iter()
        .map(|token| Span::styled(token.text.clone(), code_token_style(token.kind)))
        .collect()
}

fn code_token_style(kind: CodeTokenKind) -> Style {
    let theme = theme();
    match kind {
        CodeTokenKind::Keyword => Style::default()
            .fg(theme.keyword)
            .bg(theme.panel)
            .add_modifier(Modifier::BOLD),
        CodeTokenKind::Type => Style::default().fg(theme.type_name).bg(theme.panel),
        CodeTokenKind::Number => Style::default().fg(theme.number).bg(theme.panel),
        CodeTokenKind::StringLiteral => Style::default().fg(theme.string).bg(theme.panel),
        CodeTokenKind::Macro => Style::default()
            .fg(theme.macro_name)
            .bg(theme.panel)
            .add_modifier(Modifier::BOLD),
        CodeTokenKind::Identifier | CodeTokenKind::Plain => {
            Style::default().fg(theme.text).bg(theme.panel)
        }
        CodeTokenKind::Punctuation => Style::default().fg(theme.punctuation).bg(theme.panel),
        CodeTokenKind::Whitespace => Style::default().fg(theme.text).bg(theme.panel),
        CodeTokenKind::Comment => Style::default()
            .fg(theme.muted)
            .bg(theme.panel)
            .add_modifier(Modifier::ITALIC),
    }
}

fn status_tone_style(tone: StatusTone) -> Style {
    let theme = theme();
    match tone {
        StatusTone::Pass => Style::default()
            .fg(theme.success)
            .bg(theme.background)
            .add_modifier(Modifier::BOLD),
        StatusTone::Complete => Style::default()
            .fg(theme.type_name)
            .bg(theme.background)
            .add_modifier(Modifier::BOLD),
        StatusTone::TryAgain => Style::default()
            .fg(theme.danger)
            .bg(theme.background)
            .add_modifier(Modifier::BOLD),
        StatusTone::ExpectedLabel => Style::default().fg(theme.warning).bg(theme.background),
        StatusTone::UnknownCommand | StatusTone::Warning => {
            Style::default().fg(theme.warning).bg(theme.background)
        }
        StatusTone::Plain => muted_style(),
    }
}

fn counter_dot_style(pulsing: bool) -> Style {
    if pulsing {
        return Style::default()
            .fg(theme().success_bright)
            .bg(theme().background)
            .add_modifier(Modifier::BOLD);
    }

    Style::default().fg(theme().success).bg(theme().background)
}

fn counter_number_style() -> Style {
    Style::default().fg(theme().success).bg(theme().background)
}

fn caret_style() -> Style {
    Style::default()
        .fg(theme().caret)
        .bg(theme().panel)
        .add_modifier(Modifier::BOLD)
}

fn muted_style() -> Style {
    Style::default().fg(theme().muted).bg(theme().panel)
}

fn prompt_marker_style() -> Style {
    Style::default().fg(theme().muted).bg(theme().panel)
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
        game: [problem_lines(client_frame), workbox_lines(client_frame)]
            .concat()
            .iter()
            .map(line_text)
            .collect(),
        feedback: feedback_chrome_lines(client_frame)
            .iter()
            .map(line_text)
            .collect(),
        bottom_chrome: bottom_chrome_lines(client_frame)
            .iter()
            .map(line_text)
            .collect(),
    }
}

#[cfg(test)]
fn line_text(line: &Line<'_>) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{
        bottom_chrome_lines, feedback_chrome_lines, line_text, problem_lines, render_snapshot,
        status_tone_style, theme,
    };
    use crate::{
        client::{ClientAction, StatusTone},
        tui::TuiApp,
    };
    use ratatui::style::Modifier;

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
    fn counter_dot_and_number_are_green() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");

        assert_eq!(line.spans[0].content.as_ref(), "●");
        assert_eq!(line.spans[0].style.fg, Some(theme().success));
        assert!(!line.spans[0].style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(line.spans[1].content.as_ref(), " 0");
        assert_eq!(line.spans[1].style.fg, Some(theme().success));
    }

    #[test]
    fn pulsing_counter_dot_is_brighter_and_bold_while_number_stays_stable() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");

        assert_eq!(line.spans[0].content.as_ref(), "●");
        assert_eq!(line.spans[0].style.fg, Some(theme().success_bright));
        assert!(line.spans[0].style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(line.spans[1].content.as_ref(), " 1");
        assert_eq!(line.spans[1].style.fg, Some(theme().success));
    }

    #[test]
    fn status_theme_maps_pass_try_again_and_unknown_command_distinctly() {
        assert_eq!(
            status_tone_style(StatusTone::Pass).fg,
            Some(theme().success)
        );
        assert_eq!(
            status_tone_style(StatusTone::TryAgain).fg,
            Some(theme().danger)
        );
        assert_eq!(
            status_tone_style(StatusTone::UnknownCommand).fg,
            Some(theme().warning)
        );
        assert_ne!(
            status_tone_style(StatusTone::Pass).fg,
            status_tone_style(StatusTone::TryAgain).fg
        );
    }

    #[test]
    fn try_again_feedback_renders_red() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let lines = feedback_chrome_lines(&frame);
        let try_again = lines
            .iter()
            .find(|line| line_text(line) == "TRY AGAIN")
            .expect("expected TRY AGAIN feedback");

        assert_eq!(try_again.spans[0].style.fg, Some(theme().danger));
    }

    #[test]
    fn code_prompt_renders_with_non_default_syntax_styles() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = problem_lines(&frame);
        let prompt_line = lines.get(1).expect("expected prompt line");

        assert_eq!(line_text(prompt_line), "fn main() {}");
        assert!(
            prompt_line
                .spans
                .iter()
                .any(|span| span.content.as_ref() == "fn"
                    && span.style.fg == Some(theme().keyword))
        );
        assert!(
            prompt_line
                .spans
                .iter()
                .any(|span| span.style != super::base_style())
        );
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
