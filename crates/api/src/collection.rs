//! Collection service. Provides the basic review item manipulation api endpoints

// external imports
use sea_orm::entity::prelude::*;
use sea_orm::{EntityTrait, Paginator, QueryOrder, SelectModel, Set};
use tonic::{Code, Request, Response, Status};
// internal imports
// workspace imports
use entity::review_item;
pub use grpc::collection_server::{Collection, CollectionServer};
pub use grpc::ResponseStatus;
pub use grpc::{
    CreateReviewItemMessage, CreateReviewItemResponse, DeleteReviewItemMessage,
    DeleteReviewItemResponse, GetReviewItemMessage, GetReviewItemResponse, ListReviewItemsMessage,
    ListReviewItemsResponse, UpdateReviewItemMessage, UpdateReviewItemResponse,
};

use crate::{db_err_to_status, version, OptionToActiveValue, DEFAULT_PAGE_SIZE};

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
        // [X] return review items
        // [ ] check that api version is valid
        // [X] paginate response
        // [ ] sorting based on fieldnames in order_by field
        // - [ ] specify sort direction using order_dir field
        // [ ] filtering based on some rules
        // - [ ] parse 'filter' field into structure that in turn can be used by sea orm
        let ListReviewItemsMessage {
            version: request_expected_version,
            page,
            page_size,
            order_by,
            order_dir,
            filter,
        } = request.into_inner();

        // there were no existing paginator, create a new one

        let mut search_query: Select<review_item::Entity> = review_item::Entity::find();

        // if there exists a next_page_token, we can use it to construct
        // a search query such that we get the next query

        if let Some(filter) = filter {}

        if let Some(order_by) = order_by {
            // if we are provided a order_by fields, we try to construct an order by
            let dir = match order_dir {
                Some(order_dir) => match order_dir.as_str() {
                    "asc" => sea_orm::Order::Asc,
                    "desc" => sea_orm::Order::Desc,
                    _ => sea_orm::Order::Desc,
                },
                None => sea_orm::Order::Desc,
            };

            let order_by_col = match order_by.as_str() {
                "name" => review_item::Column::Name,
                "create_time" => review_item::Column::CreateTime,
                "update_time" => review_item::Column::UpdateTime,
                "status" => review_item::Column::Status,
                "difficulty" => review_item::Column::Difficulty,
                "stability" => review_item::Column::Stability,
                "last_review_date" => review_item::Column::LastReviewDate,
                "next_review_date" => review_item::Column::NextReviewDate,
                "item_type" => review_item::Column::ItemType,
                _ => return Err(Status::invalid_argument("tried to order on faulty field")),
            };

            search_query = search_query.order_by(order_by_col, dir);
        }

        let page_size = if page_size == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            page_size
        };

        let paginator: Paginator<DatabaseConnection, SelectModel<review_item::Model>> =
            search_query.paginate(&self.db, page_size as u64);

        let total_items = paginator.num_items().await.map_err(db_err_to_status)? as i32;

        let items: Vec<_> = paginator
            .fetch_page(page as u64)
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
            total_items,
            page_size,
            items,
        };

        Ok(Response::new(response))
    }

    async fn get_review_item(
        &self,
        request: Request<GetReviewItemMessage>,
    ) -> Result<Response<GetReviewItemResponse>, Status> {
        // [X] retrieve specific review item given the name of said item
        let message: GetReviewItemMessage = request.into_inner();

        let item = review_item::Entity::find_by_id(&message.name)
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
        // [X] create a new review item given the data field and item type
        // should I store a table of item types? probably
        // [ ]
        let message: CreateReviewItemMessage = request.into_inner();

        let grpc::NewReviewItem { item_type, data } = message.item.ok_or(Status::new(
            Code::InvalidArgument,
            "new review item not included in payload",
        ))?;

        let review_item = review_item::ActiveModel {
            item_type: Set(item_type),
            data: Set(data),
            ..Default::default()
        };

        // the initialization of all other fields is done in the 'before_save' hook in the entity definition
        // NOTE: Regeneration of the entities using the "generate_entities" script will result in the
        // 'before_save' hook being overwritten, resulting in the correct fields not being set.
        // There should be a test that detects this.
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

        let grpc::UpdateReviewItem { name, status, data } = message.item.unwrap();

        let review_item: Option<review_item::Model> = review_item::Entity::find_by_id(name.clone())
            .one(&self.db)
            .await
            .map_err(db_err_to_status)?;

        let review_item = review_item.unwrap();

        let mut modifiable_review_item: review_item::ActiveModel = review_item.into();

        // update the fields
        modifiable_review_item.status = status.into_active_value();
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

        review_item::Entity::delete_by_id(name.clone())
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
