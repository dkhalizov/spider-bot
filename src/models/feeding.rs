use crate::models::models::DbDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedingEvent {
    pub id: Option<i64>,
    pub tarantula_id: i64,
    pub feeding_date: DbDateTime,
    pub cricket_colony_id: i64,
    pub number_of_crickets: i32,
    pub feeding_status_id: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeedingRecord {
    pub tarantula_name: String,
    pub feeding_date: String,
    pub colony_name: String,
    pub number_of_crickets: i32,
    pub status: String,
    pub notes: Option<String>,
}
