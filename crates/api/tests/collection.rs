use common::ReviewItemStatus;
use futures::future::BoxFuture;
use futures::FutureExt;
// std lib imports
use grpc::{
    CreateReviewItemMessage, CreateReviewItemResponse, GetReviewItemMessage,
    ListReviewItemsMessage, ListReviewItemsResponse, NewReviewItem, ResponseStatus, ReviewItem,
    VersionInfo,
};
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
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
use tonic::Request;
use tower::service_fn;


// assertions
use pretty_assertions::assert_eq;


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

async fn run_test<F>(method: F)
where
    F: Fn(&'_ mut CollectionClient<Channel>) -> BoxFuture<'_, ()>, // the method function defines which endpoint on the client should call and how we want to interpret the result
{
    let (serve_future, mut client) = collection_server_and_client().await;
    let request_future = async {
        method(&mut client).await;
    };

    // the future that completes first will return.
    // only the request future should return.
    // if the server returns first that is because it crashed
    tokio::select! {
        _ = serve_future => panic!("server returned first"),
        _ = request_future => (),
    }
}

// ============ let the tests begin!!! ==============

#[tokio::test]
/// tests that the above collection_server_and_client function works
async fn migration_and_collection_service_creation() {
    let (_serve_future, _client) = collection_server_and_client().await;
}

#[tokio::test]
/// A just initialized spbased service should return an empty items list
async fn new_service() {
    run_test(|client| {
        let request = Request::new(ListReviewItemsMessage {
            version: Some(VersionInfo {
                api_version: "0.0.1".into(),
            }),
            page: 0,
            page_size: 50,
            order_by: None,
            order_dir: None,
            filter: None,
        });

        // the expected response
        // TODO will have to change after implementation of pagination
        let expected = ListReviewItemsResponse {
            version: Some(VersionInfo {
                api_version: "0.0.1".into(),
            }),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
            }),
            total_items: 0,
            page_size: 50,
            items: Vec::new(),
        };

        // return a future that performs the actual request and validation logic
        async move {
            let response = client
                .list_review_items(request)
                .await
                .unwrap()
                .into_inner();
            assert_eq!(response, expected);
        }
        .boxed()
    })
    .await;
}

#[tokio::test]
/// TODO Create a few items and filter on various attributes
async fn create_review_item() {
    run_test(|client| {
        let request = CreateReviewItemMessage {
            version: Some(api::version()),
            item: Some(NewReviewItem {
                item_type: "flashcard".into(),
                data: "'front':'question','back':'answer'".into(),
            }),
        };

        async move {
            let CreateReviewItemResponse {
                version,
                status,
                item,
            } = client
                .create_review_item(request)
                .await
                .expect("could not perform create review call")
                .into_inner();

            // NOTE: If this test fails it might be because the entity crate recently was re-generated using the generate_entities script
            // Make sure that the 'before_save' hook is still provided an implementation

            let version = version.expect("no version field was provided!");
            let status = status.expect("no status field was provided!");

            assert_eq!(version, api::version());
            assert_eq!(status.code, 200);

            let ReviewItem {
                name,
                create_time,
                update_time,
                status,
                difficulty,
                stability,
                last_review_date,
                next_review_date,
                item_type,
                url,
                data: _data,
            } = item.expect("no item field was provided!");

            // name should be combination of item_type and uuid (collection id and resource id), as noted here https://cloud.google.com/apis/design/resource_names
            let mut name_uuid = name.split('/');
            let name_item_type = name_uuid.next().expect("name did not contain '/'");
            assert!(name_item_type == "flashcard");
            let uuid = name_uuid.next().expect("name did not contain '/'");
            let uuid = Uuid::from_str(uuid).expect("could not parse uuid from string");
            assert_eq!(uuid.get_version(), Some(uuid::Version::Random));

            // create time and first update should be equal and also relative to the UTC timezone
            let create_time = chrono::DateTime::parse_from_rfc3339(&create_time)
                .expect("create_time fields could not be parsed into datetime!");
            let update_time = chrono::DateTime::parse_from_rfc3339(&update_time)
                .expect("update_time fields could not be parsed into datetime!");
            assert_eq!(create_time, update_time);

            // the status should be Inbox
            assert_eq!(status, ReviewItemStatus::Inbox.to_string());

            // the difficulty and stability should be 0 until the first grading
            assert!(difficulty == 0.0 && stability == 0.0);

            // last review date and next review date should be empty strings
            assert!(&last_review_date == "" && &next_review_date == "");

            // the item type should be equal to the provided type
            assert_eq!(&item_type, "flashcard");

            // the url should contain the item_type as url scheme, followed by the name;
            let mut url_scheme_and_name = url.split("://");
            let scheme = url_scheme_and_name
                .next()
                .expect("url did not contain '://'");
            let url_name = url_scheme_and_name
                .next()
                .expect("url did not contain '://'");
            assert_eq!(scheme, item_type);
            assert_eq!(url_name, name);

            // the data may be anything
        }
        .boxed()
    })
    .await;
}

