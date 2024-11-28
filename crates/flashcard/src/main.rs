use anyhow::Result;
use clap::Parser;
use flashcard::*;

fn main() -> Result<()> {
    let parser = cli::Cli::parse();
    let res = handle_command(parser.command)?;
    match (res, parser.output) {
        (Some(res), Some(path)) => {
            std::fs::write(path, res)?;
        }
        (Some(res), None) => println!("{res}"),
        _ => {}
    }
    Ok(())
}
