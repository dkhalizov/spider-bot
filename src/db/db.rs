use crate::db::init::fill_default_enums;
use crate::error::BotError;
use crate::models::cricket::ColonyStatus;
use crate::models::enums::{CricketSize, FeedingStatus, HealthStatus, MoltStage};
use crate::models::feeding::{FeedingEvent, FeedingRecord};
use crate::models::health::{HealthAlert, HealthRecord};
use crate::models::molt::MoltRecord;
use crate::models::new::{Enclosure, FeedingFrequency, FeedingSchedule, MaintenanceRecord};
use crate::models::tarantula::{MaintenanceTask, Tarantula, TarantulaListItem};
use crate::models::user::TelegramUser;
use crate::BotResult;
use async_trait::async_trait;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension, Row};

#[async_trait]
pub trait TarantulaOperations: Send + Sync {
    async fn add_tarantula(&self, user_id: u64, params: AddTarantulaParams)
        -> Result<(), BotError>;
    async fn get_tarantula_by_id(&self, user_id: u64, id: i64) -> Result<Tarantula, BotError>;
    async fn get_all_tarantulas(&self, user_id: u64) -> Result<Vec<TarantulaListItem>, BotError>;
    async fn get_tarantulas_due_feeding(
        &self,
        user_id: u64,
    ) -> Result<Vec<TarantulaListItem>, BotError>;
    async fn update_tarantula_enclosure(
        &self,
        tarantula_id: i64,
        enclosure_id: Option<i64>,
        user_id: u64,
    ) -> Result<(), BotError>;

    async fn record_feeding(&self, user_id: u64, event: FeedingEvent) -> Result<i64, BotError>;
    async fn get_recent_feeding_records(
        &self,
        user_id: u64,
        limit: i32,
    ) -> Result<Vec<FeedingRecord>, BotError>;
    async fn get_feeding_schedule(
        &self,
        species_id: i64,
        body_length_cm: f32,
    ) -> Result<Option<FeedingSchedule>, BotError>;
    async fn get_feeding_frequency(&self, id: i64) -> Result<Option<FeedingFrequency>, BotError>;

    async fn record_health_check(
        &self,
        user_id: u64,
        tarantula_id: i64,
        status: HealthStatus,
        notes: Option<String>,
    ) -> Result<(), BotError>;
    async fn get_recent_health_records(
        &self,
        user_id: u64,
        limit: i32,
    ) -> Result<Vec<HealthRecord>, BotError>;
    async fn get_health_alerts(&self, user_id: u64) -> Result<Vec<HealthAlert>, BotError>;

    async fn record_molt(
        &self,
        tarantula_id: i64,
        length_cm: f32,
        complications: Option<String>,
        notes: Option<String>,
        user_id: u64,
    ) -> Result<(), BotError>;
    async fn get_recent_molt_records(
        &self,
        user_id: u64,
        limit: i32,
    ) -> Result<Vec<MoltRecord>, BotError>;

    async fn add_colony(&self, user_id: u64, params: AddColonyParams) -> Result<(), BotError>;
    async fn get_colony_status(&self, user_id: u64) -> Result<Vec<ColonyStatus>, BotError>;
    async fn update_colony_count(
        &self,
        colony_id: i64,
        adjustment: i32,
        user_id: u64,
    ) -> Result<(), BotError>;

    async fn create_maintenance_record(&self, record: MaintenanceRecord) -> Result<i64, BotError>;
    async fn get_maintenance_history(
        &self,
        enclosure_id: i64,
        user_id: u64,
    ) -> Result<Vec<MaintenanceRecord>, BotError>;
    async fn get_maintenance_tasks(&self, user_id: u64) -> Result<Vec<MaintenanceTask>, BotError>;

    async fn create_enclosure(&self, enclosure: Enclosure) -> Result<i64, BotError>;
    async fn get_enclosure(&self, id: i64, user_id: u64) -> Result<Enclosure, BotError>;

    async fn ensure_user_exists(&self, user: &TelegramUser) -> Result<(), BotError>;

    async fn get_current_size(&self, tarantula_id: i64) -> Result<f32, BotError>;
}

trait FromRow: Sized {
    fn from_row(row: &Row) -> rusqlite::Result<Self>;
}

