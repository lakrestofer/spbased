use std::fs::{File, OpenOptions};
use std::io::Write;

use anyhow::Result;
use clap::Parser;
use flashcard::*;

fn main() -> Result<()> {
    let parser = cli::Cli::parse();
    let res = handle_command(parser.command)?;
    match (res, parser.output) {
        (Some(res), Some(path)) => {
            let mut file = File::create(path)?;
            writeln!(file, "{}", res)?;
        }
        (Some(res), None) => println!("{res}"),
        _ => {}
    }
    Ok(())
}
