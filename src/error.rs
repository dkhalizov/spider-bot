use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Telegram error: {0}")]
    Telegram(#[from] teloxide::RequestError),

    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    ValidationError(String),

    #[error("Operation failed: {0}")]
    OperationError(String),
}
