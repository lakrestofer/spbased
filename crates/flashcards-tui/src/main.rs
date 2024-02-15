pub mod cli;
pub mod config;
pub mod tui;
pub mod utils;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

use crate::utils::{initialize_logging, initialize_panic_handler};

struct App;

impl App {
    fn new() -> Result<Self> {
        Ok(Self)
    }

    async fn run(&self) -> Result<()> {
        Ok(())
    }
}

async fn tokio_main() -> Result<()> {
    // init
    initialize_logging()?;
    initialize_panic_handler()?;

    // Cli options
    let _args = Cli::parse();
    let app = App::new()?;

    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
