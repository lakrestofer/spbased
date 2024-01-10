use std::path::PathBuf;

use crate::{
    collection::{CollectionServer, CollectionService},
    scheduler::SchedulerService,
};
use grpc::scheduler_server::SchedulerServer;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use serde::{Deserialize, Serialize};
use tonic::transport::Server;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    server_address: String,
    // #[serde(skip_serializing)]
    db_dir: Option<PathBuf>, // path to the directory containing the sqlite db file
    // #[serde(skip_serializing)]
    db_name: Option<String>,
}

impl ServerConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            server_address: self.server_address,
            db_dir: self.db_dir.or(other.db_dir),
            db_name: self.db_name.or(other.db_name),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            // TODO mature and figure a better default port
            server_address: "[::1]:42069".into(),
            db_dir: Some(common::project_directory().unwrap().data_dir().into()),
            db_name: Some("db.sqlite".into()),
        }
    }
}

pub async fn run_server(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    // merge config with defaults
    let config = config.merge(ServerConfig::default());
    // all fields are now Some
    let server_address = config.server_address;
    let db_dir = config.db_dir.unwrap();
    let db_name = config.db_name.unwrap();
    let db_path = db_dir.join(db_name);
    let addr = server_address.parse().unwrap();

    // create directories if they do not exist
    std::fs::create_dir_all(db_dir)?;

    // connect to db and perform migration
    let protocol_str = format!("sqlite://{}?mode=rwc", db_path.to_str().unwrap());
    let db = Database::connect(protocol_str)
        .await
        .expect("could not connect to db");

    Migrator::up(&db, None)
        .await
        .expect("Could not perform a fresh migration");

    // build services and launch server
    let collection_service = CollectionService::new(db.clone());
    let collection_server = CollectionServer::new(collection_service);

    let scheduler_service = SchedulerService::new(db.clone());
    let scheduler_server = SchedulerServer::new(scheduler_service);

    Server::builder()
        .add_service(collection_server)
        .add_service(scheduler_server)
        .serve(addr)
        .await?;

    Ok(())
}
