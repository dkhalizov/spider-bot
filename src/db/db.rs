use crate::db::init::fill_default_enums;
use crate::error::BotError;
use crate::models::cricket::ColonyStatus;
use crate::models::enums::{CricketSize, FeedingStatus, HealthStatus, MoltStage};
use crate::models::feeding::{FeedingEvent, FeedingRecord};
use crate::models::health::{HealthAlert, HealthRecord};
use crate::models::molt::MoltRecord;
use crate::models::tarantula::{MaintenanceTask, Tarantula, TarantulaListItem};
use crate::models::user::TelegramUser;
use crate::BotResult;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::Arc;

pub struct TarantulaDB {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl TarantulaDB {
    pub fn new(db_path: &str) -> BotResult<Self> {
        let flags =
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE;

        let manager = SqliteConnectionManager::file(db_path).with_flags(flags);
        let pool = Pool::new(manager)?;

        fill_default_enums(pool.clone())?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn add_tarantula(
        &self,
        user_id: u64,
        name: &str,
        species_id: i64,
        acquisition_date: &str,
        estimated_age_months: i64,
        enclosure_number: Option<&str>,
        notes: Option<&str>,
    ) -> BotResult<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO tarantulas (
            name, species_id, acquisition_date, estimated_age_months,
            enclosure_number, notes, user_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                name,
                species_id,
                acquisition_date,
                estimated_age_months,
                enclosure_number,
                notes,
                user_id,
            ],
        )?;
        Ok(())
    }

