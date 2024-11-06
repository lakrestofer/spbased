use anyhow::Context;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use include_dir::{include_dir, Dir};
use normalize_path::NormalizePath;
use resolve_path::PathResolveExt;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use std::path::PathBuf;
use std::{cell::LazyCell, path::Path};

// ======= CLI ARGUMENT AND COMMAND DEFINITIONS BEGIN ======
// the outward facing api of the cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
/// struct defining the arguments and commands that the cli takes
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    /// the location
    #[arg(short, long)]
    pub spbased_dir: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: Option<PathBuf> },
    /// Query, or CRUD models (format of specific kind of review item)
    #[command(subcommand)]
    Model(ModelCommand),
    /// Query, or CRUD review items
    #[command(subcommand)]
    Item(ItemCommand),
    /// Review the review items
    Review {
        #[arg(short, long)]
        filter: Option<String>,
        #[command(subcommand)]
        cmd: ReviewCommand,
    },
}
#[derive(Subcommand)]
pub enum ModelCommand {
    Register { name: String, cmd: String },
    UnRegister { name: String },
}
#[derive(Subcommand)]
pub enum ItemCommand {
    Add {
        model: String,
        data: String,
    },
    Edit {
        id: i32,
        model: String,
        data: String,
    },
}
#[derive(Subcommand)]
pub enum ReviewCommand {
    /// Review the most urgent review item that is due
    Next,
    Query,
}
// ======= CLI ARGUMENT AND COMMAND DEFINITIONS END ======

// ======= CLI COMMAND HANDLERS BEGIN ======
pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Init { directory } => init(directory),
        Command::Model(_) => Ok(()),
        Command::Item(_) => Ok(()),
        Command::Review { filter, cmd } => todo!(),
    }?;
    Ok(())
}

/// Init a new .spbased directory containing a sqlite db instance
/// and a config file
pub fn init(directory: Option<PathBuf>) -> Result<()> {
    if directory.is_none() {
        // TODO init in default location .local/share/spbased
        todo!();
        return Ok(());
    }

    let directory = directory.unwrap();

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
    create_db_and_apply_migrations(&db_path)?;

    Ok(())
}
// ======= CLI COMMAND HANDLERS END ======

// ======= DB WRAPPER AND DATA MODEL BEGIN BEGIN ======

// TODO perform build step that removes any comments and whitespace from the files
static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
const MIGRATIONS: LazyCell<Migrations> =
    LazyCell::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

/// Creates a new instance of the spbased sqlite db and runs all migrations on it.
/// If there already exists an instance at `db_path`, we will reinit it.
pub fn create_db_and_apply_migrations(db_path: &Path) -> Result<()> {
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

// ======= DB WRAPPER AND DATA MODEL BEGIN END ======

// ======= SCHEDULER BEGIN ======

/// Uses the data from the models and spaced repetition algorithm to determine
pub struct Scheduler;

impl Scheduler {
    pub fn schedule() {
        todo!()
    }
}
// ======= SCHEDULER END ======
