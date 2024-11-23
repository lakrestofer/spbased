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
use rusqlite::Connection;
use rusqlite_migration::Migrations;
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
pub fn handle_command(command: Command) -> Result<Option<String>> {
    Ok(match command {
        Command::Init { directory } => {
            command::init(directory)?;
            None
        }
        Command::Items(c) => command::item::handle_command(c)?,
        Command::Review(c) => command::review::handle_command(c)?,
        Command::Tags(c) => command::tag::handle_command(c)?,
    })
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

        /// The user is required to either input a string directly or a path to a file,
        /// in case of file, read string from file.
        fn item_data_string(data: ItemInputData) -> Result<String> {
            let str_data = match (data.data, data.file) {
                (Some(data), None) => data,
                (None, Some(path)) => std::fs::read_to_string(path)?,
                _ => unreachable!("--data and --file flags where input at the same time"), // clap should handle such that this can never happen
            };
            Ok(str_data)
        }

        pub fn handle_command(command: ItemCommand) -> Result<Option<String>> {
            let mut c: Connection = db::open(&get_db_path()?)?;

            Ok(match command {
                ItemCommand::Add { model, data, tags } => {
                    let id = queries::item::add(
                        &mut c,
                        &model,
                        &item_data_string(data)?,
                        &(tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
                    )?;
                    Some(format!("{}", json!({ "id": id }).to_string()))
                }
                ItemCommand::Edit { id, model, data } => {
                    if let Some(model) = model {
                        queries::item::edit_model(&mut c, id, &model)?;
                    }
                    if let Some(data) = data {
                        queries::item::edit_data(&mut c, id, &item_data_string(data)?)?;
                    }
                    None
                }
                ItemCommand::Delete { id } => {
                    queries::item::delete(&mut c, id)?;
                    None
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
                    Some(format!("{items}"))
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

        use model::Maturity;
        use time::Duration;

        use super::*;
        // use serde_json::json;

        pub fn handle_command(command: ReviewCommand) -> Result<Option<String>> {
            let mut c: Connection = db::open(&get_db_path()?)?;
            let res: Option<String> = match command {
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
                        Some(format!("{items}"))
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
                        Some(format!("{items}"))
                    }
                },
                ReviewCommand::Score { id, grade } => {
                    use sra::model::Grade::*;
                    use Maturity::*;

                    let item = queries::item::get(&mut c, id)?;
                    let id = item.id;

                    let today = time::OffsetDateTime::now_utc();
                    let duration_since_last_review = today - item.last_review_date;
                    let n_days_since_last_review = duration_since_last_review.as_seconds_f32()
                        / Duration::DAY.as_seconds_f32();
                    let last_review_was_today = Duration::DAY < duration_since_last_review;

                    match (item.maturity, grade, last_review_was_today) {
                        // we need to review the item again in this session
                        (New, Again | Hard, _) => {
                            queries::review::increment_n_reviews(&mut c, id)?;
                        }
                        // promote item from young to new
                        (New, g, _) => {
                            let s = sra::init::s(g);
                            let d = sra::init::d(g);
                            queries::review::set_maturity(&mut c, id, Young)?;
                            queries::review::increment_n_reviews(&mut c, id)?;
                            queries::review::set_sra_params(&mut c, id, s, d, today)?;
                        }
                        (Young | Tenured, Again, true) => {
                            // the item was already reviewed today, but somehow we are reviewing it again
                            // with a failing grade. This could be because the user is cramming review items
                        }
                        (Young | Tenured, Again, false) => {
                            let r = sra::r(n_days_since_last_review, item.stability);
                            let s = sra::update::fail::s(item.stability, item.difficulty, r);
                            let d = sra::update::d(item.difficulty, Again);
                            queries::review::increment_n_reviews(&mut c, id)?;
                            queries::review::increment_n_lapses(&mut c, id)?;
                            queries::review::set_sra_params(&mut c, id, s, d, today)?;
                        }
                        (Young | Tenured, g, true) => {
                            let s = sra::update::shortterm::s(item.stability, g);
                            let d = sra::update::d(item.difficulty, g);
                            if s > 100.0 {
                                queries::review::set_maturity(&mut c, id, Tenured)?;
                            }
                            queries::review::increment_n_reviews(&mut c, id)?;
                            queries::review::set_sra_params(&mut c, id, s, d, today)?;
                        }
                        (Young | Tenured, g, false) => {
                            let r = sra::r(n_days_since_last_review, item.stability);
                            let s = sra::update::success::s(item.stability, item.difficulty, r, g);
                            let d = sra::update::d(item.difficulty, g);
                            if s > 100.0 {
                                queries::review::set_maturity(&mut c, id, Tenured)?;
                            }
                            queries::review::increment_n_reviews(&mut c, id)?;
                            queries::review::set_sra_params(&mut c, id, s, d, today)?;
                        }
                    };
                    None
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
                    Some(format!("{res}"))
                }
            };
            Ok(res)
        }
    }

    pub mod tag {
        use serde_json::json;

        use super::*;

        pub fn handle_command(command: TagCommand) -> Result<Option<String>> {
            let mut c: Connection = db::open(&get_db_path()?)?;

            Ok(match command {
                TagCommand::Add { name } => {
                    let id = queries::tag::add(&mut c, &name)?;
                    Some(format!("{}", json!({ "id": id }).to_string()))
                }
                TagCommand::Edit { old_name, new_name } => {
                    queries::tag::edit(&mut c, &old_name, &new_name)?;
                    None
                }
                TagCommand::Query {
                    pretty,
                    pre_filter,
                    post_filter,
                } => {
                    let tags = queries::tag::query(&mut c, pre_filter)?;
                    let tags = jmessearch_and_prettify(tags, post_filter, pretty)?;
                    Some(format!("{tags}"))
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
