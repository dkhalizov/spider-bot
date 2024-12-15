use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthAlert {
    pub id: i64,
    pub name: String,
    pub scientific_name: String,
    pub alert_type: String,
    pub days_in_state: i32,
}
#[derive(Debug, Serialize)]
pub struct HealthRecord {
    pub tarantula_name: String,
    pub check_date: String,
    pub status: String,
    pub weight_grams: Option<f32>,
    pub humidity_percent: Option<i32>,
    pub temperature_celsius: Option<f32>,
    pub notes: Option<String>,
}
