//! Scheduler service. Provides more directed enpoints for review events.

use chrono::{Days, FixedOffset};
use grpc::{
    GradeReviewItemMessage, GradeReviewItemResponse, ListDueReviewItemsMessage,
    ListDueReviewItemsResponse, ListNewReviewItemsMessage, ListNewReviewItemsResponse, ReviewItem,
};
// external imports
use sea_orm::entity::prelude::*;
use sea_orm::{Condition, DatabaseConnection, EntityTrait, Set};
use tonic::{Code, Request, Response, Status};
// stdlib imports
// internal imports
// workspace imports
use entity::review_item;

// scheduler
use grpc::scheduler_server::Scheduler;

// spaced repetition algorithm
use algo;

pub use grpc::{ResponseStatus, VersionInfo};

use crate::{db_err_to_status, version};

#[derive(Debug)]
pub struct SchedulerService {
    db: DatabaseConnection,
}

impl SchedulerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

use common::{self, ReviewItemStatus};

#[tonic::async_trait]
impl Scheduler for SchedulerService {
    async fn list_due_review_items(
        &self,
        request: Request<ListDueReviewItemsMessage>,
    ) -> Result<Response<ListDueReviewItemsResponse>, Status> {
        // - required -
        // [X] Return the items that are due
        // [X] (optionally) Filter on item type (if we are reviewing flashcards, we only want to return flashcards)
        // [ ] allow for optional filtering, we migtht want to return all due items
        // [X] Filter on item status (only items marked as 'in review')
        // [X] Sort the items by urgency
        // - optional -
        // [ ] secondarily sort the items by priority - NOTE: it does not make
        //     that much sense to secondarily sort on priority. One reason is that there will
        //     not exist many items that have the same urgency. It would be better if the
        //     priority and urgency.
        let ListDueReviewItemsMessage {
            item_type,
            version: _,
            page_size,
            page,
        } = request.into_inner();

        let now = chrono::Utc::now();

        // we will always apply these conditions
        let is_due_cond = review_item::Column::NextReviewDate.lte(now); // find due items
                                                                        // TODO apply this filtering conditionally
        let status_cond =
            review_item::Column::Status.eq(common::ReviewItemStatus::Review.to_string()); // only choose items that "in review"

        let mut filter_cond = Condition::all().add(is_due_cond).add(status_cond);

        // if we supply the type as well, we filter on it
        if let Some(item_type) = item_type {
            let type_cond = review_item::Column::ItemType.eq(item_type); // only choose items of the requested type
            filter_cond = filter_cond.add(type_cond);
        }

        // we have to retrieve the elements before we can sort them based based on urgency
        let select_query = review_item::Entity::find().filter(filter_cond);

        let paginator = select_query.paginate(&self.db, page_size as u64);
        let total_items = paginator.num_items().await.map_err(db_err_to_status)? as i32;

        let items = paginator
            .fetch_page(page as u64)
            .await
            .map_err(db_err_to_status)?;

        // extract out dates and filters out items with non rfc3339 new_review_date fields
        let mut items_with_dates: Vec<(review_item::Model, chrono::DateTime<FixedOffset>)> = items
            .into_iter()
            .map(|item| {
                let date = chrono::DateTime::parse_from_rfc3339(&item.next_review_date);
                (item, date)
            })
            .filter(|(_item, date_res)| date_res.is_ok())
            .map(|(item, date_res)| (item, date_res.unwrap()))
            .collect();

        // sort based on urgency. The difference in due date and review date divided by the stability gives a measure of how
        // important it is to review the item now.
        // between two items that are both three days overdue, the item that has a stability of 100 is less urgent
        // than the item that has a lower stability
        items_with_dates.sort_by(|(a_item, a_date), (b_item, b_date)| {
            // what is the difference in number of days between now and the scheduled review date
            let a_diff = now.signed_duration_since(a_date).num_days() as f32; // positive if current time is larger than due date (should be)
            let b_diff = now.signed_duration_since(b_date).num_days() as f32;

            // if the difference is negative (should never be, but we leave this
            // logic in in case we want to allow for review ahead of due date using this function)
            let a_diff_scaled = if a_diff.is_sign_positive() {
                a_diff / a_item.stability as f32
            } else {
                a_diff * a_item.stability as f32
            };
            let b_diff_scaled = if b_diff.is_sign_positive() {
                b_diff / b_item.stability as f32
            } else {
                b_diff * b_item.stability as f32
            };

            a_diff_scaled.total_cmp(&b_diff_scaled)
        });

        // the items will be sorted low to high urgency , we want the high urgency items to come first
        let items = items_with_dates.into_iter().rev();

        let items = items.map(|(item, _date)| item).map(From::from); // convert from db model into grpc model
        let items = items.collect();

        let response = Response::new(ListDueReviewItemsResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
            }),
            items,
            total_items,
            page_size,
            page,
        });

        Ok(response)
    }
    async fn list_new_review_items(
        &self,
        request: Request<ListNewReviewItemsMessage>,
    ) -> Result<Response<ListNewReviewItemsResponse>, Status> {
        // - required -
        // [X] Return items that are new
        // [ ] Limit the amount of new items returned somehow (pagination)
        // here it makes sense to sort by priority!
        // [ ] sort by priority
        let ListNewReviewItemsMessage {
            version: _version,
            item_type,
            page,
            page_size,
        } = request.into_inner();

        let is_new_cond =
            review_item::Column::Status.eq(common::ReviewItemStatus::Inbox.to_string()); // only choose items that are in the "inbox"
        let mut filter_cond = Condition::all().add(is_new_cond);

        if let Some(item_type) = item_type {
            let type_cond = review_item::Column::ItemType.eq(item_type); // only choose items of the requested type.
            filter_cond = filter_cond.add(type_cond);
        }

        let select_query = review_item::Entity::find().filter(filter_cond);

        let paginator = select_query.paginate(&self.db, page_size as u64);

        let total_items = paginator.num_items().await.map_err(db_err_to_status)? as i32;

        let items = paginator
            .fetch_page(page as u64)
            .await
            .map_err(db_err_to_status)?;

        // TODO sort by priority after the priority fields has been introduced
        // items.sort_by_cached_key(|item| item.priority);

        let items: Vec<ReviewItem> = items.into_iter().map(From::from).collect();

        let response = Response::new(ListNewReviewItemsResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
            }),
            items,
            total_items,
            page_size,
            page,
        });

        Ok(response)
    }
    async fn grade_review_item(
        &self,
        request: Request<GradeReviewItemMessage>,
    ) -> Result<Response<GradeReviewItemResponse>, Status> {
        let GradeReviewItemMessage { name, grade, .. } = request.into_inner();

        // first we check that the grade is a valid number (1,2,3 or 4)
        if grade < 1 || 4 < grade {
            return Err(Status::new(
                Code::InvalidArgument,
                "grade was not between 1 and 4!",
            ));
        }

        let item = review_item::Entity::find_by_id(name)
            .one(&self.db)
            .await
            .map_err(db_err_to_status)?;

        let item = item.ok_or(Status::new(
            Code::NotFound,
            "No item with the provided name found",
        ))?;

        let status = ReviewItemStatus::try_from(item.status.as_str()).map_err(|_err| {
            Status::new(
                Code::DataLoss,
                "Review item status field could not be parsed into valid status enum",
            )
        })?;

        let grade: algo::Grade = algo::Grade::try_from(grade).map_err(|_err| {
            Status::new(
                Code::DataLoss,
                "Review item grade fields could not be parsed into valid grade enum",
            )
        })?;

        let old_last_review_date = &item.last_review_date;
        let old_d = item.difficulty;
        let old_s = item.stability;

        let now = chrono::Utc::now();

        let (new_difficulty, new_stability,  new_next_review_date) = match status {
            ReviewItemStatus::Inbox => {
                let d = algo::difficulty_init(grade);
                let s = algo::stability_init(grade);
                let next_review_date = now.checked_add_days(Days::new(s as u64)).ok_or(Status::new(Code::Internal, "tried to add duration of days to datetime instance. Chrono did not like..."))?;
                (d, s, next_review_date)
            },
            ReviewItemStatus::Review => {
                let old_last_review_date = chrono::DateTime::parse_from_rfc3339(&old_last_review_date).map_err(|_| Status::new(Code::Internal, "could not parse date from 'last_review_date' field"))?;
                let diff = old_last_review_date.signed_duration_since(now).num_hours() as f64 / 24.0; // this will give us a fractional result

                let d = algo::update_difficulty(old_d, grade);
                let s = algo::update_stability(old_d, old_s, grade, diff);
                let next_review_date = now.checked_add_days(Days::new(s as u64)).ok_or(Status::new(Code::Internal, "tried to add duration of days to datetime instance. Chrono did not like..."))?;
                (d, s, next_review_date)
            },
            ReviewItemStatus::Burried => {
                return Err(Status::new(
                    Code::FailedPrecondition,
                    "Items was burried and cannot therefore be reviewed. Please unburry the item before review",
                ))
            } // this items should not be reviewed since it is burried. It needs to be unburried before it is valid to review it again
        };

        let mut item: review_item::ActiveModel = item.into();

        item.status = Set(ReviewItemStatus::Review.to_string());
        item.difficulty = Set(new_difficulty);
        item.stability = Set(new_stability);
        item.last_review_date = Set(now.to_rfc3339());
        item.next_review_date = Set(new_next_review_date.to_rfc3339());
        item.update_time = Set(now.to_rfc3339());

        item.update(&self.db).await.map_err(db_err_to_status)?;

        let response = Response::new(GradeReviewItemResponse {
            version: Some(version()),
            status: Some(ResponseStatus {
                code: 200,
                message: None,
            }),
        });

        Ok(response)
    }
}