#[tokio::test]
/// TODO retrieve specific review item, "after creation"
async fn review_item_get() {
    run_test(|client| {
        let request = CreateReviewItemMessage {
            version: Some(api::version()),
            item: Some(NewReviewItem {
                item_type: "flashcard".into(),
                data: "'front':'question','back':'answer'".into(),
            }),
        };
        async move {
            let create_response = client
                .create_review_item(request)
                .await
                .expect("did not retrieve response from create review item request")
                .into_inner();
            let version = create_response
                .version
                .expect("version field did not exist");
            assert_eq!(version, api::version());
            let status = create_response.status.expect("status field did not exist");
            assert_eq!(status.code, 200);

            let item = create_response.item.expect("item field did not exist");

            // using the item we got back we try to get it again
            let get_response = client
                .get_review_item(GetReviewItemMessage {
                    version: Some(api::version()),
                    name: item.name.clone(),
                })
                .await
                .expect("could not get item")
                .into_inner();

            let version = get_response.version.expect("version field did not exist");
            let status = get_response.status.expect("status field did not exist");
            assert_eq!(version, api::version());
            assert_eq!(status.code, 200);

            let item_again = get_response
                .item
                .expect("could not retrieve item, was None");

            assert_eq!(item, item_again);
        }
        .boxed()
    })
    .await;
}

#[tokio::test]
/// TODO Create a few items and filter on various attributes
async fn review_item_list() {
    run_test(|client| {
        let requests = [
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "flashcard".into(),
                    data: "'front':'capital of sweden','back':'stockholm'".into(),
                }),
            },
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "flashcard".into(),
                    data: "'front':'capital of sweden','back':'stockholm'".into(),
                }),
            },
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "cloze".into(),
                    data: "'text':'the capital of sweden is stockholm', 'mask':'                         ---------'" .into(),
                }),
            },
        ];
        async move {
            // we create a few items
            for request in requests {
                client
                    .create_review_item(request)
                    .await
                    .expect("did not retrieve response from create review item request")
                    .into_inner();
            }

            // then we return them again
            let list_response = client
                .list_review_items(ListReviewItemsMessage {
                    version: Some(api::version()),
                    page: 0,
                    page_size: 50,
                    order_by: None,
                    order_dir: None,
                    filter: None,
                })
                .await
                .expect("did not retrieve a list of items!")
                .into_inner();

            let status = list_response.status.expect("could not retrieve status from response");
            let version = list_response.version.expect("could not retrieve version from response");
            let items = list_response.items;
            assert_eq!(version, api::version());
            assert_eq!(status.code, 200);
            assert_eq!(version, api::version());
            assert_eq!(status.code, 200);

            // we expect 3 items in the result
            assert_eq!(items.len(), 3);
        }
        .boxed()
    })
    .await;
}

