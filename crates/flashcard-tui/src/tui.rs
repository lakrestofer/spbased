use crate::preamble::*;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::panic;

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

/// Initializes the terminal interface.
///
/// It enables the raw mode and sets terminal properties.
pub fn init_terminal<B: Backend>(terminal: &mut Terminal<B>) -> AppResult<()> {
    enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // Define a custom panic hook to reset the terminal properties.
    // This way, you won't have your terminal messed up if an unexpected error happens.
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset_terminal().expect("failed to reset the terminal");
        panic_hook(panic); // call the previous panic_hook as well
    }));

    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(())
}

fn reset_terminal() -> AppResult<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

pub fn exit_terminal<B: Backend>(terminal: &mut Terminal<B>) -> AppResult<()> {
    reset_terminal()?;
    terminal.show_cursor()?;
    Ok(())
}
