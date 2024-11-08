use anyhow::Result;
use clap::Parser;

use spbasedctl::cli::Cli;
use spbasedctl::handle_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    handle_command(cli.command)?;

    Ok(())
}
