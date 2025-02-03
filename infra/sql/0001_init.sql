-- auto-generated definition
create table if not exists cricket_size_types
(
    id                    INTEGER
        primary key,
    size_name             VARCHAR(50) not null
        unique,
    approximate_length_mm DECIMAL(3, 1)
);

-- auto-generated definition
create table if not exists telegram_users
(
    id          INTEGER
        primary key,
    telegram_id BIGINT not null
        unique,
    username    VARCHAR(255),
    first_name  VARCHAR(255),
    last_name   VARCHAR(255),
    created_at  TIMESTAMP default CURRENT_TIMESTAMP,
    last_active TIMESTAMP default CURRENT_TIMESTAMP
);

-- auto-generated definition
create table if not exists tarantula_species
(
    id                              INTEGER
        primary key,
    scientific_name                 VARCHAR(100) not null
        unique,
    common_name                     VARCHAR(100),
    adult_size_cm                   DECIMAL(4, 1),
    temperament                     VARCHAR(50),
    humidity_requirement_percent    INTEGER,
    temperature_requirement_celsius DECIMAL(3, 1)
);

create index if not exists idx_tarantula_species_common_name
    on tarantula_species (common_name);

-- auto-generated definition
create table if not exists molt_stages
(
    id          INTEGER
        primary key,
    stage_name  VARCHAR(50) not null
        unique,
    description TEXT
);

-- auto-generated definition
create table if not exists cricket_colonies
(
    id               INTEGER
        primary key,
    colony_name      VARCHAR(50),
    size_type_id     INTEGER
        references cricket_size_types,
    current_count    INTEGER,
    last_count_date  DATE,
    container_number VARCHAR(20)
        unique,
    notes            TEXT,
    created_at       TIMESTAMP default CURRENT_TIMESTAMP,
    updated_at       TIMESTAMP default CURRENT_TIMESTAMP,
    user_id          BIGINT
        references telegram_users (telegram_id)
);

create index if not exists idx_cricket_colonies_last_count_date
    on cricket_colonies (last_count_date);

create index if not exists idx_cricket_colonies_size_type
    on cricket_colonies (size_type_id);

create index if not exists idx_cricket_colonies_user_id
    on cricket_colonies (user_id);

-- auto-generated definition
create table if not exists enclosures
(
    id                 INTEGER
        primary key,
    name               VARCHAR(50),
    height_cm          INTEGER,
    width_cm           INTEGER,
    length_cm          INTEGER,
    substrate_depth_cm INTEGER,
    notes              TEXT,
    user_id            BIGINT
        references telegram_users (telegram_id),
    created_at         TIMESTAMP default CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP default CURRENT_TIMESTAMP
);

create index if not exists idx_enclosures_user
    on enclosures (user_id);

-- auto-generated definition
create table if not exists feeding_events
(
    id                 INTEGER
        primary key,
    tarantula_id       INTEGER
        references tarantulas,
    feeding_date       TIMESTAMP not null,
    cricket_colony_id  INTEGER
        references cricket_colonies,
    number_of_crickets INTEGER   not null,
    feeding_status_id  INTEGER
        references feeding_statuses,
    notes              TEXT,
    created_at         TIMESTAMP default CURRENT_TIMESTAMP,
    user_id            bigint
);
-- auto-generated definition
create table if not exists feeding_frequencies
(
    id             INTEGER
        primary key,
    frequency_name VARCHAR(50) not null
        unique,
    min_days       INTEGER     not null,
    max_days       INTEGER     not null,
    description    TEXT
);

-- auto-generated definition
create table if not exists feeding_schedules
(
    id                INTEGER
        primary key autoincrement,
    species_id        INTEGER
        references tarantula_species,
    size_category     TEXT,
    body_length_cm    DECIMAL(3, 1),
    prey_size         TEXT,
    feeding_frequency TEXT,
    prey_type         TEXT,
    notes             TEXT,
    frequency_id      INTEGER
        references feeding_frequencies
);

-- auto-generated definition
create table if not exists feeding_statuses
(
    id          INTEGER
        primary key,
    status_name VARCHAR(50) not null
        unique,
    description TEXT
);
-- auto-generated definition
create table if not exists tarantula_species
(
    id                              INTEGER
        primary key,
    scientific_name                 VARCHAR(100) not null
        unique,
    common_name                     VARCHAR(100),
    adult_size_cm                   DECIMAL(4, 1),
    temperament                     VARCHAR(50),
    humidity_requirement_percent    INTEGER,
    temperature_requirement_celsius DECIMAL(3, 1)
);

create index if not exists idx_tarantula_species_common_name
    on tarantula_species (common_name);


