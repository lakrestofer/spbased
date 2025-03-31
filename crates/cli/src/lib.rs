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
use std::cell::LazyCell;
use std::path::PathBuf;
use time::OffsetDateTime;

pub mod cli;
pub mod queries;

use cli::*;
use db::DB;

pub const APP_NAME: &'static str = "spbased";
pub const DEFAULT_APP_CONFIG_DIR: LazyCell<PathBuf> =
    LazyCell::new(|| dirs::config_dir().unwrap().join(APP_NAME));
pub const DB_NAME: &'static str = "db.sqlite";
pub const CONFIG_NAME: &'static str = "config.toml";

// ======= CLI COMMAND HANDLERS BEGIN ======
pub fn handle_command(root: Option<PathBuf>, command: Command) -> Result<Option<String>> {
    log::debug!("handling command: {:?}", command);
    Ok(match command {
        Command::Init { directory, force } => {
            command::init(directory, force)?;
            None
        }
        command => {
            // TODO root should be a AppRoot struct, we need to resolve the root in a similar
            // way to how we resolve the AppConfig. It needs to be environment aware.
            let root = config::AppRoot::try_resolve(root)?;
            let config = config::AppConfig::resolve(root)?;
            log::debug!("spbased config set to {:?}", &config);
            let db = DB::open(&config.db_path)?;
            match command {
                Command::Items(command) => command::item::handle_command(db, command)?,
                Command::Review(command) => command::review::handle_command(db, command)?,
                Command::Tags(command) => command::tag::handle_command(db, command)?,
                _ => unreachable!(),
            }
        }
    })
}

pub mod config {
    use super::*;
    use figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment,
    };
    use serde::{Deserialize, Serialize};
    use std::path::{Path, PathBuf};

    use crate::APP_NAME;

    #[derive(Default, Debug, Deserialize, Serialize)]
    pub struct AppRoot {
        pub root: PathBuf,
    }

    impl AppRoot {
        pub fn new(root: PathBuf) -> Self {
            Self { root }
        }

        pub fn try_resolve(root: Option<PathBuf>) -> Result<Self> {
            // if we were supplied a valid root, return it
            if let Some(root) = root {
                if app_work_dir(&root).is_dir() {
                    return Ok(AppRoot::new(root));
                }
            }
            // otherwise seach in the current dir and up
            let mut dir = std::path::absolute(std::env::current_dir()?)?;
            log::debug!("resolving spbased root directory, starting from {:?}", dir);
            loop {
                if app_work_dir(&dir).is_dir() {
                    return Ok(AppRoot::new(dir));
                }
                match dir.parent() {
                    Some(p) => dir = p.to_owned(),
                    None => {
                        log::warn!("could not resolve parent of {:?}", dir);
                        break;
                    }
                }
            }

            // otherwise check environment variables (SPBASED_ROOT)
            let figment =
                Figment::new().merge(Env::prefixed(&format!("{APP_NAME}_").to_uppercase()));

            if let Ok(app_root) = figment.extract() {
                return Ok(app_root);
            }

            Err(eyre!("could not resolve spbased root directory"))
        }
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    pub struct AppConfig {
        #[serde(skip)]
        pub app_root: PathBuf,
        #[serde(skip)]
        pub db_path: PathBuf,
    }

    impl AppConfig {
        pub fn new(app_root: PathBuf, db_path: PathBuf) -> Self {
            Self { app_root, db_path }
        }
    }

    /// the .spbased directory
    pub fn app_work_dir(dir: &Path) -> PathBuf {
        dir.to_owned().join(format!(".{APP_NAME}"))
    }

    /// .spbased/config.toml
    pub fn config_file_path(config_root: &Path) -> PathBuf {
        config_root.to_owned().join(CONFIG_NAME)
    }

    /// .spbased/db.sqlite
    pub fn db_file_path(data_root: &Path) -> PathBuf {
        data_root.to_owned().join(DB_NAME)
    }

    impl AppConfig {
        pub fn resolve(app_root: AppRoot) -> Result<Self> {
            let AppRoot { root: app_root } = app_root;
            let work_dir = app_work_dir(&app_root);
            let db_path = db_file_path(&work_dir);

            let figment = Figment::new()
                .merge(Serialized::defaults(Self::default()))
                .merge(Toml::file(config_file_path(&DEFAULT_APP_CONFIG_DIR))) // .config/markz
                .merge(Env::prefixed(&format!("{APP_NAME}_").to_uppercase())) // from enviornment variables
                .merge(Toml::file(config_file_path(&work_dir)));

            let mut config: Self = figment.extract()?;
            config.app_root = app_root;
            config.db_path = db_path;

            Ok(config)
        }
    }
}

pub mod command {

    use super::*;

    /// Init a new .spbased directory containing a sqlite db instance
    /// and a config file
    pub fn init(directory: PathBuf, force: bool) -> Result<()> {
        let full_path: PathBuf = directory.try_resolve()?.into_owned().normalize();
        let spbased_dir = config::app_work_dir(&full_path);
        log::info!("initializing spbased dir {:?}", &spbased_dir);

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
        _ = db::DB::open(&spbased_dir.join(DB_NAME))?;

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

// ======= DB WRAPPER AND DATA MODEL BEGIN BEGIN ======

pub mod db {
    use super::*;
    use rusqlite::Connection;
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
