// std lib imports
use std::future::Future;
use std::sync::Arc;
//internal imports
use grpc::collection_client::CollectionClient;
use grpc::collection_server::CollectionServer;
use grpc::scheduler_server::SchedulerServer;

use migration::{Migrator, MigratorTrait};
// external imports
use tempfile::NamedTempFile;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Request, Response};

/// Returns an unique connection to a sqlite instance and performs migrations on it.
pub async fn setup_db() -> DbConnection {
    let db = Database::connect("sqlite::memory:").expect("");

    Migrator::up(&db, None)
        .await
        .expect("Could not perform a fresh migration");

    db
}

pub async fn collection_server_and_client_stub(
) -> (impl Future<Output = ()>, CollectionClient<Channel>) {
    let socket = NamedTempFile::new().unwrap();
    let socket = Arc::new(socket.into_temp_path());
    std::fs::remove_file(&*socket).unwrap();

    let uds = UnixListener::bind(&*socket).unwrap();
    let stream = UnixListenerStream::new(uds);

    let serve_future = async {
        let db = setup_db();
        let result = Server::builder()
            .add_service(CollectionServer::new(
                crate::collection::CollectionService { db: db.clone() },
            ))
            .add_service(CollectionServer::new(
                crate::collection::CollectionService { db: db },
            ))
            .serve_with_incoming(stream)
            .await;
        // Server must be running fine...
        assert!(result.is_ok());
    };

    let socket = Arc::clone(&socket);
    // Connect to the server over a Unix socket
    // The URL will be ignored.
    let channel = Endpoint::try_from("http://any.url")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket = Arc::clone(&socket);
            async move { UnixStream::connect(&*socket).await }
        }))
        .await
        .unwrap();

    let client = MerchantServiceClient::new(channel);

    (serve_future, client)
}
