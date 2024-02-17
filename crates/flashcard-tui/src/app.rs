// internal imports
use crate::{args::Args, config::Config, preamble::*};
// external imports
use crossterm::{
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};

// types
pub type Term = Terminal<CrosstermBackend<Stdout>>;

fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn terminal() -> Result<Term> {
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

pub struct App {
    term: Term,
}

impl App {
    /// given the arguments and the config we build the app
    pub fn build(args: Args, config: Config) -> Result<App> {
        let term = terminal()?;
        Ok(Self { term: term })
    }

    pub fn run(&self) -> Result<()> {
        init_terminal()?;

        // we init the terminal handle
        restore_terminal()?;
        Ok(())
    }
}
