use crate::db::init::DbInitializer;
use crate::error::TarantulaError;
use crate::models::cricket::{ColonyMaintenanceRecord, ColonyStatus};
use crate::models::enums::{CricketSize, FeedingStatus, HealthStatus, MoltStage};
use crate::models::feeding::{FeedingEvent, FeedingRecord};
use crate::models::health::{HealthAlert, HealthRecord};
use crate::models::molt::MoltRecord;
use crate::models::tarantula::{MaintenanceTask, Tarantula, TarantulaListItem};
use crate::TarantulaResult;
use rusqlite::{params, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TarantulaDB {
    conn: Arc<Mutex<Connection>>,
}

impl TarantulaDB {
    pub fn new(db_path: &str) -> TarantulaResult<Self> {
        let flags = rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
            | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
            | rusqlite::OpenFlags::SQLITE_OPEN_SHARED_CACHE;

        let conn = Connection::open_with_flags(db_path, flags)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        let initializer = DbInitializer::new(conn);
        initializer.initialize()?;
        let conn = initializer.into_inner();
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn get_all_tarantulas(&self) -> TarantulaResult<Vec<TarantulaListItem>> {
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
        GROUP BY t.id
        ORDER BY t.name";

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        let items = stmt
            .query_map(
                params![
                    MoltStage::PreMolt.to_db_name(),
                    MoltStage::PreMolt.to_db_name(),
                    HealthStatus::Critical.to_db_name(),
                    HealthStatus::Critical.to_db_name()
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
            .map_err(TarantulaError::Database)?;

        items
            .collect::<rusqlite::Result<Vec<_>, _>>()
            .map_err(TarantulaError::Database)
    }
    pub async fn get_tarantulas_due_feeding(&self) -> TarantulaResult<Vec<TarantulaListItem>> {
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
        WHERE ms.stage_name != '{}'
        GROUP BY t.id
        HAVING days_since_feeding > 7
        ORDER BY days_since_feeding DESC",
            MoltStage::PreMolt.to_db_name()
        );

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(&sql)?;
        let items = stmt
            .query_map([], |row| {
                Ok(TarantulaListItem {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    species_name: row.get(2)?,
                    enclosure_number: row.get(3)?,
                    days_since_feeding: row.get(4)?,
                    current_status: row.get(5)?,
                })
            })
            .map_err(TarantulaError::Database)?;

        items
            .collect::<rusqlite::Result<Vec<_>, _>>()
            .map_err(TarantulaError::Database)
    }
    pub async fn record_feeding(&self, event: &FeedingEvent) -> TarantulaResult<i64> {
        let sql = "INSERT INTO feeding_events (
        tarantula_id, feeding_date, cricket_colony_id,
        number_of_crickets, feeding_status_id, notes
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

        let conn = self.conn.lock().await;
        conn.execute(
            sql,
            params![
                event.tarantula_id,
                event.feeding_date,
                event.cricket_colony_id,
                event.number_of_crickets,
                FeedingStatus::Accepted as i64,
                event.notes,
            ],
        )?;

        conn.execute(
            "UPDATE cricket_colonies
        SET current_count = current_count - ?1
        WHERE id = ?2",
            params![event.number_of_crickets, event.cricket_colony_id],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub async fn get_colony_status(&self) -> TarantulaResult<Vec<ColonyStatus>> {
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
        GROUP BY cc.id
        ORDER BY weeks_remaining ASC";

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        let colonies = stmt
            .query_map([], |row| {
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
            .map_err(TarantulaError::Database)?;

        colonies
            .collect::<Result<Vec<_>, _>>()
            .map_err(TarantulaError::Database)
    }

    pub async fn get_health_alerts(&self) -> TarantulaResult<Vec<HealthAlert>> {
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
    GROUP BY t.id
    HAVING alert_type != 'None'
    ORDER BY days_in_state DESC",
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name()
        );

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(&sql)?;
        let alerts = stmt
            .query_map([], |row| {
                Ok(HealthAlert {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    scientific_name: row.get(2)?,
                    alert_type: row.get(3)?,
                    days_in_state: row.get(4)?,
                })
            })
            .map_err(TarantulaError::Database)?;

        alerts
            .collect::<Result<Vec<_>, _>>()
            .map_err(TarantulaError::Database)
    }

    pub async fn get_maintenance_tasks(&self) -> TarantulaResult<Vec<MaintenanceTask>> {

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
    GROUP BY t.id
    HAVING required_action != 'Regular Check'
    ORDER BY priority, name",
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name(),
                          MoltStage::PreMolt.to_db_name()
        );

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(&sql)?;
        let tasks = stmt.query_map([], |row| {
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
            .map_err(TarantulaError::Database)
    }
    
    pub async fn get_tarantula_by_id(&self, id: i64) -> TarantulaResult<Tarantula> {
        let sql = "SELECT * FROM tarantulas WHERE id = ?";
        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        stmt.query_row([id], |row| {
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
                TarantulaError::NotFound(format!("Tarantula with id {} not found", id))
            }
            e => TarantulaError::Database(e),
        })
    }
    pub async fn record_molt(
        &self,
        tarantula_id: i64,
        length_cm: Option<f32>,
        complications: Option<String>,
        notes: Option<String>,
    ) -> TarantulaResult<()> {

        let mut guard = self.conn.lock().await;
        let tx = guard.transaction()?;
        let post_molt_id = MoltStage::PostMolt as i64;

        tx.execute(
            "INSERT INTO molt_records (
            tarantula_id, molt_date, molt_stage_id,
            pre_molt_length_cm, complications, notes
        ) VALUES (?, datetime('now'), ?, ?, ?, ?)",
            params![tarantula_id, post_molt_id, length_cm, complications, notes],
        )?;

        tx.execute(
            "UPDATE tarantulas SET 
            last_molt_date = datetime('now'),
            current_molt_stage_id = ?
        WHERE id = ?",
            params![post_molt_id, tarantula_id],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub async fn record_health_check(
        &self,
        tarantula_id: i64,
        status: HealthStatus,
        notes: Option<String>,
    ) -> TarantulaResult<()> {

        let mut guard = self.conn.lock().await;
        let tx = guard.transaction()?;
        let status_id = status as i64;

        let _check_id = tx.execute(
            "INSERT INTO health_check_records (
            tarantula_id, check_date, health_status_id,
            weight_grams, humidity_percent, temperature_celsius,
            notes
        ) VALUES (?, datetime('now'), ?, ?, ?, ?, ?)",
            params![tarantula_id, status_id, 0, 55, 20, notes],
        )?;

        tx.execute(
            "UPDATE tarantulas SET 
            last_health_check_date = datetime('now'),
            current_health_status_id = ?
        WHERE id = ?",
            params![status_id, tarantula_id],
        )?;

        tx.commit()?;
        Ok(())
    }
    pub(crate) async fn get_recent_feeding_records(
        &self,
        limit: i32,
    ) -> TarantulaResult<Vec<FeedingRecord>> {
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
            ORDER BY fe.feeding_date DESC
            LIMIT ?";

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        let records = stmt.query_map([limit], |row| {
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
            .map_err(TarantulaError::Database)
    }

    pub(crate) async fn get_recent_health_records(
        &self,
        limit: i32,
    ) -> TarantulaResult<Vec<HealthRecord>> {
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
            ORDER BY hcr.check_date DESC
            LIMIT ?";

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        let records = stmt.query_map([limit], |row| {
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
            .map_err(TarantulaError::Database)
    }

    pub(crate) async fn get_recent_molt_records(
        &self,
        limit: i32,
    ) -> TarantulaResult<Vec<MoltRecord>> {
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
            ORDER BY mr.molt_date DESC
            LIMIT ?";

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        let records = stmt.query_map([limit], |row| {
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
            .map_err(TarantulaError::Database)
    }

    pub(crate) async fn get_recent_colony_records(
        &self,
        limit: i32,
    ) -> TarantulaResult<Vec<ColonyMaintenanceRecord>> {
        let sql = "
            SELECT 
                cc.colony_name,
                ccm.maintenance_date,
                ccm.previous_count,
                ccm.new_count,
                ccm.food_added,
                ccm.water_gel_added,
                ccm.cleaning_performed,
                ccm.notes
            FROM cricket_colony_maintenance ccm
            JOIN cricket_colonies cc ON ccm.colony_id = cc.id
            ORDER BY ccm.maintenance_date DESC
            LIMIT ?";

        let guard = self.conn.lock().await;
        let mut stmt = guard.prepare(sql)?;
        let records = stmt.query_map([limit], |row| {
            Ok(ColonyMaintenanceRecord {
                colony_name: row.get(0)?,
                maintenance_date: row.get(1)?,
                previous_count: row.get(2)?,
                new_count: row.get(3)?,
                food_added: row.get(4)?,
                water_added: row.get(5)?,
                cleaning_performed: row.get(6)?,
                notes: row.get(7)?,
            })
        })?;

        records
            .collect::<Result<Vec<_>, _>>()
            .map_err(TarantulaError::Database)
    }
}
