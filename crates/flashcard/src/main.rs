use anyhow::Result;
use clap::Parser;
use flashcard::*;

fn main() -> Result<()> {
    let parser = cli::Cli::parse();
    handle_command(parser.command)?;
    Ok(())
}