    pub async fn add_colony(
        &self,
        user_id: u64,
        colony_name: &str,
        size_type_id: i64,
        current_count: i32,
        container_number: &str,
        notes: Option<&str>,
    ) -> BotResult<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO cricket_colonies (
            colony_name, size_type_id, current_count, container_number, notes, user_id
        ) VALUES (?, ?, ?, ?, ?, ?)",
            params![
                colony_name,
                size_type_id,
                current_count,
                container_number,
                notes,
                user_id,
            ],
        )?;
        Ok(())
    }

    pub async fn ensure_user_exists(&self, user: &TelegramUser) -> BotResult<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO telegram_users (telegram_id, username, first_name, last_name)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(telegram_id) DO UPDATE SET
                username = ?2,
                first_name = ?3,
                last_name = ?4,
                last_active = CURRENT_TIMESTAMP",
            params![
                user.telegram_id,
                user.username,
                user.first_name,
                user.last_name,
            ],
        )?;
        Ok(())
    }

    pub async fn get_all_tarantulas(&self, user_id: u64) -> BotResult<Vec<TarantulaListItem>> {
        let sql = "
        SELECT
            t.id,
            t.name,
            ts.common_name as species_name,
            t.enclosure_number,
            julianday('now') - julianday(MAX(f.feeding_date)) as days_since_feeding,
            CASE
                WHEN ms.stage_name = ? THEN ?
                WHEN hs.status_name = ? THEN ?
                WHEN julianday('now') - julianday(MAX(f.feeding_date)) > 14 THEN 'Needs feeding'
                ELSE 'Normal'
            END as current_status
        FROM tarantulas t
        JOIN tarantula_species ts ON t.species_id = ts.id
        LEFT JOIN molt_stages ms ON t.current_molt_stage_id = ms.id
        LEFT JOIN health_statuses hs ON t.current_health_status_id = hs.id
        LEFT JOIN feeding_events f ON t.id = f.tarantula_id
        WHERE t.user_id = ?
        GROUP BY t.id
        ORDER BY t.name";

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        let items = stmt
            .query_map(
                params![
                    MoltStage::PreMolt.to_db_name(),
                    MoltStage::PreMolt.to_db_name(),
                    HealthStatus::Critical.to_db_name(),
                    HealthStatus::Critical.to_db_name(),
                    user_id
                ],
                |row| {
                    Ok(TarantulaListItem {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        species_name: row.get(2)?,
                        enclosure_number: row.get(3)?,
                        days_since_feeding: row.get(4)?,
                        current_status: row.get(5)?,
                    })
                },
            )
            .map_err(BotError::Database)?;

        items
            .collect::<rusqlite::Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }
    pub async fn get_tarantulas_due_feeding(
        &self,
        user_id: u64,
    ) -> BotResult<Vec<TarantulaListItem>> {
        let sql = format!(
            "
        SELECT
            t.id,
            t.name,
            ts.common_name as species_name,
            t.enclosure_number,
            julianday('now') - julianday(MAX(f.feeding_date)) as days_since_feeding,
            'Needs feeding' as current_status
        FROM tarantulas t
        JOIN tarantula_species ts ON t.species_id = ts.id
        LEFT JOIN feeding_events f ON t.id = f.tarantula_id
        LEFT JOIN molt_stages ms ON t.current_molt_stage_id = ms.id
        WHERE ms.stage_name != '{}' AND t.user_id = ?
        GROUP BY t.id
        HAVING days_since_feeding > 7
        ORDER BY days_since_feeding DESC",
            MoltStage::PreMolt.to_db_name()
        );

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(&sql)?;
        let items = stmt
            .query_map([user_id], |row| {
                Ok(TarantulaListItem {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    species_name: row.get(2)?,
                    enclosure_number: row.get(3)?,
                    days_since_feeding: row.get(4)?,
                    current_status: row.get(5)?,
                })
            })
            .map_err(BotError::Database)?;

        items
            .collect::<rusqlite::Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }
    pub async fn record_feeding(&self, user_id: u64, event: &FeedingEvent) -> BotResult<i64> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        let tarantula_exists = tx
            .query_row(
                "SELECT 1 FROM tarantulas WHERE id = ? AND user_id = ?",
                params![event.tarantula_id, user_id],
                |_| Ok(()),
            )
            .is_ok();

        if !tarantula_exists {
            return Err(BotError::NotFound(format!(
                "Tarantula with id {} not found or access denied",
                event.tarantula_id
            )));
        }
        let rows_affected = tx.execute(
            "UPDATE cricket_colonies
        SET current_count = current_count - ?
        WHERE id = ? AND user_id = ?
        AND current_count >= ?",
            params![
                event.number_of_crickets,
                event.cricket_colony_id,
                user_id,
                event.number_of_crickets
            ],
        )?;

        if rows_affected == 0 {
            return Err(BotError::NotFound(
                "Colony not found, access denied, or insufficient crickets".to_string(),
            ));
        }

        let result = tx.execute(
            "INSERT INTO feeding_events (
            tarantula_id, 
            feeding_date, 
            cricket_colony_id,
            number_of_crickets, 
            feeding_status_id, 
            notes, 
            user_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                event.tarantula_id,
                event.feeding_date,
                event.cricket_colony_id,
                event.number_of_crickets,
                FeedingStatus::Accepted as i64,
                event.notes,
                user_id,
            ],
        )?;

        if result == 0 {
            return Err(BotError::NotFound(format!(
                "Tarantula with id {} not found or access denied",
                event.tarantula_id
            )));
        }

        let id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(id)
    }

    pub async fn record_molt(
        &self,
        tarantula_id: i64,
        length_cm: Option<f32>,
        complications: Option<String>,
        notes: Option<String>,
        user_id: u64,
    ) -> BotResult<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        let post_molt_id = MoltStage::PostMolt as i64;

        // Update tarantula and verify ownership in one query
        let rows_affected = tx.execute(
            "UPDATE tarantulas SET 
            last_molt_date = date('now'),
            current_molt_stage_id = ?
        WHERE id = ? AND user_id = ?",
            params![post_molt_id, tarantula_id, user_id],
        )?;

        if rows_affected == 0 {
            return Err(BotError::NotFound(format!(
                "Tarantula with id {} not found or access denied",
                tarantula_id
            )));
        }

        // Insert molt record after verifying ownership
        tx.execute(
            "INSERT INTO molt_records (
            tarantula_id, molt_date, molt_stage_id,
            pre_molt_length_cm, complications, notes, user_id
        ) VALUES (?, datetime('now'), ?, ?, ?, ?, ?)",
            params![
                tarantula_id,
                post_molt_id,
                length_cm,
                complications,
                notes,
                user_id
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub async fn record_health_check(
        &self,
        user_id: u64,
        tarantula_id: i64,
        status: HealthStatus,
        notes: Option<String>,
    ) -> BotResult<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        let status_id = status as i64;

        // Update tarantula and verify ownership in one query
        let rows_affected = tx.execute(
            "UPDATE tarantulas SET 
            last_health_check_date = datetime('now'),
            current_health_status_id = ?
        WHERE id = ? AND user_id = ?",
            params![status_id, tarantula_id, user_id],
        )?;

        if rows_affected == 0 {
            return Err(BotError::NotFound(format!(
                "Tarantula with id {} not found or access denied",
                tarantula_id
            )));
        }

        // Insert health check record after verifying ownership
        tx.execute(
            "INSERT INTO health_check_records (
            tarantula_id, check_date, health_status_id,
            weight_grams, humidity_percent, temperature_celsius,
            notes, user_id
        ) VALUES (?, datetime('now'), ?, ?, ?, ?, ?, ?)",
            params![tarantula_id, status_id, 0, 55, 20, notes, user_id],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub async fn get_colony_status(&self, user_id: u64) -> BotResult<Vec<ColonyStatus>> {
        let sql = "SELECT
            cc.id,
            cc.colony_name,
            cc.current_count,
            cst.size_name,
            COALESCE(SUM(fe.number_of_crickets), 0) as crickets_used_7_days,
            CASE
                WHEN SUM(fe.number_of_crickets) > 0
                THEN CAST(cc.current_count AS FLOAT) / (SUM(fe.number_of_crickets) / 7.0)
                ELSE NULL
            END as weeks_remaining
        FROM cricket_colonies cc
        JOIN cricket_size_types cst ON cc.size_type_id = cst.id
        LEFT JOIN feeding_events fe ON cc.id = fe.cricket_colony_id
            AND fe.feeding_date >= datetime('now', '-7 days')
        WHERE cc.user_id = ?
        GROUP BY cc.id
        ORDER BY weeks_remaining ASC";

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        let colonies = stmt
            .query_map([user_id], |row| {
                let size_name: String = row.get(3)?;
                let size_type = match size_name.as_str() {
                    "Pinhead" => CricketSize::Pinhead,
                    "Small" => CricketSize::Small,
                    "Medium" => CricketSize::Medium,
                    "Large" => CricketSize::Large,
                    "Adult" => CricketSize::Adult,
                    _ => CricketSize::Unknown,
                };

                Ok(ColonyStatus {
                    id: row.get(0)?,
                    colony_name: row.get(1)?,
                    current_count: row.get(2)?,
                    size_type,
                    crickets_used_7_days: row.get(4)?,
                    weeks_remaining: row.get(5)?,
                })
            })
            .map_err(BotError::Database)?;

        colonies
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    pub async fn get_health_alerts(&self, user_id: u64) -> BotResult<Vec<HealthAlert>> {
        let sql = format!("SELECT
        t.id,
        t.name,
        ts.scientific_name,
        CASE
            WHEN julianday('now') - julianday(t.last_health_check_date) >= 30 THEN 'Overdue Health Check'
            WHEN julianday('now') - julianday(MAX(f.feeding_date)) >= 14
                AND ms.stage_name != '{}' THEN 'Extended Feeding Strike'
            WHEN ms.stage_name = '{}'
                AND julianday('now') - julianday(t.last_molt_date) >= 180 THEN 'Extended Pre-molt'
            ELSE 'None'
        END as alert_type,
        CAST(
            CASE
                WHEN julianday('now') - julianday(t.last_health_check_date) >= 30
                    THEN julianday('now') - julianday(t.last_health_check_date)
                WHEN julianday('now') - julianday(MAX(f.feeding_date)) >= 14
                    THEN julianday('now') - julianday(MAX(f.feeding_date))
                WHEN ms.stage_name = '{}'
                    THEN julianday('now') - julianday(t.last_molt_date)
                ELSE 0
            END as INTEGER
        ) as days_in_state
    FROM tarantulas t
    JOIN tarantula_species ts ON t.species_id = ts.id
    LEFT JOIN feeding_events f ON t.id = f.tarantula_id
    LEFT JOIN molt_stages ms ON t.current_molt_stage_id = ms.id
    WHERE t.user_id = ?
    GROUP BY t.id
    HAVING alert_type != 'None'
    ORDER BY days_in_state DESC",
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name()
        );

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(&sql)?;
        let alerts = stmt
            .query_map([user_id], |row| {
                Ok(HealthAlert {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    scientific_name: row.get(2)?,
                    alert_type: row.get(3)?,
                    days_in_state: row.get(4)?,
                })
            })
            .map_err(BotError::Database)?;

        alerts
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    pub async fn get_maintenance_tasks(&self, user_id: u64) -> BotResult<Vec<MaintenanceTask>> {
        let sql = format!("SELECT
        t.id,
        t.name,
        COALESCE(t.enclosure_number, 'No enclosure') as enclosure_number,
        ts.scientific_name,
        CASE
            WHEN julianday('now') - julianday(t.last_health_check_date) >= 30 THEN 'Health Check Required'
            WHEN julianday('now') - julianday(MAX(f.feeding_date)) >= 7
                AND ms.stage_name != '{}' THEN 'Feeding Due'
            WHEN ms.stage_name = '{}' THEN 'Monitor for Molt'
            ELSE 'Regular Check'
        END as required_action,
        CASE
            WHEN julianday('now') - julianday(t.last_health_check_date) >= 30 THEN 1
            WHEN julianday('now') - julianday(MAX(f.feeding_date)) >= 7 THEN 2
            WHEN ms.stage_name = '{}' THEN 3
            ELSE 4
        END as priority
    FROM tarantulas t
    JOIN tarantula_species ts ON t.species_id = ts.id
    LEFT JOIN feeding_events f ON t.id = f.tarantula_id
    LEFT JOIN molt_stages ms ON t.current_molt_stage_id = ms.id
WHERE t.user_id = ?
    GROUP BY t.id
    HAVING required_action != 'Regular Check'
    ORDER BY priority, name",
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name()
        );

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(&sql)?;
        let tasks = stmt.query_map([user_id], |row| {
            Ok(MaintenanceTask {
                id: row.get(0)?,
                name: row.get(1)?,
                enclosure_number: row.get(2)?,
                scientific_name: row.get(3)?,
                required_action: row.get(4)?,
                priority: row.get(5)?,
            })
        })?;

        tasks
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    pub async fn get_tarantula_by_id(&self, user_id: u64, id: i64) -> BotResult<Tarantula> {
        let sql = "SELECT * FROM tarantulas WHERE id = ? AND user_id = ?";
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        stmt.query_row([id, user_id as i64], |row| {
            Ok(Tarantula {
                id: row.get(0)?,
                name: row.get(1)?,
                species_id: row.get(2)?,
                acquisition_date: row.get(3)?,
                last_molt_date: row.get(4)?,
                estimated_age_months: row.get(5)?,
                current_molt_stage_id: row.get(6)?,
                current_health_status_id: row.get(7)?,
                last_health_check_date: row.get(8)?,
                enclosure_number: row.get(9)?,
                notes: row.get(10)?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                BotError::NotFound(format!("Tarantula with id {} not found", id))
            }
            e => BotError::Database(e),
        })
    }

    pub(crate) async fn get_recent_feeding_records(
        &self,
        user_id: u64,
        limit: i32,
    ) -> BotResult<Vec<FeedingRecord>> {
        let sql = "
            SELECT 
                t.name as tarantula_name,
                fe.feeding_date,
                cc.colony_name,
                fe.number_of_crickets,
                fs.status_name as status,
                fe.notes
            FROM feeding_events fe
            JOIN tarantulas t ON fe.tarantula_id = t.id
            JOIN cricket_colonies cc ON fe.cricket_colony_id = cc.id
            JOIN feeding_statuses fs ON fe.feeding_status_id = fs.id
            WHERE t.user_id = ?
            ORDER BY fe.feeding_date DESC
            LIMIT ?";

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        let records = stmt.query_map([user_id, limit as u64], |row| {
            Ok(FeedingRecord {
                tarantula_name: row.get(0)?,
                feeding_date: row.get(1)?,
                colony_name: row.get(2)?,
                number_of_crickets: row.get(3)?,
                status: row.get(4)?,
                notes: row.get(5)?,
            })
        })?;

        records
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    pub(crate) async fn get_recent_health_records(
        &self,
        user_id: u64,
        limit: i32,
    ) -> BotResult<Vec<HealthRecord>> {
        let sql = "
            SELECT 
                t.name as tarantula_name,
                hcr.check_date,
                hs.status_name as status,
                hcr.weight_grams,
                hcr.humidity_percent,
                hcr.temperature_celsius,
                hcr.notes
            FROM health_check_records hcr
            JOIN tarantulas t ON hcr.tarantula_id = t.id
            JOIN health_statuses hs ON hcr.health_status_id = hs.id
            WHERE t.user_id = ?
            ORDER BY hcr.check_date DESC
            LIMIT ?";

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        let records = stmt.query_map([user_id, limit as u64], |row| {
            Ok(HealthRecord {
                tarantula_name: row.get(0)?,
                check_date: row.get(1)?,
                status: row.get(2)?,
                weight_grams: row.get(3)?,
                humidity_percent: row.get(4)?,
                temperature_celsius: row.get(5)?,
                notes: row.get(6)?,
            })
        })?;

        records
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    pub(crate) async fn get_recent_molt_records(
        &self,
        user_id: u64,
        limit: i32,
    ) -> BotResult<Vec<MoltRecord>> {
        let sql = "
            SELECT 
                t.name as tarantula_name,
                mr.molt_date,
                ms.stage_name as stage,
                mr.pre_molt_length_cm,
                mr.complications,
                mr.notes
            FROM molt_records mr
            JOIN tarantulas t ON mr.tarantula_id = t.id
            JOIN molt_stages ms ON mr.molt_stage_id = ms.id
            WHERE t.user_id = ?
            ORDER BY mr.molt_date DESC
            LIMIT ?";

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        let records = stmt.query_map([user_id, limit as u64], |row| {
            Ok(MoltRecord {
                tarantula_name: row.get(0)?,
                molt_date: row.get(1)?,
                stage: row.get(2)?,
                pre_molt_length_cm: row.get(3)?,
                complications: row.get(4)?,
                notes: row.get(5)?,
            })
        })?;

        records
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    pub(crate) async fn update_colony_count(
        &self,
        colony_id: i64,
        adjustment: i32,
        user_id: u64,
    ) -> BotResult<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "UPDATE cricket_colonies
            SET current_count = current_count + ?
            WHERE id = ? AND user_id = ?",
            params![adjustment, colony_id, user_id],
        )?;
        Ok(())
    }
}
