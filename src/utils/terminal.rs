#![allow(dead_code)]
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal as RatatuiTerminal};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("Failed to enable raw mode: {0}")]
    RawMode(#[source] io::Error),
    #[error("Failed to enter alternate screen: {0}")]
    AlternateScreen(#[source] io::Error),
    #[error("Failed to create terminal: {0}")]
    Create(#[source] io::Error),
}

pub struct Terminal {
    inner: RatatuiTerminal<CrosstermBackend<io::Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self, TerminalError> {
        enable_raw_mode().map_err(TerminalError::RawMode)?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(TerminalError::AlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = RatatuiTerminal::new(backend).map_err(TerminalError::Create)?;

        Ok(Self { inner: terminal })
    }

    pub fn draw<F>(&mut self, f: F) -> Result<(), io::Error>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.inner.draw(f)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), io::Error> {
        self.inner.clear()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.inner.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
}
