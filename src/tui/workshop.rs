use crate::{
    client::{
        CursorPosition, EditorAction, tokenize_rust_line,
        workshop::{Action, WorkshopClient},
    },
    workshop::Session,
    workshop_process::safe_text,
};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::{io, path::Path, time::Duration};

pub(super) fn run() -> io::Result<()> {
    let session = Session::open(Path::new(".rust-reps-local/workshop"))?;
    let mut app = WorkshopClient::new(session);
    let mut terminal = super::terminal::TuiTerminal::enter()?;
    let result = run_loop(&mut terminal, &mut app);
    // Reap jobs before terminal restoration on every path.
    app.session.shutdown();
    let cleanup = terminal.cleanup();
    if result.is_err() && !app.session.save() {
        match app.session.recover() {
            Ok(path) => eprintln!("Workshop buffer recovered at {}", path.display()),
            Err(error) => eprintln!(
                "Workshop recovery failed: {error}\nUnsaved source follows:\n{}",
                safe_text(app.session.source.as_bytes())
            ),
        }
    }
    result.and(cleanup)
}

fn run_loop(
    terminal: &mut super::terminal::TuiTerminal,
    app: &mut WorkshopClient,
) -> io::Result<()> {
    loop {
        app.session.poll();
        terminal.terminal.draw(|frame| render(frame, app))?;
        if app.quit {
            return Ok(());
        }
        if event::poll(Duration::from_millis(30))? {
            match event::read()? {
                Event::Key(key) if key.kind != KeyEventKind::Release => {
                    app.handle(classify(key, app.information_focus))
                }
                Event::Mouse(mouse) => {
                    let size = terminal.terminal.size()?;
                    let (_, editor, info, _, _) = areas(Rect::new(0, 0, size.width, size.height));
                    let editor_inner = Block::default().borders(Borders::ALL).inner(editor);
                    if editor_inner.contains(Position::new(mouse.column, mouse.row)) {
                        match mouse.kind {
                            MouseEventKind::ScrollUp => app.editor.scroll_viewport_up(),
                            MouseEventKind::ScrollDown => app.editor.scroll_viewport_down(),
                            MouseEventKind::Down(MouseButton::Left) => {
                                let view = app.editor.viewport();
                                app.handle(Action::Cursor(CursorPosition {
                                    line: view.first_line + usize::from(mouse.row - editor_inner.y),
                                    column: view.first_column
                                        + usize::from(mouse.column - editor_inner.x),
                                }));
                            }
                            _ => {}
                        }
                    } else if info.contains(Position::new(mouse.column, mouse.row)) {
                        match mouse.kind {
                            MouseEventKind::ScrollUp => app.handle(Action::Scroll(-3)),
                            MouseEventKind::ScrollDown => app.handle(Action::Scroll(3)),
                            MouseEventKind::Down(MouseButton::Left) => app.information_focus = true,
                            _ => {}
                        }
                    }
                }
                // Next frame recalculates viewports on resize, preserving source.
                _ => {}
            }
        }
    }
}

fn classify(key: KeyEvent, information_focus: bool) -> Action {
    match key.code {
        KeyCode::Esc => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Save,
        KeyCode::Tab | KeyCode::BackTab => Action::ToggleFocus,
        KeyCode::F(2) => Action::CyclePane,
        KeyCode::F(3) => Action::PreviousStep,
        KeyCode::F(4) => Action::NextStep,
        KeyCode::F(5) => Action::Compile,
        KeyCode::F(6) => Action::Run,
        KeyCode::F(8) => Action::Stop,
        KeyCode::PageUp => Action::Scroll(-8),
        KeyCode::PageDown => Action::Scroll(8),
        KeyCode::Up if information_focus => Action::Scroll(-1),
        KeyCode::Down if information_focus => Action::Scroll(1),
        KeyCode::Home if information_focus => Action::Scroll(i32::MIN),
        KeyCode::Left => Action::Edit(EditorAction::MoveLeft),
        KeyCode::Right => Action::Edit(EditorAction::MoveRight),
        KeyCode::Up => Action::Edit(EditorAction::MoveUp),
        KeyCode::Down => Action::Edit(EditorAction::MoveDown),
        KeyCode::Home => Action::Edit(EditorAction::MoveLineStart),
        KeyCode::End => Action::Edit(EditorAction::MoveLineEnd),
        KeyCode::Backspace => Action::Edit(EditorAction::Backspace),
        KeyCode::Delete => Action::Edit(EditorAction::DeleteForward),
        KeyCode::Enter => Action::Edit(EditorAction::InsertNewline),
        KeyCode::Char(ch)
            if !key
                .modifiers
                .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
                && !ch.is_control() =>
        {
            Action::Edit(EditorAction::InsertChar(ch))
        }
        _ => Action::Ignore,
    }
}

