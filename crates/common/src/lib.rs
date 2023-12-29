pub enum ReviewItemStatus {
    Inbox = 0,
    Review = 1,
    Burried = 2,
}

pub struct ConversionError;

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
    type Error = ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReviewItemStatus::Inbox),
            1 => Ok(ReviewItemStatus::Review),
            2 => Ok(ReviewItemStatus::Burried),
            _ => Err(ConversionError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_to_i32() {
        assert_eq!(ReviewItemStatus::Inbox.as_i32(), 0);
        assert_eq!(ReviewItemStatus::Review.as_i32(), 1);
        assert_eq!(ReviewItemStatus::Burried.as_i32(), 2);
    }

    #[test]
    fn parse_from_i32() {
        let works_nums = [0, 1, 2];
        for i in works_nums {
            assert!(ReviewItemStatus::try_from(i).is_ok());
        }
        let not_works_nums = [3, 52, 13, 420];
        for i in not_works_nums {
            assert!(ReviewItemStatus::try_from(i).is_err());
        }
    }
}
