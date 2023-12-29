use std::cell::OnceCell;

pub mod error;
pub mod server;
#[cfg(test)]
pub mod test;
pub mod types;

mod collection;
mod scheduler;

use grpc::VersionInfo;
use sea_orm::{ActiveValue, DbErr};
use tonic::{Code, Status};
pub const VERSION: OnceCell<VersionInfo> = OnceCell::new();
pub fn version() -> VersionInfo {
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
