use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{
    cursor::Show,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::{catalog, practice};

pub fn run_tui() -> io::Result<()> {
    let mut terminal = enter_tui()?;
    let loop_result = run_tui_loop(&mut terminal);
    let cleanup_result = disable_tui();

    match loop_result {
        Ok(()) => cleanup_result,
        Err(loop_err) => {
            if cleanup_result.is_err() {
                cleanup_result
            } else {
                Err(loop_err)
            }
        }
    }
}

fn run_tui_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut state = TuiState::new();

    loop {
        terminal.draw(|frame| render_ui(frame, &state))?;

        if state.should_quit {
            break;
        }

        let key = match event::read() {
            Ok(Event::Key(key)) => key,
            Ok(_) => continue,
            Err(err) => return Err(err),
        };

        if handle_key_event(&mut state, key) {
            break;
        }
    }

    Ok(())
}

struct TuiState {
    session: practice::PracticeSession,
    input: String,
    status_lines: Vec<String>,
    show_help_panel: bool,
    should_quit: bool,
    completed: bool,
}

impl TuiState {
    fn new() -> Self {
        Self {
            session: practice::PracticeSession::new(),
            input: String::new(),
            status_lines: vec!["Type an exact answer and press Enter".to_string()],
            show_help_panel: true,
            should_quit: false,
            completed: false,
        }
    }

    fn active_drill(&self) -> &'static catalog::Drill {
        self.session
            .current_drill()
            .unwrap_or(&catalog::drills()[0])
    }
}

fn handle_key_event(state: &mut TuiState, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => {
            state.should_quit = true;
            state.status_lines = collect_summary_lines("QUIT", &state.session);
            true
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.should_quit = true;
            state.status_lines = collect_summary_lines("QUIT", &state.session);
            true
        }
        KeyCode::Char('?') => {
            state.show_help_panel = !state.show_help_panel;
            false
        }
        KeyCode::F(1) => {
            state.show_help_panel = !state.show_help_panel;
            false
        }
        KeyCode::Backspace => {
            state.input.pop();
            false
        }
        KeyCode::Enter => {
            if state.completed {
                return false;
            }

            let submitted = state.input.clone();
            state.input.clear();
            state.status_lines = match state.session.submit_input(&submitted) {
                practice::PracticeEvent::Quit => {
                    state.should_quit = true;
                    collect_summary_lines("QUIT", &state.session)
                }
                practice::PracticeEvent::Help => vec![
                    ":help    show commands".to_string(),
                    ":status  show session status".to_string(),
                    ":list    show drills".to_string(),
                    ":skip    skip current drill".to_string(),
                    ":q       quit".to_string(),
                    ":quit    quit".to_string(),
                ],
                practice::PracticeEvent::Status => vec![
                    format!(
                        "Current: {}/{}",
                        state.active_drill().id,
                        state.session.total_drills()
                    ),
                    format!(
                        "Passed: {}/{}",
                        state.session.passed(),
                        state.session.total_drills()
                    ),
                    format!("Skipped: {}", state.session.skipped()),
                    format!("Attempts: {}", state.session.attempts()),
                ],
                practice::PracticeEvent::List => catalog::drills()
                    .iter()
                    .map(|drill| format!("{}  {}", drill.id, drill.expected))
                    .collect(),
                practice::PracticeEvent::Skip => {
                    if state.session.is_complete() {
                        state.completed = true;
                        collect_complete_lines(&state.session)
                    } else {
                        vec!["SKIP".to_string()]
                    }
                }
                practice::PracticeEvent::UnknownCommand => {
                    vec![
                        "UNKNOWN COMMAND".to_string(),
                        "Type :help for commands.".to_string(),
                    ]
                }
                practice::PracticeEvent::Correct => {
                    if state.session.is_complete() {
                        state.completed = true;
                        let mut lines = Vec::from(["PASS".to_string()]);
                        lines.extend(collect_complete_lines(&state.session));
                        lines
                    } else {
                        vec!["PASS".to_string()]
                    }
                }
                practice::PracticeEvent::TryAgain => {
                    let expected = state.active_drill().expected;
                    vec![
                        "TRY AGAIN".to_string(),
                        "Expected:".to_string(),
                        expected.to_string(),
                    ]
                }
            };
            false
        }
        KeyCode::Tab => false,
        KeyCode::Char(ch) => {
            state.input.push(ch);
            false
        }
        _ => false,
    }
}

