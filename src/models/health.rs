#[derive(Debug, Serialize)]
pub struct HealthAlert {
    pub id: i64,
    pub name: String,
    pub scientific_name: String,
    pub alert_type: String,
    pub days_in_state: i32,
}
#[derive(Debug, Serialize)]
struct HealthRecord {
    tarantula_name: String,
    check_date: String,
    status: String,
    weight_grams: Option<f32>,
    humidity_percent: Option<i32>,
    temperature_celsius: Option<f32>,
    notes: Option<String>,
}