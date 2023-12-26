use std::cell::OnceCell;

pub mod server;
pub mod types;

mod collection;
mod scheduler;

use grpc::VersionInfo;
pub const VERSION: OnceCell<VersionInfo> = OnceCell::new();
pub fn version() -> VersionInfo {
    VERSION
        .get_or_init(|| VersionInfo {
            api_version: "0.0.1".into(),
        })
        .clone()
}
