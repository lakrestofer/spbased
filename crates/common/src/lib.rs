use std::fmt::Display;

use directories::ProjectDirs;

pub fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("", "", "spbased")
}

pub enum ReviewItemStatus {
    Inbox,
    Review,
    Burried,
}

pub struct ConversionError;

impl Display for ReviewItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewItemStatus::Inbox => write!(f, "inbox"),
            ReviewItemStatus::Review => write!(f, "review"),
            ReviewItemStatus::Burried => write!(f, "burried"),
        }
    }
}

impl TryFrom<&str> for ReviewItemStatus {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "inbox" => Ok(ReviewItemStatus::Inbox),
            "review" => Ok(ReviewItemStatus::Review),
            "burried" => Ok(ReviewItemStatus::Burried),
            _ => Err(ConversionError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_from_i32() {
        let works_nums = ["inbox", "review", "burried"];
        for i in works_nums {
            assert!(ReviewItemStatus::try_from(i).is_ok());
        }
        let not_works_nums = ["42", "52", "not_status", "boofed"];
        for i in not_works_nums {
            assert!(ReviewItemStatus::try_from(i).is_err());
        }
    }
}