fn areas(area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
    let rows = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(2),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .split(area);
    let panes =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);
    (rows[0], panes[0], panes[1], rows[2], rows[3])
}

fn render(frame: &mut Frame<'_>, app: &mut WorkshopClient) {
    let (heading, editor, info, status, controls) = areas(frame.area());
    let save = if app.session.dirty() {
        "UNSAVED"
    } else {
        "saved"
    };
    frame.render_widget(
        Paragraph::new(safe_text(
            format!(
                "Workshop preview • revision {} • {save}\n{}",
                app.session.revision,
                app.session.source_path().display()
            )
            .as_bytes(),
        ))
        .wrap(Wrap { trim: false }),
        heading,
    );
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .title(if app.information_focus {
            " Source • Tab to edit "
        } else {
            " Source • editing "
        });
    let inner = editor_block.inner(editor);
    if app.editor.viewport().visible_lines != usize::from(inner.height) {
        app.editor.set_viewport_height(usize::from(inner.height));
    }
    if app.editor.viewport().visible_columns != usize::from(inner.width).max(1) {
        app.editor.set_viewport_width(usize::from(inner.width));
    }
    let view = app.editor.view();
    let lines: Vec<Line> = view
        .visible_lines()
        .iter()
        .map(|line| {
            let clipped: String = line
                .chars()
                .skip(view.viewport.first_column)
                .take(view.viewport.visible_columns)
                .collect();
            Line::from(
                tokenize_rust_line(&safe_text(clipped.as_bytes()))
                    .into_iter()
                    .map(|token| {
                        Span::styled(token.text, super::theme::code_token_style(token.kind))
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    frame.render_widget(Paragraph::new(lines).block(editor_block), editor);
    if !app.information_focus
        && inner.width > 0
        && inner.height > 0
        && let (Some(row), Some(col)) = (
            view.cursor.line.checked_sub(view.viewport.first_line),
            view.cursor.column.checked_sub(view.viewport.first_column),
        )
        && row < usize::from(inner.height)
        && col < usize::from(inner.width)
    {
        frame.set_cursor_position((inner.x + col as u16, inner.y + row as u16));
    }
    let info_block = Block::default().borders(Borders::ALL).title(format!(
        " {:?} • {} • F2 tabs ",
        app.pane,
        if app.information_focus {
            "focused"
        } else {
            "Tab to focus"
        }
    ));
    let info_inner = info_block.inner(info);
    let content = safe_text(app.information().as_bytes());
    let paragraph = Paragraph::new(content).wrap(Wrap { trim: false });
    let total = paragraph.line_count(info_inner.width.max(1));
    let index = app.pane_index();
    app.scroll[index] = app.scroll[index].min(total.saturating_sub(usize::from(info_inner.height)));
    frame.render_widget(
        paragraph
            .scroll((app.scroll[index].min(u16::MAX as usize) as u16, 0))
            .block(info_block),
        info,
    );
    let eligibility = app
        .session
        .run_block_reason()
        .unwrap_or("Run ready: successful build for displayed revision");
    frame.render_widget(
        Paragraph::new(safe_text(
            format!("{}\n{eligibility}", app.session.message).as_bytes(),
        ))
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Yellow)),
        status,
    );
    frame.render_widget(Paragraph::new("Ctrl-S Save · F5 Compile · F6 Run · F8 Stop · Esc Save/Quit\nTab focus · F2 tabs · F3/F4 steps · PgUp/PgDn or wheel scroll\nTrusted local code runs with your permissions; NOT a security sandbox.").wrap(Wrap { trim: false }), controls);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workshop::{
        Activity,
        tests::{Fixture, settle},
    };
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn manually_typed_program_runs_and_input_navigation_stays_responsive() {
        let fixture = Fixture::new();
        let mut app = WorkshopClient::new(fixture.open());
        // Real character events, using existing delimiter pairing and indentation.
        for ch in "fn main() {\nprintln!(\"typed\");".chars() {
            let code = if ch == '\n' {
                KeyCode::Enter
            } else {
                KeyCode::Char(ch)
            };
            app.handle(classify(KeyEvent::from(code), app.information_focus));
        }
        assert_eq!(
            app.editor.text(),
            "fn main() {\n    println!(\"typed\");\n}"
        );
        app.handle(classify(KeyEvent::from(KeyCode::F(5)), false));
        assert!(app.session.busy());
        // A running compiler does not block pane navigation, scrolling or resize.
        app.handle(classify(KeyEvent::from(KeyCode::Tab), false));
        app.handle(classify(KeyEvent::from(KeyCode::F(2)), true));
        app.handle(classify(KeyEvent::from(KeyCode::PageDown), true));
        assert!(app.information_focus);
        assert_eq!(app.pane, crate::client::workshop::Pane::Program);
        for (width, height) in [(120, 35), (60, 16), (14, 5), (100, 30)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            terminal.draw(|frame| render(frame, &mut app)).unwrap();
            assert!(app.editor.text().contains("typed"));
        }
        settle(&mut app.session);
        assert!(
            app.session
                .output(Activity::Compile)
                .unwrap()
                .outcome
                .as_ref()
                .unwrap()
                .success()
        );
        app.handle(Action::Run);
        settle(&mut app.session);
        assert_eq!(
            app.session.output(Activity::Run).unwrap().capture.stdout,
            b"typed\n"
        );
        assert!(app.information().contains("stdout (observed):\ntyped"));
        let saved_source = app.editor.text();
        app.handle(Action::NextStep);
        app.handle(Action::NextStep);
        app.handle(Action::PreviousStep);
        assert_eq!(
            app.editor.text(),
            saved_source,
            "guidance must never fill a solution"
        );
        app.handle(Action::ToggleFocus);
        app.handle(Action::Edit(EditorAction::InsertChar(' ')));
        app.pane = crate::client::workshop::Pane::Program;
        assert!(app.information().contains("PREVIOUS revision"));
        assert!(app.session.run_block_reason().is_some());
    }

    #[test]
    fn quit_save_failure_preserves_buffer_and_blocks_exit() {
        let fixture = Fixture::new();
        let mut app = WorkshopClient::new(fixture.open());
        app.handle(Action::Edit(EditorAction::InsertChar('x')));
        std::fs::write(app.session.source_path(), "external").unwrap();
        app.handle(Action::Quit);
        assert!(!app.quit);
        assert_eq!(app.editor.text(), "x");
        assert!(app.session.message.contains("Save failed"));
        std::fs::write(app.session.source_path(), "").unwrap();
        app.handle(Action::Quit);
        assert!(app.quit);
        assert_eq!(
            std::fs::read_to_string(app.session.source_path()).unwrap(),
            "x"
        );
    }

    #[test]
    fn output_controls_are_escaped_in_the_actual_frame() {
        let fixture = Fixture::new();
        let mut app = WorkshopClient::new(fixture.open());
        app.session.message = "control \u{1b}[2J\u{202e}text".into();
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();
        let buffer = terminal.backend().buffer();
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains('\u{1b}'));
        assert!(!text.contains('\u{202e}'));
        assert!(text.contains("\\u{1b}[2J"));
    }

    #[test]
    fn editor_scrolling_survives_redraw_and_new_build_marks_retained_output_previous() {
        let fixture = Fixture::new();
        let mut app = WorkshopClient::new(fixture.open());
        let source = format!("fn main() {{}}\n{}", "// source line\n".repeat(30));
        app.handle(Action::Edit(EditorAction::SetText(source)));
        app.editor.set_cursor(CursorPosition::origin());
        let mut terminal = Terminal::new(TestBackend::new(100, 20)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();
        app.editor.scroll_viewport_down();
        app.editor.scroll_viewport_down();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();
        assert_eq!(app.editor.viewport().first_line, 2);
        app.handle(Action::Compile);
        settle(&mut app.session);
        app.handle(Action::Run);
        settle(&mut app.session);
        assert!(!app.information().contains("PREVIOUS"));
        app.handle(Action::Compile);
        app.handle(Action::Stop);
        settle(&mut app.session);
        app.pane = crate::client::workshop::Pane::Program;
        assert!(
            app.information()
                .contains("PREVIOUS revision/build attempt")
        );
        assert!(app.session.run_block_reason().is_some());
    }
}
