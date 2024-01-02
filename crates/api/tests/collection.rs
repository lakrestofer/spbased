use futures::future::BoxFuture;
use futures::FutureExt;
// std lib imports
use grpc::{ListReviewItemsMessage, ListReviewItemsResponse, ResponseStatus, VersionInfo};
use std::future::Future;
use std::sync::Arc;
//internal imports
use grpc::collection_client::CollectionClient;
use grpc::collection_server::CollectionServer;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
// external imports
use tempfile::NamedTempFile;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tonic::{Request, Response};
use tower::service_fn;

/// Returns an unique connection to a sqlite instance and performs migrations on it.
pub async fn setup_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("Could not open in memory instance of sqlite database");

    Migrator::up(&db, None)
        .await
        .expect("Could not perform a fresh migration");

    db
}

// adapted from this -  https://stackoverflow.com/questions/69845664/how-to-integration-test-tonic-application
pub async fn collection_server_and_client() -> (impl Future<Output = ()>, CollectionClient<Channel>)
{
    let socket = NamedTempFile::new().unwrap();
    let socket = Arc::new(socket.into_temp_path());
    std::fs::remove_file(&*socket).unwrap();

    let uds = UnixListener::bind(&*socket).unwrap();
    let stream = UnixListenerStream::new(uds);

    let serve_future = async {
        let db = setup_db().await; // get a connection to an unique db
        let result = Server::builder()
            .add_service(CollectionServer::new(
                api::collection::CollectionService::new(db),
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

    let client = grpc::collection_client::CollectionClient::new(channel);

    (serve_future, client)
}

async fn run_test<A, B, FMethod, FCompare>(
    request: Request<A>,
    method: FMethod,
    compare_method: FCompare,
) where
    FMethod: Fn(&'_ mut CollectionClient<Channel>, Request<A>) -> BoxFuture<'_, B>,
    FCompare: Fn(B),
    B: PartialEq + std::fmt::Debug,
{
    let (serve_future, mut client) = collection_server_and_client().await;
    let request_future = async {
        let response = method(&mut client, request).await;
        compare_method(response);
    };

    // the future that completes first will return.
    // only the request future should return.
    // if the server returns that is because it crashed
    tokio::select! {
        _ = serve_future => panic!("server returned first"),
        _ = request_future => (),
    }
}

#[tokio::test]
/// tests that the above collection_server_and_client function works
async fn migration_and_collection_service_creation() {
    let (_serve_future, _client) = collection_server_and_client().await;
}

#[tokio::test]
/// does a db without any items return an empty list?
async fn test_list() {
    let request = Request::new(ListReviewItemsMessage {
        version: Some(VersionInfo {
            api_version: "0.0.1".into(),
        }),
        page_token: "".into(),
        page_size: 0,
        order_by: "".into(),
        order_dir: "".into(),
        filter: "".into(),
    });
    let expected = ListReviewItemsResponse {
        version: Some(VersionInfo {
            api_version: "0.0.1".into(),
        }),
        status: Some(ResponseStatus {
            code: 200,
            message: None,
        }),
        next_page_token: "".into(),
        items: Vec::new(),
    };
    run_test(
        request,
        |client, request| {
            async move {
                client
                    .list_review_items(request)
                    .await
                    .unwrap()
                    .into_inner()
            }
            .boxed()
        },
        |response| {
            assert_eq!(response, expected);
        },
    )
    .await;
}
