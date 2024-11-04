use anyhow::Result;
use clap::Parser;

use spbasedctl::handle_command;
use spbasedctl::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    handle_command(cli.command)?;

    Ok(())
}
