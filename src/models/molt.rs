

#[derive(Debug, Serialize)]
struct MoltRecord {
    tarantula_name: String,
    molt_date: String,
    stage: String,
    pre_molt_length_cm: Option<f32>,
    complications: Option<String>,
    notes: Option<String>,
}