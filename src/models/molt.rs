use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MoltRecord {
    pub tarantula_name: String,
    pub molt_date: String,
    pub stage: String,
    pub pre_molt_length_cm: Option<f32>,
    pub complications: Option<String>,
    pub notes: Option<String>,
}
