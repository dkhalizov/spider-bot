use rusqlite::{params, Connection, Result};
use crate::models::enums::{CricketSize, FeedingStatus, HealthStatus, MoltStage};

pub struct DbInitializer {
    conn: Connection,
}

impl DbInitializer {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn into_inner(self) -> Connection {
        self.conn
    }

    pub fn initialize(&self) -> Result<()> {
        self.init_health_statuses()?;
        self.init_feeding_statuses()?;
        self.init_molt_stages()?;
        self.init_cricket_sizes()?;
        Ok(())
    }

    fn init_health_statuses(&self) -> Result<()> {
        let statuses = [
            HealthStatus::Healthy,
            HealthStatus::Monitor,
            HealthStatus::Critical,
        ];

        for status in statuses.iter() {
            self.conn.execute(
                "INSERT OR IGNORE INTO health_statuses (id, status_name, description)
                 VALUES (?, ?, ?)",
                params![*status as i32, status.to_db_name(), status.description()],
            )?;
        }
        Ok(())
    }

    fn init_feeding_statuses(&self) -> Result<()> {
        let statuses = [
            FeedingStatus::Accepted,
            FeedingStatus::Rejected,
            FeedingStatus::Partial,
            FeedingStatus::PreMolt,
            FeedingStatus::Dead,
            FeedingStatus::Overflow,
        ];

        for status in statuses.iter() {
            self.conn.execute(
                "INSERT OR IGNORE INTO feeding_statuses (id, status_name, description)
                 VALUES (?, ?, ?)",
                params![*status as i32, status.to_db_name(), status.description()],
            )?;
        }
        Ok(())
    }

    fn init_molt_stages(&self) -> Result<()> {
        let stages = [
            MoltStage::Normal,
            MoltStage::PreMolt,
            MoltStage::Molting,
            MoltStage::PostMolt,
            MoltStage::Failed,
        ];

        for stage in stages.iter() {
            self.conn.execute(
                "INSERT OR IGNORE INTO molt_stages (id, stage_name, description)
                 VALUES (?, ?, ?)",
                params![*stage as i32, stage.to_db_name(), stage.description()],
            )?;
        }
        Ok(())
    }

    fn init_cricket_sizes(&self) -> Result<()> {
        let sizes = [
            CricketSize::Pinhead,
            CricketSize::Small,
            CricketSize::Medium,
            CricketSize::Large,
            CricketSize::Adult,
        ];

        for size in sizes.iter() {
            self.conn.execute(
                "INSERT OR IGNORE INTO cricket_size_types (id, size_name, approximate_length_mm)
                 VALUES (?, ?, ?)",
                params![*size as i32, size.to_db_name(), size.length_mm()],
            )?;
        }
        Ok(())
    }
}