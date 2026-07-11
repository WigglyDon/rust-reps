use std::io;
use std::time::{Duration, Instant};

use crate::{client::ClientAction, engine};
use crossterm::event::{self, Event};

mod app;
mod frame_scheduler;
mod input;
mod render;
mod terminal;
mod theme;

use frame_scheduler::FrameScheduler;

pub use app::TuiApp;
pub use input::classify_key_event;
use input::classify_mouse_event;

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);
const TARGET_FRAME_RATE: u16 = 60;

pub fn run_tui_with_session(session: engine::Session) -> io::Result<()> {
    run_tui_app(TuiApp::with_session(session))
}

fn run_tui_app(mut app: TuiApp) -> io::Result<()> {
    let mut terminal = terminal::TuiTerminal::enter()?;

    let loop_result = run_tui_loop(&mut terminal, &mut app);
    let cleanup_result = terminal.cleanup();

    match loop_result {
        Ok(()) => cleanup_result,
        Err(loop_err) => {
            if cleanup_result.is_err() {
                return cleanup_result;
            }
            return Err(loop_err);
        }
    }
}

fn run_tui_loop(terminal: &mut terminal::TuiTerminal, app: &mut TuiApp) -> io::Result<()> {
    let mut frame_scheduler = FrameScheduler::new(TARGET_FRAME_RATE);
    frame_scheduler.request_redraw_now(Instant::now());
    let mut next_cursor_tick = Instant::now() + CURSOR_BLINK_INTERVAL;

    loop {
        let now = Instant::now();
        if frame_scheduler.consume_due_redraw(now) {
            app.settle_animations(now);
            terminal
                .terminal
                .draw(|frame| render::render_ui(frame, app, now))?;

            if app.should_quit() {
                break;
            }

            if app.visual_feedback_active_at(now) {
                frame_scheduler.request_redraw_after(now, frame_scheduler.min_frame_interval());
            }

            continue;
        }

        let now = Instant::now();
        if now >= next_cursor_tick {
            app.tick_cursor();
            next_cursor_tick = now + CURSOR_BLINK_INTERVAL;
            frame_scheduler.request_redraw_now(now);
            continue;
        }

        let cursor_wait = next_cursor_tick.saturating_duration_since(now);
        let timeout = frame_scheduler
            .next_wait_duration(now)
            .map(|frame_wait| frame_wait.min(cursor_wait))
            .unwrap_or(cursor_wait);

        if event::poll(timeout)? {
            let event = match event::read() {
                Ok(event) => event,
                Err(err) => return Err(err),
            };
            let action_at = Instant::now();

            let intent = match event {
                Event::Key(key) => classify_key_event(key),
                Event::Mouse(mouse) => {
                    let size = terminal.terminal.size()?;
                    let area = ratatui::layout::Rect::new(0, 0, size.width, size.height);
                    let layout = render::input_layout(area, app, action_at);
                    app.set_editor_viewport_height(render::visible_editor_rows(
                        layout.editor.height,
                    ));
                    let client_frame = app.frame_at(action_at);
                    let editor_cursor = render::editor_cursor_at(
                        layout.editor,
                        &client_frame,
                        mouse.column,
                        mouse.row,
                    );
                    classify_mouse_event(
                        mouse,
                        layout.mouse_pane_at(mouse.column, mouse.row),
                        editor_cursor,
                    )
                }
                _ => ClientAction::Ignore,
            };
            app.handle_action_at(intent, action_at);
            next_cursor_tick = action_at + CURSOR_BLINK_INTERVAL;
            frame_scheduler.request_redraw_now(action_at);
        } else {
            let timeout_at = Instant::now();
            if timeout_at >= next_cursor_tick {
                app.tick_cursor();
                next_cursor_tick = timeout_at + CURSOR_BLINK_INTERVAL;
                frame_scheduler.request_redraw_now(timeout_at);
            }
        }
    }

    Ok(())
}
