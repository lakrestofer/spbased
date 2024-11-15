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
use serde_json::json;
use std::path::PathBuf;
use std::{cell::LazyCell, path::Path};
use time::OffsetDateTime;

pub mod cli;
pub mod queries;

use cli::*;

pub const SPBASED_WORK_DIR: &'static str = ".spbased";
pub const SPBASED_DB_NAME: &'static str = "db.sqlite";
pub const SPBASED_CONFIG_NAME: &'static str = "config.toml";

// ======= CLI COMMAND HANDLERS BEGIN ======
pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Init { directory } => command::init(directory)?,
        Command::Items(c) => command::item::handle_command(c)?,
        Command::Review(c) => command::review::handle_command(c)?,
        Command::Tags(c) => command::tag::handle_command(c)?,
    };
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

        // init the db
        db::init(&spbased_dir.join(SPBASED_DB_NAME))?;

        Ok(())
    }

    pub mod item {

        use super::*;

        pub fn handle_command(command: ItemCommand) -> Result<()> {
            let mut c: Connection = db::open(&get_db_path()?)?;

            Ok(match command {
                ItemCommand::Add { model, data, tags } => {
                    let id = queries::item::add(
                        &mut c,
                        &model,
                        &data,
                        &(tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
                    )?;
                    println!("{}", json!({ "id": id }).to_string())
                }
                ItemCommand::Edit { id, model, data } => {
                    if let Some(model) = model {
                        queries::item::edit_model(&mut c, id, &model)?;
                    }
                    if let Some(data) = data {
                        queries::item::edit_data(&mut c, id, &data)?;
                    }
                }
                ItemCommand::Query {
                    pre_filter,
                    post_filter,
                    pretty,
                } => {
                    // we apply sql filtering on items
                    let items = queries::item::query(&mut c, pre_filter)?;
                    // we apply json filter on items
                    let items = jmessearch_and_prettify(items, post_filter, pretty)?;
                    println!("{items}");
                }
            })
        }
    }

    fn jmessearch_and_prettify<T: serde::ser::Serialize>(
        value: T,
        filter: Option<String>,
        pretty: bool,
    ) -> Result<String> {
        Ok(match (filter, pretty) {
            (Some(filter), pretty) => {
                let expr =
                    jmespath::compile(&filter).context("compiling jmespath filter expression")?;
                let json = jmespath::Variable::from_serializable(value)?;
                let var = expr.search(json)?;
                if pretty {
                    serde_json::to_string_pretty(&var)?
                } else {
                    serde_json::to_string(&var)?
                }
            }
            (None, true) => serde_json::to_string_pretty(&value)?,
            (None, false) => serde_json::to_string(&value)?,
        })
    }

    pub mod review {

        use time::Duration;

        use super::*;
        // use serde_json::json;

        pub fn handle_command(command: ReviewCommand) -> Result<()> {
            let mut c: Connection = db::open(&get_db_path()?)?;
            match command {
                ReviewCommand::Next(cmd) => match cmd {
                    NextReviewCommand::New {
                        pre_filter,
                        post_filter,
                        pretty,
                    } => {
                        // we apply sql filtering on items
                        let items = queries::review::study_new(&mut c, pre_filter)?;

                        // we apply json filter on items
                        let items = jmessearch_and_prettify(items, post_filter, pretty)?;
                        println!("{items}");
                    }
                    NextReviewCommand::Due {
                        pre_filter,
                        post_filter,
                        pretty,
                    } => {
                        // we apply sql filtering on items
                        let items = queries::review::study_due(&mut c, pre_filter)?;

                        // we apply json filter on items
                        let items = jmessearch_and_prettify(items, post_filter, pretty)?;
                        println!("{items}");
                    }
                },
                ReviewCommand::Score { id, grade } => {
                    use sra::model::Grade::*;
                    use Maturity::*;

                    let item = queries::item::get(&mut c, id)?;

                    let g = grade;
                    let today = time::OffsetDateTime::now_utc();
                    let duration_since_last_review = today - item.last_review_date;
                    let last_review_was_today = Duration::days(1) < duration_since_last_review;

                    match (item.maturity, grade, last_review_was_today) {
                        (New, Again | Hard, _) => {
                            // we need to review the item again in this session
                            queries::review::increment_n_reviews(&mut c, item.id)?;
                        }
                        (New, grade, _) => {}
                        (Young, Again, true) => todo!(),
                        (Young, Again, false) => todo!(),
                        (Young, Hard, true) => todo!(),
                        (Young, Hard, false) => todo!(),
                        (Young, Good, true) => todo!(),
                        (Young, Good, false) => todo!(),
                        (Young, Easy, true) => todo!(),
                        (Young, Easy, false) => todo!(),
                        (Tenured, Again, true) => todo!(),
                        (Tenured, Again, false) => todo!(),
                        (Tenured, Hard, true) => todo!(),
                        (Tenured, Hard, false) => todo!(),
                        (Tenured, Good, true) => todo!(),
                        (Tenured, Good, false) => todo!(),
                        (Tenured, Easy, true) => todo!(),
                        (Tenured, Easy, false) => todo!(),
                    };
                }
                ReviewCommand::QueryCount(cmd) => {
                    let res = match cmd {
                        QueryCountCommand::Due { filter } => {
                            queries::review::query_n_due(&mut c, filter)?
                        }
                        QueryCountCommand::New { filter } => {
                            queries::review::query_n_new(&mut c, filter)?
                        }
                    };
                    if let Some(res) = res {
                        println!("{res}");
                    }
                }
            }
            Ok(())
        }
    }

    pub mod tag {
        use serde_json::json;

        use super::*;

        pub fn handle_command(command: TagCommand) -> Result<()> {
            let mut c: Connection = db::open(&get_db_path()?)?;

            Ok(match command {
                TagCommand::Add { name } => {
                    let id = queries::tag::add(&mut c, &name)?;
                    println!("{}", json!({ "id": id }).to_string());
                }
                TagCommand::Edit { old_name, new_name } => {
                    queries::tag::edit(&mut c, &old_name, &new_name)?;
                }
                TagCommand::Query {
                    pretty,
                    pre_filter,
                    post_filter,
                } => {
                    let tags = queries::tag::query(&mut c, pre_filter)?;
                    let tags = jmessearch_and_prettify(tags, post_filter, pretty)?;
                    println!("{tags}");
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
            "New" => Ok(Maturity::New),
            "Young" => Ok(Maturity::Young),
            "Tenured" => Ok(Maturity::Tenured),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl std::fmt::Display for Maturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Maturity::New => write!(f, "New"),
            Maturity::Young => write!(f, "Young"),
            Maturity::Tenured => write!(f, "Tenured"),
        }
    }
}

pub mod model {
    use super::*;

    pub type ItemModel = String;
    pub type ItemData = String;
    pub type TagName = String;

    #[derive(Serialize, Deserialize)]
    pub struct Item {
        pub id: i32,
        pub maturity: Maturity,
        pub stability: sra::model::Stability,
        pub difficulty: sra::model::Difficulty,
        #[serde(with = "time::serde::rfc3339")]
        pub last_review_date: OffsetDateTime,
        pub n_reviews: i32,
        pub model: ItemModel,
        pub data: ItemData,
        #[serde(with = "time::serde::rfc3339")]
        pub updated_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,
    }
    #[derive(Serialize, Deserialize)]
    pub struct Tag {
        pub id: i32,
        pub name: TagName,
        #[serde(with = "time::serde::rfc3339")]
        pub updated_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,
    }
}

pub mod db {
    use super::*;
    // TODO in future: perform build step that removes any comments and whitespace from the files
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
