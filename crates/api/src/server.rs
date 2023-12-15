// external imports
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use tonic::{Code, Request, Response, Status};
// stdlib imports
use std::cell::OnceCell;
// internal imports
// workspace imports
use entity::review_item::Entity as DbReviewItem;
use grpc::collection_server::{Collection, CollectionServer};
use grpc::{
    CreateReviewItemMessage, CreateReviewItemResponse, DeleteReviewItemMessage,
    DeleteReviewItemResponse, GetReviewItemMessage, GetReviewItemResponse, ListReviewItemsMessage,
    ListReviewItemsResponse, UpdateReviewItemMessage, UpdateReviewItemResponse,
};
use grpc::{ResponseStatus, VersionInfo};

#[derive(Debug)]
struct CollectionService {
    db: DatabaseConnection,
    // TODO pagination on list response, this requires that some state is saved (to DB?)
}

const VERSION: OnceCell<VersionInfo> = OnceCell::new();
fn version() -> VersionInfo {
    VERSION
        .get_or_init(|| VersionInfo {
            api_version: "0.0.1".into(),
        })
        .clone()
}

fn db_err_to_status(err: DbErr) -> Status {
    // TODO do we need to handle the errors any any more robust way than this?
    // might be nice to differenciate between not having enough
    Status::new(Code::Unavailable, "could not access db")
}

#[tonic::async_trait]
impl Collection for CollectionService {
    async fn list_review_items(
        &self,
        request: Request<ListReviewItemsMessage>,
    ) -> Result<Response<ListReviewItemsResponse>, Status> {
        let _message: ListReviewItemsMessage = request.into_inner();
        // TODO implement checking of version info
        // this might be something that we want to use middleware for?
        // TODO implement filtering based on the request message
        let items: Vec<_> = DbReviewItem::find()
            .all(&self.db)
            .await
            .map_err(db_err_to_status)?;

        // convert from dbreviewitems (models) into the rRPC version
        let items = items.into_iter().map(From::from).collect();

        let response = ListReviewItemsResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: "".into(),
            }),
            next_page_token: "".into(), // empty for now
            items,
        };

        Ok(Response::new(response))
    }

    async fn get_review_item(
        &self,
        request: Request<GetReviewItemMessage>,
    ) -> Result<Response<GetReviewItemResponse>, Status> {
        let message: GetReviewItemMessage = request.into_inner();

        let item = DbReviewItem::find_by_id(&message.name)
            .one(&self.db)
            .await
            .map_err(db_err_to_status)?;

        if item.is_none() {
            // there were no item with this
            return Err(Status::new(
                Code::NotFound,
                format!("Could not find review item with name: {}", message.name),
            ));
        }

        let item = item.map(From::from);

        return Ok(Response::new(GetReviewItemResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: "".into(),
            }),
            item,
        }));
    }

    async fn create_review_item(
        &self,
        request: Request<CreateReviewItemMessage>,
    ) -> Result<Response<CreateReviewItemResponse>, Status> {
        todo!();
    }

    async fn update_review_item(
        &self,
        request: Request<UpdateReviewItemMessage>,
    ) -> Result<Response<UpdateReviewItemResponse>, Status> {
        todo!();
    }

    async fn delete_review_item(
        &self,
        request: Request<DeleteReviewItemMessage>,
    ) -> Result<Response<DeleteReviewItemResponse>, Status> {
        todo!();
    }
}
