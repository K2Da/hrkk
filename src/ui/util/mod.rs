use super::TypedTerminal;
use crate::error::Result;
use std::io;
use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
pub mod event;

pub fn terminal() -> Result<TypedTerminal> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}
