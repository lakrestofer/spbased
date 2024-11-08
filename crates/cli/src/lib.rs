//! Spbasedctl implementation
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use include_dir::{include_dir, Dir};
use normalize_path::NormalizePath;
use resolve_path::PathResolveExt;
use rusqlite::params;
use rusqlite::params_from_iter;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::path::PathBuf;
use std::{cell::LazyCell, path::Path};
use time::OffsetDateTime;

pub mod cli;
pub mod config;
pub mod queries;

use cli::*;

pub const SPBASED_WORK_DIR: &'static str = ".spbased";
pub const SPBASED_DB_NAME: &'static str = "db.sqlite";

// ======= CLI COMMAND HANDLERS BEGIN ======
pub fn handle_command(command: Command) -> Result<()> {
    let output: String = match command {
        Command::Init { directory } => {
            command::init(directory)?;
            "spbased initialized successfully".into()
        }
        Command::Item(c) => command::item::handle_command(c)?,
        Command::Review(_) => todo!(),
        Command::Tags(_) => todo!(),
    };
    println!("{output}");
    Ok(())
}

pub mod command {
    use super::*;

    /// Init a new .spbased directory containing a sqlite db instance
    /// and a config file
    pub fn init(directory: PathBuf) -> Result<()> {
        let full_path: PathBuf = directory.try_resolve()?.into_owned().normalize();

        if !Confirm::new()
            .with_prompt(format!(
                "Are you sure that you want to init spbased here: {:?}",
                full_path
            ))
            .interact()
            .context("tried to retrieve an answer from the user")?
        {
            println!("Goodbye!");
            return Ok(());
        }

        let spbased_dir = full_path.join(SPBASED_WORK_DIR);

        // confirm that user wants to overwrite dir
        if spbased_dir.exists() {
            let res = Confirm::new()
        .with_prompt(format!(
            "A directory called {} already exists at {:?}. Are you sure that you want to (re)init spased here?",
            SPBASED_WORK_DIR,
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

        let db_path = spbased_dir.join(SPBASED_DB_NAME);
        db::init(&db_path)?;

        Ok(())
    }

    pub mod item {
        use serde_json::json;

        use super::*;
        use crate::ItemCommand;

        pub fn handle_command(command: ItemCommand) -> Result<String> {
            let mut c: Connection = db::open(&get_db_path()?)?;

            Ok(match command {
                ItemCommand::Add { model, data, tags } => {
                    let id = queries::item::add(
                        &mut c,
                        &model,
                        &data,
                        &(tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
                    )?;
                    json!({ "id": id }).to_string()
                }
                ItemCommand::Edit { id, model, data } => {
                    queries::item::edit_model(&mut c, id, &model)?;
                    queries::item::edit_data(&mut c, id, &data)?;
                    "".into()
                }
                ItemCommand::Query { filter: _ } => {
                    let items = queries::item::query(&mut c)?;
                    serde_json::to_string(&items)?
                }
            })
        }
    }
}

pub fn get_db_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let spbased_dir = cwd.join(SPBASED_WORK_DIR);
    if !spbased_dir.is_dir() {
        return Err(anyhow!(
            "directory .spbased could not be found in current working directory"
        ));
    }
    let spbased_db = spbased_dir.join(SPBASED_DB_NAME);
    if !spbased_db.is_file() {
        return Err(anyhow!(
            "{} could not be found in .spbased",
            SPBASED_DB_NAME
        ));
    }
    return Ok(spbased_db);
}

// ======= DB WRAPPER AND DATA MODEL BEGIN BEGIN ======

/// A measure of how well we've 'learnt' an item.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Maturity {
    /// This item has not yet been reviewed
    #[default]
    New,
    /// This item has been reviewed but has a stability less than 1 year.
    Young,
    /// This items has been reviewed many times and can probably be considered fully 'learnt'
    Tenured,
}
impl FromSql for Maturity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        match s {
            "NEW" => Ok(Maturity::New),
            "YOUNG" => Ok(Maturity::Young),
            "TENURED" => Ok(Maturity::Tenured),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl std::fmt::Display for Maturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Maturity::New => write!(f, "NEW"),
            Maturity::Young => write!(f, "YOUNG"),
            Maturity::Tenured => write!(f, "TENURED"),
        }
    }
}

/// Review item
pub type ItemModel = String;
pub type ItemData = String;
pub type TagName = String;
#[derive(Serialize, Deserialize)]
pub struct Item {
    id: i32,
    maturity: Maturity,
    stability: sra::model::Stability,
    difficulty: sra::model::Difficulty,
    last_review_date: OffsetDateTime,
    model: ItemModel,
    data: ItemData,
    updated_at: OffsetDateTime,
    created_at: OffsetDateTime,
}
pub struct Tag {
    id: i32,
    name: TagName,
    updated_at: OffsetDateTime,
    created_at: OffsetDateTime,
}

pub mod db {
    use super::*;
    // TODO perform build step that removes any comments and whitespace from the files
    pub const MIGRATIONS: LazyCell<Migrations> = LazyCell::new(|| {
        static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
        Migrations::from_directory(&DIR).unwrap()
    });
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

    pub fn open(db_path: &Path) -> Result<Connection> {
        let mut conn = Connection::open(db_path).context("trying to open connection")?;
        // run migrations on it
        MIGRATIONS
            .to_latest(&mut conn)
            .context("Trying to migrate sqlite schema")?;
        conn.execute("PRAGMA foreign_keys = ON", ())?; // enable foreign keys constraint
        Ok(conn)
    }
}
