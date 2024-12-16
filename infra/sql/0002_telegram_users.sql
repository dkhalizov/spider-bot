CREATE TABLE IF NOT EXISTS telegram_users (
                                              id INTEGER PRIMARY KEY,
                                              telegram_id BIGINT NOT NULL UNIQUE,
                                              username VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

ALTER TABLE tarantulas ADD COLUMN user_id BIGINT REFERENCES telegram_users(telegram_id);
ALTER TABLE cricket_colonies ADD COLUMN user_id BIGINT REFERENCES telegram_users(telegram_id);

CREATE INDEX IF NOT EXISTS idx_tarantulas_user_id ON tarantulas(user_id);
CREATE INDEX IF NOT EXISTS idx_cricket_colonies_user_id ON cricket_colonies(user_id);

ALTER TABLE feeding_events ADD COLUMN user_id BIGINT REFERENCES telegram_users(telegram_id);
ALTER TABLE molt_records ADD COLUMN user_id BIGINT REFERENCES telegram_users(telegram_id);
ALTER TABLE health_check_records ADD COLUMN user_id BIGINT REFERENCES telegram_users(telegram_id);
ALTER TABLE cricket_colony_maintenance ADD COLUMN user_id BIGINT REFERENCES telegram_users(telegram_id);

CREATE INDEX IF NOT EXISTS idx_feeding_events_user_id ON feeding_events(user_id);
CREATE INDEX IF NOT EXISTS idx_molt_records_user_id ON molt_records(user_id);
CREATE INDEX IF NOT EXISTS idx_health_checks_user_id ON health_check_records(user_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_user_id ON cricket_colony_maintenance(user_id);