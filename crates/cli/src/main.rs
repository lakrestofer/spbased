use anyhow::Result;
use clap::Parser;

use spbased_cli::handle_command;
use spbased_cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    handle_command(cli.command)?;

    Ok(())
}