#[tokio::test]
// Check that pagination without filtering or sorting results works
async fn review_item_list_pagination() {
    run_test(|client| {
        let requests = [
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "flashcard".into(),
                    data: "'front':'capital of norway','back':'oslo'".into(),
                }),
            },
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "flashcard".into(),
                    data: "'front':'capital of sweden','back':'stockholm'".into(),
                }),
            },
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "cloze".into(),
                    data: "'text':'the capital of sweden is stockholm', 'mask':'                         ---------'" .into(),
                }),
            },
        ];
        async move {
            // we create a few items
            for request in requests {
                client
                    .create_review_item(request)
                    .await
                    .expect("did not retrieve response from create review item request")
                    .into_inner();
            }

            for i in 0..=2 {
            // then we return them again
                let list_response = client
                    .list_review_items(ListReviewItemsMessage {
                        version: Some(api::version()),
                        page: i,
                        page_size: 1,
                        order_by: None,
                        order_dir: None,
                        filter: None,
                    })
                    .await
                    .expect("did not retrieve a list of items!")
                    .into_inner();

                let status = list_response.status.expect("could not retrieve status from response");
                let version = list_response.version.expect("could not retrieve version from response");
                let items = list_response.items;
                assert_eq!(version, api::version());
                assert_eq!(status.code, 200);
                assert_eq!(version, api::version());
                assert_eq!(status.code, 200);

                // we expect 1 items in the result
                assert_eq!(items.len(), 1);
                
            }
        }
        .boxed()
    })
    .await;
}


#[tokio::test]
// Check that pagination without filtering or sorting results works
async fn review_item_list_pagination_2() {
    run_test(|client| {
        let requests = [
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "flashcard".into(),
                    data: "'front':'capital of norway','back':'oslo'".into(),
                }),
            },
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "flashcard".into(),
                    data: "'front':'capital of sweden','back':'stockholm'".into(),
                }),
            },
            CreateReviewItemMessage {
                version: Some(api::version()),
                item: Some(NewReviewItem {
                    item_type: "cloze".into(),
                    data: "'text':'the capital of sweden is stockholm', 'mask':'                         ---------'" .into(),
                }),
            },
        ];
        async move {
            // we create a few items
            for request in requests {
                client
                    .create_review_item(request)
                    .await
                    .expect("did not retrieve response from create review item request")
                    .into_inner();
            }

            // ======== first we get the list of all items =========
            // then we return them again
            let list_response = client
                .list_review_items(ListReviewItemsMessage {
                    version: Some(api::version()),
                    page: 0,
                    page_size: 50,
                    order_by: None,
                    order_dir: None,
                    filter: None,
                })
                .await
                .expect("did not retrieve a list of items!")
                .into_inner();

            let status = list_response.status.expect("could not retrieve status from response");
            let version = list_response.version.expect("could not retrieve version from response");
            assert_eq!(version, api::version());
            assert_eq!(status.code, 200);
            assert_eq!(version, api::version());
            assert_eq!(status.code, 200);

            // we expect 3 items in the result
            let items = list_response.items;
            assert_eq!(items.len(), 3);

            let mut paginated_items = Vec::new();
            

            // then we retrieve them again but paginated
            for i in 0..=2 {
                let list_response = client
                    .list_review_items(ListReviewItemsMessage {
                        version: Some(api::version()),
                        page: i,
                        page_size: 1,
                        order_by: None,
                        order_dir: None,
                        filter: None,
                    })
                    .await
                    .expect("did not retrieve a list of items!")
                    .into_inner();

                let status = list_response.status.expect("could not retrieve status from response");
                let version = list_response.version.expect("could not retrieve version from response");
                let items = list_response.items;
                assert_eq!(version, api::version());
                assert_eq!(status.code, 200);
                assert_eq!(version, api::version());
                assert_eq!(status.code, 200);

                // we expect 3 items in the result
                assert_eq!(items.len(), 1);

                let item = items[0].clone();
                paginated_items.push(item);
            }

            // we then check that the first list were we retrieved all items is equal 
            // to the second list which we filled incrementally

            assert_eq!(items, paginated_items);
        }
        .boxed()
    })
    .await;
}



#[tokio::test]
/// TODO Create a few items and filter on various attributes
async fn review_item_list_filtering() {
    todo!("Filtering on review item list endpoint");
}

#[tokio::test]
/// TODO Create a few items and sort on various attributes
async fn review_item_list_sort() {
    todo!("Sort on review item list endpoint");
}
