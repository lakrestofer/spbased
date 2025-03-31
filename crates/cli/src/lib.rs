//! Spbasedctl implementation
use clap::{Parser, Subcommand};
use eyre::eyre;
use eyre::Result;
use eyre::WrapErr;
use normalize_path::NormalizePath;
use resolve_path::PathResolveExt;
use rusqlite::params;
use rusqlite::params_from_iter;
use rusqlite::Connection;
use serde_json::json;
use std::path::PathBuf;
use time::OffsetDateTime;

pub mod cli;
pub mod queries;

use cli::*;
use db::DB;

pub const SPBASED_ROOT_ENV_VAR_NAME: &'static str = "SPBASED_ROOT";
pub const SPBASED_DATA_DIR: &'static str = "spbased";
pub const SPBASED_WORK_DIR: &'static str = ".spbased";
pub const SPBASED_DB_NAME: &'static str = "db.sqlite";
pub const SPBASED_CONFIG_NAME: &'static str = "config.toml";

// ======= CLI COMMAND HANDLERS BEGIN ======
pub fn handle_command(root: Option<PathBuf>, command: Command) -> Result<Option<String>> {
    log::debug!("handling command: {:?}", command);
    Ok(match command {
        Command::Init { directory, force } => {
            command::init(directory, force)?;
            None
        }
        command => {
            let spbased_dir = get_spbased_dir(root)?;
            log::debug!("spbased_working_dir set to {:?}", &spbased_dir);
            let db_dir = get_db_path(spbased_dir);
            log::debug!("spbased_db_dir set to {:?}", &db_dir);
            let db = DB::open(&db_dir)?;
            match command {
                Command::Items(command) => command::item::handle_command(db, command)?,
                Command::Review(command) => command::review::handle_command(db, command)?,
                Command::Tags(command) => command::tag::handle_command(db, command)?,
                _ => unreachable!(),
            }
        }
    })
}

pub mod command {

    use super::*;

    /// Init a new .spbased directory containing a sqlite db instance
    /// and a config file
    pub fn init(directory: Option<PathBuf>, force: bool) -> Result<()> {
        let (full_path, spbased_dir) = match directory {
            Some(d) => {
                let full_path: PathBuf = d.try_resolve()?.into_owned().normalize();
                let spbased_dir = full_path.join(SPBASED_WORK_DIR);
                (full_path, spbased_dir)
            }
            None => {
                let full_path =
                    dirs::data_local_dir().ok_or(eyre!("could not find a local data directory"))?;
                let spbased_dir = full_path.join(SPBASED_DATA_DIR);
                (full_path, spbased_dir)
            }
        };
        log::info!("initializing spbased dir at {:?}", full_path);

        // confirm that user wants to overwrite dir
        if spbased_dir.exists() {
            if !force {
                log::warn!("{:?} already exists", spbased_dir);
                log::warn!("pass --force if you are sure");
                return Ok(());
            }
            std::fs::remove_dir_all(&spbased_dir)?;
        }

        // create the directory
        std::fs::create_dir_all(&spbased_dir)?;

        // init the db
        _ = db::DB::open(&spbased_dir.join(SPBASED_DB_NAME))?;

        Ok(())
    }

    pub mod item {

        use db::DB;

        use super::*;

