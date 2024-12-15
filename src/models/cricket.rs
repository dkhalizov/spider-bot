#[derive(Debug, Serialize, Deserialize)]
pub struct CricketColony {
    pub id: Option<i64>,
    pub colony_name: String,
    pub size_type_id: i64,
    pub current_count: i32,
    pub last_count_date: DbDateTime,
    pub container_number: String,
    pub notes: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct ColonyStatus {
    pub id: i64,
    pub colony_name: String,
    pub current_count: i32,
    pub size_name: String,
    pub crickets_used_7_days: i32,
    pub weeks_remaining: Option<f64>,
}


#[derive(Debug, Serialize)]
struct ColonyMaintenanceRecord {
    colony_name: String,
    maintenance_date: String,
    previous_count: i32,
    new_count: i32,
    food_added: bool,
    water_added: bool,
    cleaning_performed: bool,
    notes: Option<String>,
}
