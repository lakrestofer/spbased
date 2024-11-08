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
use std::path::PathBuf;
use std::{cell::LazyCell, path::Path};
use time::OffsetDateTime;

pub mod queries;

// ======= CLI ARGUMENT AND COMMAND DEFINITIONS BEGIN ======
#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Struct defining the arguments and commands that the cli takes.
/// The outward facing api of the cli
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: PathBuf },
    /// Query, or CRUD items
    #[command(subcommand)]
    Items(ItemCommand),
    /// Query, or CRUD tags
    #[command(subcommand)]
    Tags(TagCommand),
    /// Review the items
    #[command(subcommand)]
    Review(ReviewCommand),
}

#[derive(Subcommand)]
pub enum ItemCommand {
    Add {
        model: String,
        data: String,
        tags: Option<Vec<String>>,
    },
    Edit {
        id: i32,
        model: String,
        data: String,
    },
    // TODO add filters, for now simply list all options
    Query {
        filter: Option<String>,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Serialize, Deserialize)]
pub enum Grade {
    Fail = 1,
    Hard = 2,
    Ok = 3,
    Easy = 4,
}

impl From<Grade> for sra::model::Grade {
    fn from(g: Grade) -> Self {
        use sra::model::Grade as Out;
        use Grade as In;
        match g {
            In::Fail => Out::Fail,
            In::Hard => Out::Hard,
            In::Ok => Out::Ok,
            In::Easy => Out::Easy,
        }
    }
}

#[derive(Subcommand)]
pub enum ReviewCommand {
    /// Review the most urgent review item that is due
    Next,
    /// Directly score a review item, updating its scheduling data.
    Score { score: Grade },
    /// Retrieve information about a review event
    Query { filter: Option<String> },
}
#[derive(Subcommand)]
pub enum TagCommand {
    /// Add a new tag
    Add { name: String },
    /// Edit a tag
    Edit { old_name: String, new_name: String },
    /// List tags
    /// TODO: add query options
    Query { filter: Option<String> },
}
// ======= CLI ARGUMENT AND COMMAND DEFINITIONS END ======

// ======= CONFIG FILE DEFINITION START ======

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    models: Vec<Model>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    name: String,
    program: String,
    description: Option<String>,
}

impl Model {
    pub fn new(name: &str, program: &str) -> Self {
        Self {
            name: name.into(),
            program: program.into(),
            description: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            models: [Model::new("flashcard", "pelji")].into(),
        }
    }
}
// ======= CONFIG FILE DEFINITION END ======

// ======= CLI COMMAND HANDLERS BEGIN ======
pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Init { directory } => init(directory),
        Command::Items(_) => Ok(()),
        Command::Review(_) => todo!(),
        Command::Tags(_) => todo!(),
    }?;
    Ok(())
}

/// Init a new .spbased directory containing a sqlite db instance
/// and a config file
pub fn init(directory: PathBuf) -> Result<()> {
    let full_path: PathBuf = directory.try_resolve()?.into_owned().normalize(); // resolve directory like "~/some/dir" into "/home/username/some/dir"

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

    // confirm that user wants to overwrite dir
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
    db::init(&db_path)?;

    Ok(())
}
// ======= CLI COMMAND HANDLERS END ======

// ======= DB WRAPPER AND DATA MODEL BEGIN BEGIN ======

/// A measure of how well we've 'learnt' an item.
#[derive(Debug, Default, PartialEq, Eq)]
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
