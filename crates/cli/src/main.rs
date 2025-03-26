use anyhow::Result;
use clap::Parser;

use env_logger::Env;
use spbasedctl::cli::Cli;
use spbasedctl::handle_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let env = Env::new().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let root = cli.root;

    let res = handle_command(root, cli.command)?;

    if let Some(res) = res {
        println!("{}", res);
    }
    Ok(())
}
