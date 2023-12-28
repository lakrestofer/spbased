use crate::error::{self, ApiError};

pub enum ReviewItemStatus {
    Inbox = 0,
    Review = 1,
    Burried = 2,
}

impl ReviewItemStatus {
    pub fn as_i32(&self) -> i32 {
        match self {
            ReviewItemStatus::Inbox => ReviewItemStatus::Inbox as i32,
            ReviewItemStatus::Review => ReviewItemStatus::Review as i32,
            ReviewItemStatus::Burried => ReviewItemStatus::Burried as i32,
        }
    }
}

impl TryFrom<i32> for ReviewItemStatus {
    type Error = error::ApiError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReviewItemStatus::Inbox),
            1 => Ok(ReviewItemStatus::Review),
            2 => Ok(ReviewItemStatus::Burried),
            _ => Err(ApiError::ConversionError),
        }
    }
}
