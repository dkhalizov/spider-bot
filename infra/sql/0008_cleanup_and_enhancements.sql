-- Add missing indices for feeding_events
CREATE INDEX IF NOT EXISTS idx_feeding_events_user_id ON feeding_events (user_id);

-- Create enclosures table with essential info only
CREATE TABLE IF NOT EXISTS enclosures
(
    id                 INTEGER PRIMARY KEY,
    name               VARCHAR(50),
    height_cm          INTEGER,
    width_cm           INTEGER,
    length_cm          INTEGER,
    substrate_depth_cm INTEGER,
    notes              TEXT,
    user_id            BIGINT REFERENCES telegram_users (telegram_id),
    created_at         TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Add enclosure reference to tarantulas
ALTER TABLE tarantulas
    ADD COLUMN enclosure_id INTEGER REFERENCES enclosures (id);

-- Create maintenance_records table for basic tracking
CREATE TABLE IF NOT EXISTS maintenance_records
(
    id                  INTEGER PRIMARY KEY,
    enclosure_id        INTEGER REFERENCES enclosures (id),
    maintenance_date    DATE NOT NULL,
    temperature_celsius DECIMAL(3, 1),
    humidity_percent    INTEGER,
    notes               TEXT,
    user_id             BIGINT REFERENCES telegram_users (telegram_id),
    created_at          TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indices
CREATE INDEX IF NOT EXISTS idx_maintenance_enclosure ON maintenance_records (enclosure_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_date ON maintenance_records (maintenance_date);
CREATE INDEX IF NOT EXISTS idx_maintenance_user ON maintenance_records (user_id);
CREATE INDEX IF NOT EXISTS idx_enclosures_user ON enclosures (user_id);

-- Create trigger for updated_at
CREATE TRIGGER IF NOT EXISTS update_enclosures_updated_at
    AFTER UPDATE
    ON enclosures
BEGIN
    UPDATE enclosures
    SET updated_at = datetime('now')
    WHERE id = NEW.id;
END;
