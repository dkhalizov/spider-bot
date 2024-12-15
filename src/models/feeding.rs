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
struct FeedingRecord {
    tarantula_name: String,
    feeding_date: String,
    colony_name: String,
    number_of_crickets: i32,
    status: String,
    notes: Option<String>,
}