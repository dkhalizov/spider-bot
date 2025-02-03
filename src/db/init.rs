use crate::models::enums::{CricketSize, FeedingStatus, HealthStatus, MoltStage};
use crate::BotResult;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

pub fn fill_default_enums(pool: Pool<SqliteConnectionManager>) -> BotResult<()> {
    init_health_statuses(pool.clone())?;
    init_feeding_statuses(pool.clone())?;
    init_molt_stages(pool.clone())?;
    init_cricket_sizes(pool.clone())?;
    init_feeding_frequencies(pool)?;
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

fn init_feeding_frequencies(conn: Pool<SqliteConnectionManager>) -> BotResult<()> {
    let frequencies = [
        (
            "3-4 times per week",
            2,
            3,
            "Very frequent feeding for spiderlings",
        ),
        (
            "2-3 times per week",
            3,
            4,
            "Frequent feeding for spiderlings",
        ),
        ("Every 4-5 days", 4, 5, "Regular feeding for juveniles"),
        ("Every 5-7 days", 5, 7, "Standard juvenile feeding"),
        ("Every 7 days", 7, 7, "Weekly feeding"),
        ("Every 7-10 days", 7, 10, "Extended weekly feeding"),
        ("Every 10-14 days", 10, 14, "Bi-weekly feeding"),
        ("Every 14 days", 14, 14, "Strict bi-weekly feeding"),
        ("Every 14-21 days", 14, 21, "Extended bi-weekly feeding"),
        ("Every 21-28 days", 21, 28, "Monthly feeding"),
        ("Every 21-30 days", 21, 30, "Extended monthly feeding"),
    ];

    for (name, min, max, desc) in frequencies.iter() {
        conn.get()?.execute(
            "INSERT OR IGNORE INTO feeding_frequencies (frequency_name, min_days, max_days, description)
             VALUES (?, ?, ?, ?)",
            params![name, min, max, desc],
        )?;
    }
    Ok(())
}