-- auto-generated definition
create table if not exists health_check_records
(
    id                  INTEGER
        primary key,
    tarantula_id        INTEGER
        references tarantulas,
    check_date          DATE not null,
    health_status_id    INTEGER
        references health_statuses,
    weight_grams        DECIMAL(5, 2),
    humidity_percent    INTEGER,
    temperature_celsius DECIMAL(3, 1),
    abnormalities       TEXT,
    notes               TEXT,
    created_at          TIMESTAMP default CURRENT_TIMESTAMP,
    user_id             BIGINT
        references telegram_users (telegram_id)
);

create index if not exists idx_health_checks_date
    on health_check_records (check_date);

create index if not exists idx_health_checks_status
    on health_check_records (health_status_id);

create index if not exists idx_health_checks_tarantula
    on health_check_records (tarantula_id);

create index if not exists idx_health_checks_user_id
    on health_check_records (user_id);

-- auto-generated definition
create table if not exists health_statuses
(
    id          INTEGER
        primary key,
    status_name VARCHAR(50) not null
        unique,
    description TEXT
);

-- auto-generated definition
create table if not exists maintenance_records
(
    id                  INTEGER
        primary key,
    enclosure_id        INTEGER
        references enclosures,
    maintenance_date    DATE not null,
    temperature_celsius DECIMAL(3, 1),
    humidity_percent    INTEGER,
    notes               TEXT,
    user_id             BIGINT
        references telegram_users (telegram_id),
    created_at          TIMESTAMP default CURRENT_TIMESTAMP
);

create index if not exists idx_maintenance_date
    on maintenance_records (maintenance_date);

create index if not exists idx_maintenance_enclosure
    on maintenance_records (enclosure_id);

create index if not exists idx_maintenance_user
    on maintenance_records (user_id);

-- auto-generated definition
create table if not exists molt_records
(
    id                  INTEGER
        primary key,
    tarantula_id        INTEGER
        references tarantulas,
    molt_date           DATE not null,
    molt_stage_id       INTEGER
        references molt_stages,
    pre_molt_length_cm  DECIMAL(4, 1),
    post_molt_length_cm DECIMAL(4, 1),
    complications       TEXT,
    notes               TEXT,
    created_at          TIMESTAMP default CURRENT_TIMESTAMP,
    user_id             BIGINT
        references telegram_users (telegram_id)
);

create index if not exists idx_molt_records_date
    on molt_records (molt_date);

create index if not exists idx_molt_records_stage
    on molt_records (molt_stage_id);

create index if not exists idx_molt_records_tarantula
    on molt_records (tarantula_id);

create index if not exists idx_molt_records_user_id
    on molt_records (user_id);



create index if not exists idx_feeding_events_colony
    on feeding_events (cricket_colony_id);

create index if not exists idx_feeding_events_date
    on feeding_events (feeding_date);

create index if not exists idx_feeding_events_status
    on feeding_events (feeding_status_id);

create index if not exists idx_feeding_events_tarantula
    on feeding_events (tarantula_id);

create index if not exists idx_feeding_events_user_id
    on feeding_events (user_id);

-- auto-generated definition
create table if not exists tarantulas
(
    id                       INTEGER
        primary key,
    name                     VARCHAR(50),
    species_id               INTEGER
        references tarantula_species,
    acquisition_date         DATE not null,
    last_molt_date           DATE,
    estimated_age_months     INTEGER,
    current_molt_stage_id    INTEGER
        references molt_stages,
    current_health_status_id INTEGER
        references health_statuses,
    last_health_check_date   DATE,
    enclosure_number         VARCHAR(20)
        unique,
    notes                    TEXT,
    created_at               TIMESTAMP default CURRENT_TIMESTAMP,
    updated_at               TIMESTAMP default CURRENT_TIMESTAMP,
    user_id                  BIGINT
        references telegram_users (telegram_id),
    enclosure_id             INTEGER
        references enclosures
);

create index if not exists idx_tarantulas_acquisition_date
    on tarantulas (acquisition_date);

create index if not exists idx_tarantulas_health_status
    on tarantulas (current_health_status_id);

create index if not exists idx_tarantulas_last_molt_date
    on tarantulas (last_molt_date);

create index if not exists idx_tarantulas_molt_stage
    on tarantulas (current_molt_stage_id);

create index if not exists idx_tarantulas_species
    on tarantulas (species_id);

create index if not exists idx_tarantulas_user_id
    on tarantulas (user_id);

UPDATE feeding_schedules
SET frequency_id = (SELECT id
                    FROM feeding_frequencies
                    WHERE frequency_name = feeding_frequency);

-- Create trigger for updated_at
CREATE TRIGGER IF NOT EXISTS update_enclosures_updated_at
    AFTER UPDATE
    ON enclosures
BEGIN
    UPDATE enclosures
    SET updated_at = datetime('now')
    WHERE id = NEW.id;
END;
