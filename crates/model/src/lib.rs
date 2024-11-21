use rusqlite;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::ToSql;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

pub type ItemModel = String;
pub type ItemData = String;
pub type TagName = String;

/// A measure of how well we've 'learnt' an item.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Maturity {
    /// This item has not yet been reviewed
    #[default]
    New,
    /// This item has been reviewed but has a stability less than 100 days.
    Young,
    /// This items has been reviewed many times and can probably be considered fully 'learnt'
    Tenured,
}
impl FromSql for Maturity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        match s {
            "New" => Ok(Maturity::New),
            "Young" => Ok(Maturity::Young),
            "Tenured" => Ok(Maturity::Tenured),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Maturity {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(match self {
            Maturity::New => "New".into(),
            Maturity::Young => "Young".into(),
            Maturity::Tenured => "Tenured".into(),
        })
    }
}

impl std::fmt::Display for Maturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Maturity::New => write!(f, "New"),
            Maturity::Young => write!(f, "Young"),
            Maturity::Tenured => write!(f, "Tenured"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub maturity: Maturity,
    pub stability: sra::model::Stability,
    pub difficulty: sra::model::Difficulty,
    #[serde(with = "time::serde::rfc3339")]
    pub last_review_date: OffsetDateTime,
    pub n_reviews: i32,
    pub n_lapses: i32,
    pub model: ItemModel,
    pub data: ItemData,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: TagName,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
