use std::io::{self, Stdout};

use crossterm::cursor::{SetCursorStyle, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

pub struct TuiTerminal {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    activated: bool,
}

impl TuiTerminal {
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        let entered = execute!(
            io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            SetCursorStyle::SteadyBar,
            Show
        );
        let result = entered.and_then(|()| {
            let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
            terminal.clear()?;
            Ok(Self {
                terminal,
                activated: true,
            })
        });
        if result.is_err() {
            let _ = execute!(
                io::stdout(),
                DisableMouseCapture,
                LeaveAlternateScreen,
                SetCursorStyle::DefaultUserShape,
                Show
            );
            let _ = disable_raw_mode();
        }
        result
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        if !self.activated {
            return Ok(());
        }

        let screen_result = execute!(
            io::stdout(),
            DisableMouseCapture,
            LeaveAlternateScreen,
            SetCursorStyle::DefaultUserShape,
            Show
        );
        let raw_result = disable_raw_mode();
        if screen_result.is_ok() && raw_result.is_ok() {
            self.activated = false;
        }
        screen_result.and(raw_result)
    }
}

impl Drop for TuiTerminal {
    fn drop(&mut self) {
        if !self.activated {
            return;
        }

        let _ = execute!(
            io::stdout(),
            DisableMouseCapture,
            LeaveAlternateScreen,
            SetCursorStyle::DefaultUserShape,
            Show
        );
        let _ = disable_raw_mode();
        self.activated = false;
    }
}
