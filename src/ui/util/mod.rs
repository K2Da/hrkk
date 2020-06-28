use super::TypedTerminal;
use crate::error::Result;
use tui::{backend::RustboxBackend, Terminal};
pub(crate) mod event;

pub(crate) fn terminal() -> Result<TypedTerminal> {
    let backend = RustboxBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}
