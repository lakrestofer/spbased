//! Scheduler service. Provides more directed enpoints for review events.

use grpc::{
    GradeReviewItemMessage, GradeReviewItemResponse, ListDueReviewItemsMessage,
    ListDueReviewItemsResponse, ListNewReviewItemsMessage, ListNewReviewItemsResponse,
};
// external imports
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, Condition, DatabaseConnection, DbErr, EntityTrait, Set};
use tonic::{Code, Request, Response, Status};
// stdlib imports
use std::cell::OnceCell;
// internal imports
// workspace imports
use entity::review_item;

// scheduler
use grpc::scheduler_server::{Scheduler, SchedulerServer};

#[derive(Debug)]
pub struct CollectionService {
    db: DatabaseConnection,
}

pub use grpc::{ResponseStatus, VersionInfo};

use crate::{db_err_to_status, version};

pub use crate::types::ReviewItemStatus;

#[derive(Debug)]
pub struct SchedulerService {
    db: DatabaseConnection,
}

impl SchedulerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl Scheduler for SchedulerService {
    async fn list_due_review_items(
        &self,
        request: Request<ListDueReviewItemsMessage>,
    ) -> Result<Response<ListDueReviewItemsResponse>, Status> {
        // - required -
        // [X] Return the items that are due
        // [X] Filter on item type (if we are reviewing flashcards, we only want to return flashcards)
        // [X] Filter on item status (only items marked as 'in review')
        // [ ] Sort the items by urgency
        // - optional -
        // [ ] secondarily sort the items by priority
        let ListDueReviewItemsMessage { item_type, .. } = request.into_inner();

        let now = chrono::Utc::now();

        let is_due_cond = review_item::Column::NextReviewDate.lte(now); // find due items
        let type_cond = review_item::Column::ItemType.eq(item_type); // only choose items of the requested type
        let status_cond = review_item::Column::ItemType.eq(ReviewItemStatus::Review.as_i32()); // only choose items that "in review"
        let filter_cond = Condition::all()
            .add(is_due_cond)
            .add(type_cond)
            .add(status_cond);

        // since the type fields is on the review item and not the scheduled event
        // we have to retrieve all items
        let select_query = review_item::Entity::find().filter(filter_cond);

        let items = select_query.all(&self.db).await.map_err(db_err_to_status)?;
        let items = items.into_iter().map(|item| item.into()).collect();

        let response = Response::new(ListDueReviewItemsResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
            }),
            next_page_token: "".into(),
            items,
        });

        Ok(response)
    }
    async fn list_new_review_items(
        &self,
        request: Request<ListNewReviewItemsMessage>,
    ) -> Result<Response<ListNewReviewItemsResponse>, Status> {
        // - required -
        // [ ] Return items that are new
        todo!();
    }
    async fn grade_review_item(
        &self,
        request: Request<GradeReviewItemMessage>,
    ) -> Result<Response<GradeReviewItemResponse>, Status> {
        todo!();
    }
}
