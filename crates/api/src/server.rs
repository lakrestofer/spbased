use crate::collection::{CollectionServer, CollectionService};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tonic::transport::Server;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let protocol_str = format!("sqlite://db.sqlite?mode=rwc");
    let db = Database::connect(protocol_str)
        .await
        .expect("could not connect to db");

    Migrator::up(&db, None)
        .await
        .expect("Could not perform a fresh migration");

    let collection = CollectionService::new(db);

    let collection_server = CollectionServer::new(collection);

    Server::builder()
        .add_service(collection_server)
        .serve(addr)
        .await?;

    Ok(())
}
