//! this module is responsible from converting db entities to and from gRPC entities

// internal imports
use super::review_item::Model as DbReviewItem;
// external imports
use grpc::ReviewItem as gReviewItem;

impl From<DbReviewItem> for gReviewItem {
    fn from(value: DbReviewItem) -> Self {
        let DbReviewItem {
            name,
            create_time,
            update_time,
            status,
            difficulty,
            stability,
            next_review_date,
            item_type,
            url,
            data,
        } = value;
        gReviewItem {
            name,
            create_time,
            update_time,
            status,
            difficulty,
            stability,
            next_review_date,
            item_type,
            url,
            data,
        }
    }
}

impl From<gReviewItem> for DbReviewItem {
    fn from(value: gReviewItem) -> Self {
        let gReviewItem {
            name,
            create_time,
            update_time,
            status,
            difficulty,
            stability,
            next_review_date,
            item_type,
            url,
            data,
        } = value;
        DbReviewItem {
            name,
            create_time,
            update_time,
            status,
            difficulty,
            stability,
            next_review_date,
            item_type,
            url,
            data,
        }
    }
}
