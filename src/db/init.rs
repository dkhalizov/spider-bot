use crate::models::enums::{CricketSize, FeedingStatus, HealthStatus, MoltStage};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use crate::BotResult;

pub fn fill_default_enums(pool: Pool<SqliteConnectionManager>) -> BotResult<()> {
    init_health_statuses(pool.clone())?;
    init_feeding_statuses(pool.clone())?;
    init_molt_stages(pool.clone())?;
    init_cricket_sizes(pool)?;
    Ok(())
}

fn init_health_statuses(conn: Pool<SqliteConnectionManager>) -> BotResult<()> {
    let statuses = [
        HealthStatus::Healthy,
        HealthStatus::Monitor,
        HealthStatus::Critical,
    ];

    for status in statuses.iter() {
        let conn = conn.get()?;
        conn.execute(
            "INSERT OR IGNORE INTO health_statuses (id, status_name, description)
                 VALUES (?, ?, ?)",
            params![*status as i32, status.to_db_name(), status.description()],
        )?;
    }
    Ok(())
}

fn init_feeding_statuses(conn: Pool<SqliteConnectionManager>) -> BotResult<()> {
    let statuses = [
        FeedingStatus::Accepted,
        FeedingStatus::Rejected,
        FeedingStatus::Partial,
        FeedingStatus::PreMolt,
        FeedingStatus::Dead,
        FeedingStatus::Overflow,
    ];

    for status in statuses.iter() {
        conn.get()?.execute(
            "INSERT OR IGNORE INTO feeding_statuses (id, status_name, description)
                 VALUES (?, ?, ?)",
            params![*status as i32, status.to_db_name(), status.description()],
        )?;
    }
    Ok(())
}

fn init_molt_stages(conn: Pool<SqliteConnectionManager>) -> BotResult<()> {
    let stages = [
        MoltStage::Normal,
        MoltStage::PreMolt,
        MoltStage::Molting,
        MoltStage::PostMolt,
        MoltStage::Failed,
    ];

    for stage in stages.iter() {
        conn.get()?.execute(
            "INSERT OR IGNORE INTO molt_stages (id, stage_name, description)
                 VALUES (?, ?, ?)",
            params![*stage as i32, stage.to_db_name(), stage.description()],
        )?;
    }
    Ok(())
}

fn init_cricket_sizes(conn: Pool<SqliteConnectionManager>) -> BotResult<()> {
    let sizes = [
        CricketSize::Pinhead,
        CricketSize::Small,
        CricketSize::Medium,
        CricketSize::Large,
        CricketSize::Adult,
    ];

    for size in sizes.iter() {
        conn.get()?.execute(
            "INSERT OR IGNORE INTO cricket_size_types (id, size_name, approximate_length_mm)
                 VALUES (?, ?, ?)",
            params![*size as i32, size.to_db_name(), size.length_mm()],
        )?;
    }
    Ok(())
}
