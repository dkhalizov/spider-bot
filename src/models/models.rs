use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbDateTime(DateTime<Utc>);

impl Default for DbDateTime {
    fn default() -> Self {
        DbDateTime(Utc::now())
    }
}

impl rusqlite::types::FromSql for DbDateTime {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            parse_db_datetime(s.to_string())
                .map(DbDateTime)
                .map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
        })
    }
}

impl rusqlite::types::ToSql for DbDateTime {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(datetime_to_string(&self.0).into())
    }
}

fn parse_db_datetime(s: String) -> rusqlite::Result<DateTime<Utc>> {
    let naive = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
}

fn datetime_to_string(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