        pub fn handle_command(mut c: DB, command: ItemCommand) -> Result<Option<String>> {
            Ok(match command {
                ItemCommand::Add { model, data, tags } => {
                    let id = queries::item::add(
                        &mut c,
                        &model,
                        &data.to_string(),
                        &(tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
                    )?;
                    Some(format!("{}", json!({ "id": id }).to_string()))
                }
                ItemCommand::Edit {
                    id,
                    model,
                    data,
                    add_tags,
                    remove_tags,
                } => {
                    if let Some(model) = model {
                        queries::item::edit_model(&mut c, id, &model)?;
                    }
                    if let Some(data) = data {
                        queries::item::edit_data(&mut c, id, &data.to_string())?;
                    }
                    if add_tags.len() > 0 {
                        queries::item::add_tags(
                            &mut c,
                            id,
                            &(add_tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
                        )?;
                    }
                    if remove_tags.len() > 0 {
                        queries::item::remove_tags(
                            &mut c,
                            id,
                            &(remove_tags
                                .iter()
                                .map(|s| s.as_str())
                                .collect::<Vec<&str>>()),
                        )?;
                    }
                    None
                }
                ItemCommand::Delete { id } => {
                    queries::item::delete(&mut c, id)?;
                    None
                }
                ItemCommand::GetTags {
                    id,
                    post_filter,
                    pretty,
                } => {
                    let tags = queries::item::get_tags(&mut c, id)?;
                    let tags = jmessearch_and_prettify(tags, post_filter, pretty)?;
                    Some(format!("{tags}"))
                }
                ItemCommand::Query {
                    pre_filter,
                    post_filter,
                    include_tags,
                    exclude_tags,
                    pretty,
                } => {
                    // we apply sql filtering on items
                    let items = queries::item::query(
                        &mut c,
                        pre_filter,
                        &(include_tags
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>()),
                        &(exclude_tags
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>()),
                    )?;
                    // we apply json filter on items
                    let items = jmessearch_and_prettify(items, post_filter, pretty)?;
                    // TODO check README TODO for what to do here
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
        use rand::Rng;
        use time::Duration;

        use super::*;
        // use serde_json::json;

        pub fn handle_command(mut c: DB, command: ReviewCommand) -> Result<Option<String>> {
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
                        Some(format!("{}", items))
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
                        // promote item from new to young
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
                            let s = s * (1.0 + rand::rng().random_range(-0.1..=0.1)); // add some random noise on ordinary reviews
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

        pub fn handle_command(mut c: DB, command: TagCommand) -> Result<Option<String>> {
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

pub fn get_spbased_dir_cwd() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let spbased_dir = cwd.join(SPBASED_WORK_DIR);
    if spbased_dir.is_dir() {
        Ok(spbased_dir)
    } else {
        Err(eyre!(
            "Could not find .spbased directory in current working directory"
        ))
    }
}

pub fn get_spbased_dir_user_data() -> Result<PathBuf> {
    let user_data_dir =
        dirs::data_local_dir().ok_or(eyre!("Could not find user data directory"))?;
    let spbased_dir = user_data_dir.join(SPBASED_DATA_DIR);
    if spbased_dir.is_dir() {
        Ok(spbased_dir)
    } else {
        Err(eyre!(
            "Could not find spbased directory in {:?}",
            user_data_dir
        ))
    }
}

/// Retrieve the spbased directory containing the config file and sqlite db.
/// Checked in this order: --root flag, environment variable, current directory,
/// xdg user data directory
pub fn get_spbased_dir(root: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = root {
        // if --root flag is supplied it must be valid, otherwise return
        return get_spbased_dir_flag(path);
    }
    get_spbased_dir_cwd() // first check in current dir
        .or_else(|_| get_spbased_dir_env_var()) // then in $SPBASED_DIR
        .or_else(|_| get_spbased_dir_user_data()) // then in xdg user data dir
        .map_err(|_| eyre!("Could not find spbased directory"))
}

pub fn get_spbased_dir_env_var() -> Result<PathBuf> {
    log::debug!("retriving spbased directory from SPBASED_ROOT");
    match std::env::var(SPBASED_ROOT_ENV_VAR_NAME) {
        Ok(path) => validate_spbased_root_dir(path.into()),
        Err(e) => Err(eyre!("Could not find spbased dir: {:?}", e)),
    }
}

pub fn get_spbased_dir_flag(root: PathBuf) -> Result<PathBuf> {
    log::debug!(
        "retriving spbased directory from current working directory: {:?}",
        root
    );
    validate_spbased_root_dir(root)
}

pub fn validate_spbased_root_dir(path: PathBuf) -> Result<PathBuf> {
    log::debug!("checking if {:?} contains {:?}", path, SPBASED_WORK_DIR);
    let spbased_dir = path.join(SPBASED_WORK_DIR);
    if spbased_dir.is_dir() {
        Ok(spbased_dir)
    } else {
        Err(eyre!(
            "Could not find .spbased directory in current working directory"
        ))
    }
}

pub fn get_db_path(spbased_dir: PathBuf) -> PathBuf {
    spbased_dir.join(SPBASED_DB_NAME)
}

// ======= DB WRAPPER AND DATA MODEL BEGIN BEGIN ======

pub mod db {
    use super::*;
    use rusqlite::{Connection, OpenFlags};
    use sql_minifier::macros::load_sql;
    use std::{
        cell::LazyCell,
        ops::{Deref, DerefMut},
        path::Path,
    };

    use rusqlite_migration::{Migrations, M};

    pub const DB_OPEN: &str = load_sql!("sql/db_open.sql");
    pub const DB_CLOSE: &str = load_sql!("sql/db_close.sql");

    pub const MIGRATIONS: LazyCell<Migrations> =
        LazyCell::new(|| Migrations::new(vec![M::up(load_sql!("sql/001_init.sql"))]));

    #[repr(transparent)]
    pub struct DB(Connection);

    impl DB {
        pub fn open<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<DB> {
            log::debug!("opening db at {:?}", path);
            // open and create a sqlite db
            let mut conn = Connection::open(path).wrap_err("trying to open connection")?;

            conn.execute_batch(DB_OPEN)?;

            MIGRATIONS.to_latest(&mut conn)?;

            Ok(DB(conn))
        }
    }

    // util traits
    impl Drop for DB {
        fn drop(&mut self) {
            self.0
                .execute_batch(DB_CLOSE)
                .wrap_err("trying to apply pragmas at close")
                .unwrap();
        }
    }
    impl Deref for DB {
        type Target = Connection;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for DB {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        pub fn init() -> Result<()> {
            Ok(())
        }
    }
}
