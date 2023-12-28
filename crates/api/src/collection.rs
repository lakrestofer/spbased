//! Collection service. Provides the basic review item manipulation api endpoints

// external imports
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, EntityTrait, Set};
use tonic::{Code, Request, Response, Status};
// internal imports
// workspace imports
use entity::review_item::ActiveModel as ReviewItemActiveModel;
use entity::review_item::Entity as ReviewItemEntity;
use entity::review_item::Model as ReviewItemModel;
pub use grpc::collection_server::{Collection, CollectionServer};
pub use grpc::ResponseStatus;
pub use grpc::{
    CreateReviewItemMessage, CreateReviewItemResponse, DeleteReviewItemMessage,
    DeleteReviewItemResponse, GetReviewItemMessage, GetReviewItemResponse, ListReviewItemsMessage,
    ListReviewItemsResponse, UpdateReviewItemMessage, UpdateReviewItemResponse,
};

use crate::{db_err_to_status, version, OptionToActiveValue};

#[derive(Debug)]
pub struct CollectionService {
    db: DatabaseConnection,
}

impl CollectionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl Collection for CollectionService {
    async fn list_review_items(
        &self,
        request: Request<ListReviewItemsMessage>,
    ) -> Result<Response<ListReviewItemsResponse>, Status> {
        let _message: ListReviewItemsMessage = request.into_inner();
        let items: Vec<_> = ReviewItemEntity::find()
            .all(&self.db)
            .await
            .map_err(db_err_to_status)?;

        // convert from dbreviewitems (models) into the rRPC version
        let items = items.into_iter().map(From::from).collect();

        let response = ListReviewItemsResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
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

        let item = ReviewItemEntity::find_by_id(&message.name)
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
                message: None,
            }),
            item,
        }));
    }

    async fn create_review_item(
        &self,
        request: Request<CreateReviewItemMessage>,
    ) -> Result<Response<CreateReviewItemResponse>, Status> {
        let message: CreateReviewItemMessage = request.into_inner();

        let grpc::NewReviewItem { item_type, data } = message.item.ok_or(Status::new(
            Code::InvalidArgument,
            "new review item not included in payload",
        ))?;

        let review_item = ReviewItemActiveModel {
            item_type: Set(item_type),
            data: Set(data),
            ..Default::default()
        };

        let res = review_item
            .insert(&self.db)
            .await
            .map_err(db_err_to_status)?;

        let review_item: grpc::ReviewItem = res.into();

        let response = CreateReviewItemResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
            }),
            item: Some(review_item),
        };

        Ok(Response::new(response))
    }

    async fn update_review_item(
        &self,
        request: Request<UpdateReviewItemMessage>,
    ) -> Result<Response<UpdateReviewItemResponse>, Status> {
        let message: UpdateReviewItemMessage = request.into_inner();

        if message.item.is_none() {
            return Err(Status::new(
                Code::InvalidArgument,
                "payload had no 'item' field!",
            ));
        }

        let grpc::UpdateReviewItem {
            name,
            status,
            difficulty,
            stability,
            next_review_date,
            data,
        } = message.item.unwrap();

        let review_item: Option<ReviewItemModel> = ReviewItemEntity::find_by_id(name.clone())
            .one(&self.db)
            .await
            .map_err(db_err_to_status)?;

        let review_item = review_item.unwrap();

        let mut modifiable_review_item: ReviewItemActiveModel = review_item.into();

        // update the fields
        modifiable_review_item.update_time = ActiveValue::Set(chrono::Utc::now().to_rfc3339());
        modifiable_review_item.status = status.into_active_value();
        modifiable_review_item.difficulty = difficulty.into_active_value();
        modifiable_review_item.stability = stability.into_active_value();
        modifiable_review_item.next_review_date = next_review_date.into_active_value();
        modifiable_review_item.data = data.into_active_value();

        let review_item = modifiable_review_item
            .update(&self.db)
            .await
            .map_err(db_err_to_status)?;

        let review_item: grpc::ReviewItem = review_item.into();

        let response = UpdateReviewItemResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: Some(format!("item with name: {name} updated!")),
            }),
            item: Some(review_item),
        };

        Ok(Response::new(response))
    }

    async fn delete_review_item(
        &self,
        request: Request<DeleteReviewItemMessage>,
    ) -> Result<Response<DeleteReviewItemResponse>, Status> {
        let message = request.into_inner();
        let name = message.name;

        ReviewItemEntity::delete_by_id(name.clone())
            .exec(&self.db)
            .await
            .map_err(db_err_to_status)?;

        let response = DeleteReviewItemResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: Some(format!("item with name: {name} deleted!")),
            }),
        };

        Ok(Response::new(response))
    }
}
