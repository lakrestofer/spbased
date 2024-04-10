use std::io::Stdout;

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

pub type AppResult<T> = Result<T>;

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Clone, Copy)]
pub enum ApplicationEvent {
    Shutdown,
}
