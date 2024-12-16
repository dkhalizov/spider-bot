use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::enums::CricketSize;

#[derive(Debug, Serialize, Deserialize)]
pub struct CricketColony {
    pub id: Option<i64>,
    pub colony_name: String,
    pub size_type_id: i64,
    pub current_count: i32,
    pub last_count_date: NaiveDate,
    pub container_number: String,
    pub notes: Option<String>,
}
#[derive(Debug, Serialize, Clone)]
pub struct ColonyStatus {
    pub id: i64,
    pub colony_name: String,
    pub current_count: i32,
    pub size_type: CricketSize,
    pub crickets_used_7_days: i32,
    pub weeks_remaining: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ColonyMaintenanceRecord {
    pub colony_name: String,
    pub maintenance_date: String,
    pub previous_count: i32,
    pub new_count: i32,
    pub food_added: bool,
    pub water_added: bool,
    pub cleaning_performed: bool,
    pub notes: Option<String>,
}
