use crate::models::models::DbDateTime;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enclosure {
    pub id: Option<i64>,
    pub name: String,
    pub height_cm: i32,
    pub width_cm: i32,
    pub length_cm: i32,
    pub substrate_depth_cm: i32,
    pub notes: Option<String>,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedingSchedule {
    pub species_id: i64,
    pub size_category: String,
    pub body_length_cm: f32,
    pub prey_size: String,
    pub feeding_frequency: String,
    pub prey_type: String,
    pub notes: Option<String>,
    pub frequency_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedingFrequency {
    pub id: i64,
    pub frequency_name: String,
    pub min_days: i32,
    pub max_days: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceRecord {
    pub id: Option<i64>,
    pub enclosure_id: i64,
    pub maintenance_date: NaiveDate,
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<i32>,
    pub notes: Option<String>,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarantula {
    pub id: i64,
    pub name: String,
    pub species_id: i64,
    pub acquisition_date: NaiveDate,
    pub last_molt_date: Option<NaiveDate>,
    pub estimated_age_months: Option<i32>,
    pub current_molt_stage_id: Option<i64>,
    pub current_health_status_id: Option<i64>,
    pub last_health_check_date: Option<NaiveDate>,
    pub enclosure_number: Option<String>,
    pub notes: Option<String>,
    pub user_id: i64,
    pub enclosure_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CricketColony {
    pub id: Option<i64>,
    pub colony_name: String,
    pub size_type_id: i64,
    pub current_count: i32,
    pub last_count_date: NaiveDate,
    pub container_number: String,
    pub notes: Option<String>,
    pub user_id: i64, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedingEvent {
    pub id: Option<i64>,
    pub tarantula_id: i64,
    pub feeding_date: DbDateTime,
    pub cricket_colony_id: i64,
    pub number_of_crickets: i32,
    pub feeding_status_id: i64,
    pub notes: Option<String>,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckRecord {
    pub id: Option<i64>,
    pub tarantula_id: i64,
    pub check_date: NaiveDate,
    pub health_status_id: i64,
    pub weight_grams: Option<f32>,
    pub humidity_percent: Option<i32>,
    pub temperature_celsius: Option<f32>,
    pub notes: Option<String>,
    pub user_id: i64,
}
