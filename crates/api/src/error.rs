use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Conversion error")]
    ConversionError,
    #[error("Other: {0}")]
    Other(String),
}
