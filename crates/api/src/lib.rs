use std::cell::OnceCell;

pub mod server;
pub mod types;

mod collection;
mod scheduler;

use grpc::VersionInfo;
use sea_orm::{ActiveValue, DatabaseConnection};
pub const VERSION: OnceCell<VersionInfo> = OnceCell::new();
pub fn version() -> VersionInfo {
    VERSION
        .get_or_init(|| VersionInfo {
            api_version: "0.0.1".into(),
        })
        .clone()
}

/// The main Service struct on which all the grpc service traits are implemented.
#[derive(Debug)]
pub struct ServiceProvider {
    db: DatabaseConnection,
}

impl ServiceProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
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
