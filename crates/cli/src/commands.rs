//! Implementations of the various commands that spbased-cli exposes

use crate::db::DB;
use anyhow::{Context, Result};
use dialoguer::Confirm;
use normalize_path::NormalizePath;
use resolve_path::PathResolveExt;
use std::path::PathBuf;

/// Init a new .spbased directory containing a sqlite db instance
/// and a config file
pub fn init(directory: PathBuf) -> Result<()> {
    let full_path: PathBuf = directory.try_resolve()?.into_owned().normalize();

    let res = Confirm::new()
        .with_prompt(format!(
            "Are you sure that you want to init spbased here: {:?}",
            full_path
        ))
        .interact()
        .context("tried to retrieve an answer from the user")?;

    if !res {
        println!("Goodbye!");
        return Ok(());
    }

    let spbased_dir = full_path.join(".spbased");

    // if an file with the path we want to use already exists, then exist
    if spbased_dir.exists() {
        let res = Confirm::new()
        .with_prompt(format!(
            "A directory called .spbased already exists at {:?}. Are you sure that you want to (re)init spased here?",
            full_path
        ))
        .interact()
        .context("tried to retrieve an answer from the user")?;

        if !res {
            return Ok(());
        }
    }

    // create the directory
    std::fs::create_dir_all(&spbased_dir)?;

    let db_path = spbased_dir.join("db.sqlite");
    DB::init(&db_path)?;

    Ok(())
}
