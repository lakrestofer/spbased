use anyhow::Result;
use clap::Parser;

use spbasedctl::cli::Cli;
use spbasedctl::handle_command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::init();

    let root = cli.root;

    let res = handle_command(root, cli.command)?;

    if let Some(res) = res {
        println!("{}", res);
    }
    Ok(())
}
