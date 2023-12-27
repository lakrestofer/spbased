//! Scheduler service. Provides more directed enpoints for review events.

// external imports
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Set};
use tonic::{Code, Request, Response, Status};
// stdlib imports
use std::cell::OnceCell;
// internal imports
// workspace imports
use entity::review_item::ActiveModel as ReviewItemActiveModel;
use entity::review_item::Entity as ReviewItemEntity;
use entity::review_item::Model as ReviewItemModel;

// scheduler
use grpc::scheduler_server::{Scheduler, SchedulerServer};

#[derive(Debug)]
pub struct CollectionService {
    db: DatabaseConnection,
}

pub use grpc::{ResponseStatus, VersionInfo};
