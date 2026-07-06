use std::io;

use crossterm::event::{self, Event};

mod app;
mod input;
mod render;
mod terminal;

pub use app::TuiApp;
pub use input::{UiIntent, classify_key_event};

pub fn run_tui() -> io::Result<()> {
    let mut terminal = terminal::TuiTerminal::enter()?;
    let mut app = TuiApp::new();

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
    loop {
        terminal
            .terminal
            .draw(|frame| render::render_ui(frame, app))?;

        if app.should_quit() {
            break;
        }

        let event = match event::read() {
            Ok(event) => event,
            Err(err) => return Err(err),
        };

        let intent = match event {
            Event::Key(key) => classify_key_event(key),
            _ => UiIntent::Ignore,
        };
        app.handle_intent(intent);
    }

    Ok(())
}
