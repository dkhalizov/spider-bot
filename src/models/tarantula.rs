mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarantula {
    pub id: Option<i64>,
    pub name: String,
    pub species_id: i64,
    pub acquisition_date: DbDateTime,
    pub last_molt_date: Option<DbDateTime>,
    pub estimated_age_months: Option<i32>,
    pub current_molt_stage_id: Option<i64>,
    pub current_health_status_id: Option<i64>,
    pub last_health_check_date: Option<DbDateTime>,
    pub enclosure_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TarantulaListItem {
    pub id: i64,
    pub name: String,
    pub species_name: String,
    pub enclosure_number: Option<String>,
    pub days_since_feeding: Option<f64>,
    pub current_status: String,
}

#[derive(Debug, Serialize)]
pub struct MaintenanceTask {
    pub id: i64,
    pub name: String,
    pub enclosure_number: String,
    pub scientific_name: String,
    pub required_action: String,
    pub priority: i32,
}
