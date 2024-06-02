use std::{io, panic};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use miette::{IntoDiagnostic, Result};

use super::{event::EventHandler, Picker};
pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    pub(crate) events: EventHandler,
    terminal: CrosstermTerminal,
    is_running: bool,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self {
            terminal,
            events,
            is_running: true,
        }
    }

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode().into_diagnostic()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)
            .into_diagnostic()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor().into_diagnostic()?;
        self.terminal.clear().into_diagnostic()?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode().into_diagnostic()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)
            .into_diagnostic()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor().into_diagnostic()?;
        self.is_running = false;
        Ok(())
    }

    pub fn draw(&mut self, picker: &mut Picker) -> Result<()> {
        self.terminal
            .draw(|frame| picker.render(frame))
            .into_diagnostic()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        if self.is_running {
            self.exit().unwrap();
        }
    }
}
