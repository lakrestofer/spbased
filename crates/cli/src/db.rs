//! Wrapper around the sqlite db that spbased used

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use std::{cell::LazyCell, path::Path};

/// Wrapper around rustqlite [`Connection`].
/// We expose the inner conn since this struct
/// is to be used using a dependency injection
/// style.
pub struct DB {
    /// the inner connection fields
    pub conn: Connection,
}

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
const MIGRATIONS: LazyCell<Migrations> =
    LazyCell::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

impl DB {
    /// Creates a new instance of the spbased sqlite db and runs all migrations on it.
    /// If there already exists an instance at `db_path`, we will reinit it.
    pub fn init(db_path: &Path) -> Result<()> {
        // if a file with the name db_path exist, we delete it
        if db_path.exists() {
            std::fs::remove_file(db_path)?;
        }

        // open and create a sqlite db
        let mut conn = Connection::open(db_path).context("trying to open connection")?;

        // run migrations on it
        MIGRATIONS
            .to_latest(&mut conn)
            .context("Trying to migrate sqlite schema")?;

        Ok(())
    }
}
