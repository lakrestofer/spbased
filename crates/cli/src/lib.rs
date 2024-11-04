pub mod commands;
pub mod db;

pub mod scheduler {

    /// Uses the data from the models and spaced repetition algorithm to determine
    pub struct Scheduler;

    impl Scheduler {
        pub fn schedule() {
            todo!()
        }
    }
}

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::init;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// struct defining the arguments and commands that the cli takes
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: PathBuf },
}

pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Init { directory } => init(directory),
    }?;
    Ok(())
}
