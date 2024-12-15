use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramUser {
    pub telegram_id: u64,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
}