impl FromRow for Tarantula {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            species_id: row.get("species_id")?,
            acquisition_date: row.get("acquisition_date")?,
            last_molt_date: row.get("last_molt_date")?,
            estimated_age_months: row.get("estimated_age_months")?,
            current_molt_stage_id: row.get("current_molt_stage_id")?,
            current_health_status_id: row.get("current_health_status_id")?,
            last_health_check_date: row.get("last_health_check_date")?,
            enclosure_number: row.get("enclosure_number")?,
            notes: row.get("notes")?,
        })
    }
}

pub struct TarantulaDB {
    pool: Pool<SqliteConnectionManager>,
}

#[derive(Debug)]
pub struct AddTarantulaParams {
    pub name: String,
    pub species_id: i64,
    pub acquisition_date: String,
    pub estimated_age_months: i64,
    pub enclosure_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct AddColonyParams {
    pub colony_name: String,
    pub size_type_id: i64,
    pub current_count: i32,
    pub container_number: String,
    pub notes: Option<String>,
}

impl TarantulaDB {
    pub fn new(db_path: &str) -> BotResult<Self> {
        let flags =
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE;

        let manager = SqliteConnectionManager::file(db_path).with_flags(flags);
        let pool = Pool::new(manager)?;

        fill_default_enums(pool.clone())?;
        Ok(Self { pool })
    }

