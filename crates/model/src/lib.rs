use rusqlite;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::ToSql;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonData(pub serde_json::Value);

impl FromSql for JsonData {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        match serde_json::from_str(s) {
            Ok(v) => Ok(JsonData(v)),
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for JsonData {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.0.to_string().into())
    }
}

pub type ItemModel = String;
pub type ItemData = JsonData;
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
            "new" => Ok(Maturity::New),
            "young" => Ok(Maturity::Young),
            "tenured" => Ok(Maturity::Tenured),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Maturity {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(match self {
            Maturity::New => "new".into(),
            Maturity::Young => "young".into(),
            Maturity::Tenured => "tenured".into(),
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

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: TagName,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
