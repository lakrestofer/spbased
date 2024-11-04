use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};
use dialoguer::Input;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    migration_directory: PathBuf,
}

fn main() -> Result<()> {
    let Cli {
        migration_directory,
    } = Cli::parse();

    let name: String = Input::new()
        .with_prompt("Name of migration")
        .interact_text()
        .unwrap();
    let name: String = name.split_whitespace().collect::<Vec<&str>>().join("_");

    // read the directory to find the largest id
    let dir = migration_directory.read_dir();
    let n_migrations = 'block: {
        if let Ok(dir) = dir {
            break 'block dir.count();
        } else {
            break 'block 1;
        };
    };

    let dir_path = migration_directory.join(format!("{:03}-{name}", n_migrations));
    let up_path = dir_path.join("up.sql");
    let down_path = dir_path.join("down.sql");

    std::fs::create_dir_all(dir_path)?;
    std::fs::File::create_new(up_path)?;
    std::fs::File::create_new(down_path)?;

    Ok(())
}
