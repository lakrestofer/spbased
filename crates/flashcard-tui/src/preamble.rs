use std::io::Stdout;

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

pub type AppResult<T> = Result<T>;
pub use crate::util::Boxed;

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

pub enum ApplicationEvent {
    Shutdown,
}