    fn conn(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, BotError> {
        self.pool.get().map_err(Into::into)
    }
}

#[async_trait]
impl TarantulaOperations for TarantulaDB {
    async fn add_tarantula(&self, user_id: u64, params: AddTarantulaParams) -> BotResult<()> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO tarantulas (
            name, species_id, acquisition_date, estimated_age_months,
            enclosure_number, notes, user_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                params.name,
                params.species_id,
                params.acquisition_date,
                params.estimated_age_months,
                params.enclosure_number,
                params.notes,
                user_id,
            ],
        )?;
        Ok(())
    }
    async fn get_tarantula_by_id(&self, user_id: u64, id: i64) -> BotResult<Tarantula> {
        const SQL: &str = r#"SELECT id, name, species_id, acquisition_date, last_molt_date, estimated_age_months, current_molt_stage_id, current_health_status_id, last_health_check_date, enclosure_number, notes  FROM tarantulas WHERE id = ? AND user_id = ?"#;
        let conn = self.conn()?;
        let mut stmt = conn.prepare(SQL)?;
        stmt.query_row([id, user_id as i64], |row| Tarantula::from_row(row))
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    BotError::NotFound(format!("Tarantula with id {} not found", id))
                }
                e => BotError::Database(e),
            })
    }

    async fn get_all_tarantulas(&self, user_id: u64) -> BotResult<Vec<TarantulaListItem>> {
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

        let conn = self.conn()?;
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

    async fn get_tarantulas_due_feeding(&self, user_id: u64) -> BotResult<Vec<TarantulaListItem>> {
        let sql = "WITH LastFeeding AS (
            SELECT
                tarantula_id,
                MAX(feeding_date) as last_feeding_date,
                julianday('now') - julianday(MAX(feeding_date)) as days_since_feeding
            FROM feeding_events
            GROUP BY tarantula_id
        ),
        CurrentSize AS (
            -- Get the most recent size measurement from molt records
            SELECT
                t.id as tarantula_id,
                COALESCE(
                    mr.post_molt_length_cm,
                    -- If no measurements, use a size category based on species
                    CASE
                        WHEN ts.adult_size_cm <= 8 THEN ts.adult_size_cm * 0.3  -- Dwarf species start smaller
                        ELSE ts.adult_size_cm * 0.4  -- Regular species
                    END
                ) as current_size_cm
            FROM tarantulas t
            JOIN tarantula_species ts ON t.species_id = ts.id
            LEFT JOIN (
                SELECT tarantula_id, post_molt_length_cm
                FROM molt_records
                WHERE molt_date = (
                    SELECT MAX(molt_date)
                    FROM molt_records mr2
                    WHERE mr2.tarantula_id = molt_records.tarantula_id
                    AND mr2.post_molt_length_cm IS NOT NULL
                )
            ) mr ON t.id = mr.tarantula_id
        ),
        TarantulaSchedule AS (
            SELECT
                t.id as tarantula_id,
                fs.feeding_frequency,
                ff.min_days,
                ff.max_days,
                CASE
                    WHEN ms.stage_name = 'Pre-molt' THEN true
                    ELSE false
                END as is_premolt
            FROM tarantulas t
            JOIN tarantula_species ts ON t.species_id = ts.id
            JOIN CurrentSize cs ON t.id = cs.tarantula_id
            JOIN feeding_schedules fs ON ts.id = fs.species_id
            JOIN feeding_frequencies ff ON fs.frequency_id = ff.id
            LEFT JOIN molt_stages ms ON t.current_molt_stage_id = ms.id
            WHERE
                fs.size_category = (
                    SELECT size_category
                    FROM feeding_schedules fs2
                    WHERE fs2.species_id = ts.id
                    AND fs2.body_length_cm >= cs.current_size_cm
                    ORDER BY fs2.body_length_cm ASC
                    LIMIT 1
                )
        )
        SELECT
            t.id,
            t.name,
            ts.common_name as species_name,
            t.enclosure_number,
            COALESCE(lf.days_since_feeding, 999) as days_since_feeding,
            CASE
                WHEN ts2.is_premolt THEN 'In pre-molt'
                WHEN lf.days_since_feeding IS NULL THEN 'Never fed'
                WHEN lf.days_since_feeding > ts2.max_days THEN
                    'Overdue feeding (' || ts2.feeding_frequency || ')'
                ELSE 'Due for feeding'
            END as current_status
        FROM tarantulas t
        JOIN tarantula_species ts ON t.species_id = ts.id
        JOIN TarantulaSchedule ts2 ON t.id = ts2.tarantula_id
        LEFT JOIN LastFeeding lf ON t.id = lf.tarantula_id
        WHERE
            t.user_id = ? AND
            NOT ts2.is_premolt AND
            (
                lf.days_since_feeding IS NULL OR
                lf.days_since_feeding > ts2.max_days
            )
        ORDER BY
            CASE
                WHEN lf.days_since_feeding IS NULL THEN 999
                ELSE lf.days_since_feeding
            END DESC".to_string();

        let conn = self.conn()?;
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
    
    async fn update_tarantula_enclosure(
        &self,
        tarantula_id: i64,
        enclosure_id: Option<i64>,
        user_id: u64,
    ) -> BotResult<()> {
        let conn = self.conn()?;
        conn.execute(
            "UPDATE tarantulas
             SET enclosure_id = ?
             WHERE id = ? AND user_id = ?",
            params![enclosure_id, tarantula_id, user_id],
        )?;
        Ok(())
    }
    async fn record_feeding(&self, user_id: u64, event: FeedingEvent) -> BotResult<i64> {
        let mut conn = self.conn()?;
        transactionally(&mut conn, |tx| {
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

            Ok(tx.last_insert_rowid())
        })
    }

    async fn get_recent_feeding_records(
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

        let conn = self.conn()?;
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

    
    async fn get_feeding_schedule(
        &self,
        species_id: i64,
        body_length_cm: f32,
    ) -> BotResult<Option<FeedingSchedule>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT fs.species_id, fs.size_category, fs.body_length_cm, fs.prey_size,
                    fs.feeding_frequency, fs.prey_type, fs.notes, fs.frequency_id
             FROM feeding_schedules fs
             WHERE fs.species_id = ?
             AND fs.body_length_cm >= ?
             ORDER BY fs.body_length_cm ASC
             LIMIT 1",
        )?;

        let schedule = stmt
            .query_row(params![species_id, body_length_cm], |row| {
                Ok(FeedingSchedule {
                    species_id: row.get(0)?,
                    size_category: row.get(1)?,
                    body_length_cm: row.get(2)?,
                    prey_size: row.get(3)?,
                    feeding_frequency: row.get(4)?,
                    prey_type: row.get(5)?,
                    notes: row.get(6)?,
                    frequency_id: row.get(7)?,
                })
            })
            .optional()?;

        Ok(schedule)
    }

    async fn get_feeding_frequency(&self, id: i64) -> BotResult<Option<FeedingFrequency>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, frequency_name, min_days, max_days, description
             FROM feeding_frequencies
             WHERE id = ?",
        )?;

        let frequency = stmt
            .query_row(params![id], |row| {
                Ok(FeedingFrequency {
                    id: row.get(0)?,
                    frequency_name: row.get(1)?,
                    min_days: row.get(2)?,
                    max_days: row.get(3)?,
                    description: row.get(4)?,
                })
            })
            .optional()?;

        Ok(frequency)
    }

    async fn record_health_check(
        &self,
        user_id: u64,
        tarantula_id: i64,
        status: HealthStatus,
        notes: Option<String>,
    ) -> BotResult<()> {
        let mut conn = self.conn()?;
        let tx = conn.transaction()?;
        let status_id = status as i64;

        
        let rows_affected = tx.execute(
            "UPDATE tarantulas SET 
            last_health_check_date = date('now'),
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

    async fn get_recent_health_records(
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

        let conn = self.conn()?;
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

    async fn get_health_alerts(&self, user_id: u64) -> BotResult<Vec<HealthAlert>> {
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

        let conn = self.conn()?;
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

    async fn record_molt(
        &self,
        tarantula_id: i64,
        length_cm: f32,
        complications: Option<String>,
        notes: Option<String>,
        user_id: u64,
    ) -> BotResult<()> {
        let mut conn = self.conn()?;
        let tx = conn.transaction()?;
        let post_molt_id = MoltStage::PostMolt as i64;

        
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

        
        tx.execute(
            "INSERT INTO molt_records (
            tarantula_id, molt_date, molt_stage_id,
            post_molt_length_cm, complications, notes, user_id
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

    async fn get_recent_molt_records(
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
                mr.notes,
                mr.post_molt_length_cm
            FROM molt_records mr
            JOIN tarantulas t ON mr.tarantula_id = t.id
            JOIN molt_stages ms ON mr.molt_stage_id = ms.id
            WHERE t.user_id = ?
            ORDER BY mr.molt_date DESC
            LIMIT ?";

        let conn = self.conn()?;
        let mut stmt = conn.prepare(sql)?;
        let records = stmt.query_map([user_id, limit as u64], |row| {
            Ok(MoltRecord {
                tarantula_name: row.get(0)?,
                molt_date: row.get(1)?,
                stage: row.get(2)?,
                pre_molt_length_cm: row.get(3)?,
                complications: row.get(4)?,
                notes: row.get(5)?,
                post_molt_length_cm: row.get(6)?,
            })
        })?;

        records
            .collect::<Result<Vec<_>, _>>()
            .map_err(BotError::Database)
    }

    async fn add_colony(&self, user_id: u64, params: AddColonyParams) -> BotResult<()> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO cricket_colonies (
            colony_name, size_type_id, current_count, container_number, notes, user_id
        ) VALUES (?, ?, ?, ?, ?, ?)",
            params![
                params.colony_name,
                params.size_type_id,
                params.current_count,
                params.container_number,
                params.notes,
                user_id,
            ],
        )?;
        Ok(())
    }

    async fn get_colony_status(&self, user_id: u64) -> BotResult<Vec<ColonyStatus>> {
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

        let conn = self.conn()?;
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
    async fn update_colony_count(
        &self,
        colony_id: i64,
        adjustment: i32,
        user_id: u64,
    ) -> BotResult<()> {
        let conn = self.conn()?;
        conn.execute(
            "UPDATE cricket_colonies
            SET current_count = current_count + ?
            WHERE id = ? AND user_id = ?",
            params![adjustment, colony_id, user_id],
        )?;
        Ok(())
    }
    async fn create_maintenance_record(&self, record: MaintenanceRecord) -> BotResult<i64> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO maintenance_records (enclosure_id, maintenance_date, temperature_celsius,
             humidity_percent, notes, user_id)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                record.enclosure_id,
                record.maintenance_date,
                record.temperature_celsius,
                record.humidity_percent,
                record.notes,
                record.user_id,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }
    async fn get_maintenance_history(
        &self,
        enclosure_id: i64,
        user_id: u64,
    ) -> BotResult<Vec<MaintenanceRecord>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, enclosure_id, maintenance_date, temperature_celsius, humidity_percent, notes, user_id
             FROM maintenance_records
             WHERE enclosure_id = ? AND user_id = ?
             ORDER BY maintenance_date DESC",
        )?;

        let records = stmt
            .query_map(params![enclosure_id, user_id], |row| {
                Ok(MaintenanceRecord {
                    id: Some(row.get(0)?),
                    enclosure_id: row.get(1)?,
                    maintenance_date: row.get(2)?,
                    temperature_celsius: row.get(3)?,
                    humidity_percent: row.get(4)?,
                    notes: row.get(5)?,
                    user_id: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    async fn get_maintenance_tasks(&self, user_id: u64) -> BotResult<Vec<MaintenanceTask>> {
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

        let conn = self.conn()?;
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

    async fn create_enclosure(&self, enclosure: Enclosure) -> BotResult<i64> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO enclosures (name, height_cm, width_cm, length_cm, substrate_depth_cm, notes, user_id)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                enclosure.name,
                enclosure.height_cm,
                enclosure.width_cm,
                enclosure.length_cm,
                enclosure.substrate_depth_cm,
                enclosure.notes,
                enclosure.user_id,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    async fn get_enclosure(&self, id: i64, user_id: u64) -> BotResult<Enclosure> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, height_cm, width_cm, length_cm, substrate_depth_cm, notes, user_id
             FROM enclosures
             WHERE id = ? AND user_id = ?",
        )?;

        let enclosure = stmt.query_row(params![id, user_id], |row| {
            Ok(Enclosure {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                height_cm: row.get(2)?,
                width_cm: row.get(3)?,
                length_cm: row.get(4)?,
                substrate_depth_cm: row.get(5)?,
                notes: row.get(6)?,
                user_id: row.get(7)?,
            })
        })?;

        Ok(enclosure)
    }

    async fn ensure_user_exists(&self, user: &TelegramUser) -> BotResult<()> {
        let conn = self.conn()?;
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

    
    async fn get_current_size(&self, tarantula_id: i64) -> BotResult<f32> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "WITH LastMoltSize AS (
    SELECT 
        tarantula_id,
        post_molt_length_cm,
        molt_date
    FROM molt_records 
    WHERE molt_date = (
        SELECT MAX(molt_date)
        FROM molt_records mr2
        WHERE mr2.tarantula_id = molt_records.tarantula_id
        AND mr2.post_molt_length_cm IS NOT NULL
    )
),
EstimatedSize AS (
    SELECT
        t.id as tarantula_id,
        COALESCE(
            lms.post_molt_length_cm,
            CASE 
                -- If we have a molt stage, use feeding schedule size
                WHEN t.current_molt_stage_id IS NOT NULL THEN
                    (SELECT fs.body_length_cm
                     FROM feeding_schedules fs
                     WHERE fs.species_id = t.species_id
                     AND CASE 
                         WHEN ms.stage_name LIKE '%spiderling%' THEN 'Spiderling'
                         WHEN ms.stage_name LIKE '%juvenile%' THEN 'Juvenile'
                         WHEN ms.stage_name LIKE '%sub%adult%' THEN 'Sub-Adult'
                         WHEN ms.stage_name LIKE '%adult%' THEN 'Adult'
                     END = fs.size_category
                     LIMIT 1)
                -- If we have estimated age, make a reasonable guess
                WHEN t.estimated_age_months IS NOT NULL THEN
                    CASE 
                        WHEN t.estimated_age_months < 6 THEN ts.adult_size_cm * 0.2
                        WHEN t.estimated_age_months < 12 THEN ts.adult_size_cm * 0.4
                        WHEN t.estimated_age_months < 24 THEN ts.adult_size_cm * 0.6
                        ELSE ts.adult_size_cm * 0.8
                    END
                -- Last resort: use adult size if acquisition date > 2 years
                ELSE 
                    CASE 
                        WHEN julianday('now') - julianday(t.acquisition_date) > 730 THEN ts.adult_size_cm
                        ELSE ts.adult_size_cm * 0.5
                    END
            END
        ) as current_size_cm
    FROM tarantulas t
    JOIN tarantula_species ts ON t.species_id = ts.id
    LEFT JOIN molt_stages ms ON t.current_molt_stage_id = ms.id
    LEFT JOIN LastMoltSize lms ON t.id = lms.tarantula_id
    WHERE t.id = ?
)
SELECT current_size_cm FROM EstimatedSize;",
        )?;

        let size = stmt.query_row(params![tarantula_id], |row| row.get::<_, f32>(0))?;
        Ok(size)
    }
}

fn transactionally<T>(
    conn: &mut rusqlite::Connection,
    f: impl FnOnce(&rusqlite::Transaction) -> Result<T, BotError>,
) -> Result<T, BotError> {
    let tx = conn.transaction()?;
    let result = f(&tx)?;
    tx.commit()?;
    Ok(result)
}
