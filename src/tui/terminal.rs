use std::io::{self, Stdout};

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
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            crossterm::cursor::Hide
        )?;
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        terminal.clear()?;
        Ok(Self {
            terminal,
            activated: true,
        })
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        if !self.activated {
            return Ok(());
        }

        execute!(
            io::stdout(),
            DisableMouseCapture,
            LeaveAlternateScreen,
            crossterm::cursor::Show
        )?;
        disable_raw_mode()?;
        self.activated = false;
        Ok(())
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
            crossterm::cursor::Show
        );
        let _ = disable_raw_mode();
        self.activated = false;
    }
}
