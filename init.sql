CREATE TABLE IF NOT EXISTS feeding_status_types
(
    id          INTEGER PRIMARY KEY,
    status      VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
    );

CREATE TABLE IF NOT EXISTS tarantula_species
(
    id                              INTEGER PRIMARY KEY,
    scientific_name                 VARCHAR(100) NOT NULL UNIQUE,
    common_name                     VARCHAR(100),
    adult_size_cm                   DECIMAL(4, 1),
    temperament                     VARCHAR(50),
    humidity_requirement_percent    INTEGER,
    temperature_requirement_celsius DECIMAL(3, 1)
    );
CREATE INDEX IF NOT EXISTS idx_tarantula_species_common_name ON tarantula_species (common_name);

CREATE TABLE IF NOT EXISTS molt_stages
(
    id          INTEGER PRIMARY KEY,
    stage_name  VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
    );

CREATE TABLE IF NOT EXISTS cricket_size_types
(
    id                    INTEGER PRIMARY KEY,
    size_name             VARCHAR(50) NOT NULL UNIQUE,
    approximate_length_mm DECIMAL(3, 1)
    );

-- Main Tables

CREATE TABLE IF NOT EXISTS tarantulas
(
    id                       INTEGER PRIMARY KEY,
    name                     VARCHAR(50),
    species_id               INTEGER REFERENCES tarantula_species (id),
    acquisition_date         DATE NOT NULL,
    last_molt_date           DATE,
    estimated_age_months     INTEGER,
    current_molt_stage_id    INTEGER REFERENCES molt_stages (id),
    current_health_status_id INTEGER REFERENCES health_statuses (id),
    last_health_check_date   DATE,
    enclosure_number         VARCHAR(20) UNIQUE,
    notes                    TEXT,
    created_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
CREATE INDEX IF NOT EXISTS idx_tarantulas_species ON tarantulas (species_id);
CREATE INDEX IF NOT EXISTS idx_tarantulas_health_status ON tarantulas (current_health_status_id);
CREATE INDEX IF NOT EXISTS idx_tarantulas_molt_stage ON tarantulas (current_molt_stage_id);
CREATE INDEX IF NOT EXISTS idx_tarantulas_acquisition_date ON tarantulas (acquisition_date);
CREATE INDEX IF NOT EXISTS idx_tarantulas_last_molt_date ON tarantulas (last_molt_date);

CREATE TABLE IF NOT EXISTS cricket_colonies
(
    id               INTEGER PRIMARY KEY,
    colony_name      VARCHAR(50),
    size_type_id     INTEGER REFERENCES cricket_size_types (id),
    current_count    INTEGER,
    last_count_date  DATE,
    container_number VARCHAR(20) UNIQUE,
    notes            TEXT,
    created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
CREATE INDEX IF NOT EXISTS idx_cricket_colonies_size_type ON cricket_colonies (size_type_id);
CREATE INDEX IF NOT EXISTS idx_cricket_colonies_last_count_date ON cricket_colonies (last_count_date);

CREATE TABLE IF NOT EXISTS feeding_events
(
    id                 INTEGER PRIMARY KEY,
    tarantula_id       INTEGER REFERENCES tarantulas (id),
    feeding_date       TIMESTAMP NOT NULL,
    cricket_colony_id  INTEGER REFERENCES cricket_colonies (id),
    number_of_crickets INTEGER   NOT NULL,
    feeding_status_id  INTEGER REFERENCES feeding_status_types (id),
    notes              TEXT,
    created_at         TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
CREATE INDEX IF NOT EXISTS idx_feeding_events_tarantula ON feeding_events (tarantula_id);
CREATE INDEX IF NOT EXISTS idx_feeding_events_colony ON feeding_events (cricket_colony_id);
CREATE INDEX IF NOT EXISTS idx_feeding_events_date ON feeding_events (feeding_date);
CREATE INDEX IF NOT EXISTS idx_feeding_events_status ON feeding_events (feeding_status_id);

CREATE TABLE IF NOT EXISTS molt_records
(
    id                  INTEGER PRIMARY KEY,
    tarantula_id        INTEGER REFERENCES tarantulas (id),
    molt_date           DATE NOT NULL,
    molt_stage_id       INTEGER REFERENCES molt_stages (id),
    pre_molt_length_cm  DECIMAL(4, 1),
    post_molt_length_cm DECIMAL(4, 1),
    complications       TEXT,
    notes               TEXT,
    created_at          TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
CREATE INDEX IF NOT EXISTS idx_molt_records_tarantula ON molt_records (tarantula_id);
CREATE INDEX IF NOT EXISTS idx_molt_records_date ON molt_records (molt_date);
CREATE INDEX IF NOT EXISTS idx_molt_records_stage ON molt_records (molt_stage_id);

CREATE TABLE IF NOT EXISTS health_check_records
(
    id                  INTEGER PRIMARY KEY,
    tarantula_id        INTEGER REFERENCES tarantulas (id),
    check_date          DATE NOT NULL,
    health_status_id    INTEGER REFERENCES health_statuses (id),
    weight_grams        DECIMAL(5, 2),
    humidity_percent    INTEGER,
    temperature_celsius DECIMAL(3, 1),
    abnormalities       TEXT,
    notes               TEXT,
    created_at          TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
CREATE INDEX IF NOT EXISTS idx_health_checks_tarantula ON health_check_records (tarantula_id);
CREATE INDEX IF NOT EXISTS idx_health_checks_date ON health_check_records (check_date);
CREATE INDEX IF NOT EXISTS idx_health_checks_status ON health_check_records (health_status_id);

CREATE TRIGGER IF NOT EXISTS update_tarantulas_updated_at
    AFTER UPDATE
                                ON tarantulas
BEGIN
UPDATE tarantulas
SET updated_at = datetime('now')
WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_cricket_colonies_updated_at
    AFTER UPDATE
                                ON cricket_colonies
BEGIN
UPDATE cricket_colonies
SET updated_at = datetime('now')
WHERE id = NEW.id;
END;

CREATE TABLE IF NOT EXISTS health_statuses
(
    id          INTEGER PRIMARY KEY,
    status_name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
    );

CREATE TABLE IF NOT EXISTS feeding_statuses
(
    id          INTEGER PRIMARY KEY,
    status_name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
    );

INSERT OR IGNORE INTO health_statuses (id, status_name, description)
VALUES (1, 'Healthy', 'Normal health status with no concerns'),
       (2, 'Monitor', 'Requires extra attention and monitoring'),
       (3, 'Critical', 'Immediate attention required');

-- Insert default feeding statuses
INSERT OR IGNORE INTO feeding_statuses (id, status_name, description)
VALUES (1, 'Accepted', 'Food was accepted normally'),
       (2, 'Rejected', 'Food was rejected'),
       (3, 'Partial', 'Only part of the food was consumed');
