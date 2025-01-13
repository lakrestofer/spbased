use anyhow::Result;
use clap::Parser;

use spbasedctl::cli::Cli;
use spbasedctl::handle_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let res = handle_command(cli.command)?;

    if let Some(res) = res {
        println!("{}", res);
    }
    Ok(())
}