fn render_ui(frame: &mut ratatui::Frame, state: &TuiState) {
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

    let mut header = vec![
        Line::raw("Rust Reps"),
        Line::raw("Type-based typing practice"),
    ];
    if state.completed {
        header.push(Line::raw("Session complete • Press Esc or Ctrl+C to quit"));
    } else {
        header.push(Line::raw("Press Esc or Ctrl+C to quit"));
    }

    frame.render_widget(
        Paragraph::new(Text::from(header)).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Rust Reps TUI"),
        ),
        chunks[0],
    );

    let drill = state.active_drill();
    let drill_panel = vec![
        Line::raw(format!(
            "Drill {}/{}",
            drill.id,
            state.session.total_drills()
        )),
        Line::raw(""),
        Line::raw(drill.expected),
        Line::raw(""),
        Line::raw(format!("> {}", state.input)),
    ];
    frame.render_widget(
        Paragraph::new(Text::from(drill_panel)).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Drill"),
        ),
        chunks[1],
    );

    let stats = [
        Line::raw(format!(
            "Passed: {}/{}",
            state.session.passed(),
            state.session.total_drills()
        )),
        Line::raw(format!("Skipped: {}", state.session.skipped())),
        Line::raw(format!("Attempts: {}", state.session.attempts())),
    ];
    frame.render_widget(
        Paragraph::new(Text::from(stats.to_vec()))
            .block(Block::default().borders(Borders::ALL).title("Session")),
        chunks[2],
    );

    frame.render_widget(
        Paragraph::new("F1 or ? for command panel")
            .block(Block::default().borders(Borders::ALL).title("Mode")),
        chunks[3],
    );

    let mut status: Vec<Line> = state
        .status_lines
        .iter()
        .map(|line| Line::raw(line.as_str()))
        .collect();

    if state.show_help_panel {
        let help_lines = [
            "Commands:",
            "Backspace: delete char",
            "Enter: submit input",
            "Esc / Ctrl+C: quit",
            "F1 or ?: toggle this panel",
        ];
        status.extend(help_lines.into_iter().map(Line::from));
    }

    frame.render_widget(
        Paragraph::new(Text::from(status))
            .block(Block::default().borders(Borders::ALL).title("Output")),
        chunks[4],
    );
}

fn collect_complete_lines(session: &practice::PracticeSession) -> Vec<String> {
    let mut lines = vec!["COMPLETE".to_string(), "Summary:".to_string()];
    lines.extend(collect_summary_pairs(session));
    lines
}

fn collect_summary_lines(action: &str, session: &practice::PracticeSession) -> Vec<String> {
    let mut lines = vec![action.to_string()];
    if action == "QUIT" || action == "COMPLETE" {
        lines.push("Summary:".to_string());
        lines.extend(collect_summary_pairs(session));
    }
    lines
}

fn collect_summary_pairs(session: &practice::PracticeSession) -> Vec<String> {
    vec![
        format!("Passed: {}/{}", session.passed(), session.total_drills()),
        format!("Skipped: {}", session.skipped()),
        format!("Attempts: {}", session.attempts()),
    ]
}

fn enter_tui() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let stdout = io::stdout();
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, crossterm::cursor::Hide)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.clear()?;
    Ok(terminal)
}

fn disable_tui() -> io::Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{collect_summary_lines, collect_summary_pairs};
    use crate::practice;

    #[test]
    fn summary_lines_include_attempt_counts() {
        let mut session = practice::PracticeSession::new();
        let drill = session
            .current_drill()
            .expect("expected first drill")
            .expected;
        let _ = session.submit_input(drill);
        let lines = collect_summary_lines("QUIT", &session);

        let summary = collect_summary_pairs(&session);
        assert!(lines[0].contains("QUIT"));
        assert!(lines.contains(&"Summary:".to_string()));
        for item in summary {
            assert!(lines.contains(&item));
        }
    }
}
