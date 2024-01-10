use std::sync::OnceLock;

pub mod client;
pub mod server;

pub mod error;
pub mod types;

pub mod collection;
pub mod scheduler;

use grpc::VersionInfo;
use sea_orm::{ActiveValue, DbErr};
use tonic::{Code, Status};

pub const DEFAULT_PAGE_SIZE: i32 = 50;

pub fn version() -> VersionInfo {
    // right now the versioninfo is totaly known statically
    // I would like to read in the api version from some
    // configuration file later at build time
    static VERSION: OnceLock<VersionInfo> = OnceLock::new();
    VERSION
        .get_or_init(|| VersionInfo {
            api_version: "0.0.1".into(),
        })
        .clone()
}

trait OptionToActiveValue<T: Into<migration::Value>> {
    fn into_active_value(self) -> ActiveValue<T>;
}

impl<T> OptionToActiveValue<T> for Option<T>
where
    T: Into<migration::Value>,
{
    fn into_active_value(self) -> ActiveValue<T> {
        match self {
            Some(value) => ActiveValue::Set(value),
            None => ActiveValue::NotSet,
        }
    }
}

fn db_err_to_status(err: DbErr) -> Status {
    let err: String = format!("something went wrong on the db layer: {}", err.to_string());
    Status::new(Code::Unavailable, &err)
}
