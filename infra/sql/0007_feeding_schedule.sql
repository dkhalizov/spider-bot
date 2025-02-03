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

INSERT INTO feeding_schedules (species_id, size_category, body_length_cm, prey_size, feeding_frequency, prey_type,
                               notes)

SELECT id                            as species_id,
       'Spiderling'                  as size_category,
       CASE
           WHEN adult_size_cm <= 8 THEN 0.3 -- Dwarf species
           WHEN adult_size_cm <= 10 THEN 0.4 -- Small species
           ELSE 0.5 -- Medium and large species
           END                       as body_length_cm,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Pre-killed fruit fly'
           ELSE 'Pre-killed cricket leg/pinhead'
           END                       as prey_size,
       CASE
           WHEN adult_size_cm <= 8 THEN '3-4 times per week'
           ELSE '2-3 times per week'
           END                       as feeding_frequency,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Fruit flies, pre-killed'
           ELSE 'Pinhead crickets, fruit flies'
           END                       as prey_type,
       'Very delicate at this stage' as notes
FROM tarantula_species
UNION ALL

-- Juvenile stage 
SELECT id,
       'Juvenile',
       CASE
           WHEN adult_size_cm <= 8 THEN 1.0 -- Dwarf species
           WHEN adult_size_cm <= 10 THEN 1.5 -- Small species
           WHEN adult_size_cm <= 13 THEN 2.0 -- Medium species
           WHEN adult_size_cm <= 16 THEN 2.5 -- Large species
           ELSE 3.0 -- Giant species
           END,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Pinhead cricket/fruit fly'
           WHEN adult_size_cm <= 10 THEN 'Small cricket nymph'
           WHEN adult_size_cm <= 13 THEN 'Small cricket/roach nymph'
           WHEN adult_size_cm <= 16 THEN 'Medium cricket/roach'
           ELSE 'Medium-large cricket/roach'
           END,
       CASE
           WHEN adult_size_cm <= 10 THEN 'Every 4-5 days'
           ELSE 'Every 5-7 days'
           END,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Pinhead crickets, fruit flies'
           ELSE 'Crickets, roaches appropriate to size'
           END,
       'Regular feeding promotes growth'
FROM tarantula_species
UNION ALL

-- Sub-Adult stage 
SELECT id,
       'Sub-Adult',
       CASE
           WHEN adult_size_cm <= 8 THEN 2.0 -- Dwarf species
           WHEN adult_size_cm <= 10 THEN 3.0 -- Small species
           WHEN adult_size_cm <= 13 THEN 4.0 -- Medium species
           WHEN adult_size_cm <= 16 THEN 5.0 -- Large species
           ELSE 6.0 -- Giant species
           END,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Small cricket'
           WHEN adult_size_cm <= 10 THEN 'Medium cricket'
           WHEN adult_size_cm <= 13 THEN '1-2 medium crickets'
           WHEN adult_size_cm <= 16 THEN '2 medium crickets'
           ELSE '2-3 large crickets'
           END,
       CASE
           WHEN adult_size_cm <= 10 THEN 'Every 7 days'
           WHEN adult_size_cm <= 13 THEN 'Every 7-10 days'
           ELSE 'Every 10-14 days'
           END,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Small crickets, small roaches'
           WHEN adult_size_cm <= 10 THEN 'Medium crickets, small roaches'
           WHEN adult_size_cm <= 13 THEN 'Medium crickets, medium roaches'
           ELSE 'Large crickets, adult roaches'
           END,
       'Watch for premolt signs'
FROM tarantula_species
UNION ALL

-- Adult stage 
SELECT id,
       'Adult',
       adult_size_cm,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Medium cricket'
           WHEN adult_size_cm <= 10 THEN 'Large cricket'
           WHEN adult_size_cm <= 13 THEN '1-2 large crickets'
           WHEN adult_size_cm <= 16 THEN '2-3 large crickets'
           ELSE '3-4 large crickets'
           END,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Every 10-14 days'
           WHEN adult_size_cm <= 13 THEN 'Every 14 days'
           WHEN adult_size_cm <= 16 THEN 'Every 14-21 days'
           ELSE 'Every 21-28 days'
           END,
       CASE
           WHEN adult_size_cm <= 8 THEN 'Medium crickets, small roaches'
           WHEN adult_size_cm <= 10 THEN 'Large crickets, medium roaches'
           WHEN adult_size_cm <= 13 THEN 'Large crickets, adult roaches'
           ELSE 'Multiple large crickets, adult roaches'
           END,
       'Adjust based on abdomen size'
FROM tarantula_species;

-- Specific adjustments for dwarf species
UPDATE feeding_schedules
SET prey_size = CASE
                    WHEN size_category = 'Adult' THEN 'Small-medium cricket'
                    ELSE prey_size
    END,
    notes     = notes || '; Careful not to overfeed'
WHERE species_id IN (SELECT id
                     FROM tarantula_species
                     WHERE adult_size_cm <= 8);

-- Adjustments for arboreal species 
UPDATE feeding_schedules
SET prey_type = CASE
                    WHEN size_category IN ('Sub-Adult', 'Adult') THEN prey_type || ', moths, flying insects'
                    ELSE prey_type
    END,
    notes     = notes || '; Prefers aerial prey catching; may refuse ground prey'
WHERE species_id IN (SELECT id
                     FROM tarantula_species
                     WHERE scientific_name IN (
                                               'Avicularia avicularia',
                                               'Caribena versicolor',
                                               'Psalmopoeus irminia',
                                               'Poecilotheria regalis',
                                               'Poecilotheria metallica',
                                               'Heteroscodra maculata'
                         ));

-- Adjustments for desert species 
UPDATE feeding_schedules
SET feeding_frequency = CASE
                            WHEN size_category = 'Adult' THEN 'Every 21-30 days'
                            WHEN size_category = 'Sub-Adult' THEN 'Every 14-21 days'
                            ELSE feeding_frequency
    END,
    notes             = notes || '; Adapted to irregular feeding; may fast during winter months'
WHERE species_id IN (SELECT id
                     FROM tarantula_species
                     WHERE scientific_name IN (
                                               'Aphonopelma chalcodes',
                                               'Grammostola rosea',
                                               'Aphonopelma hentzi',
                                               'Eupalaestrus campestratus'
                         ));

-- Special handling for aggressive/defensive species
UPDATE feeding_schedules
SET notes     = notes || '; Use long tongs for feeding; ensure retreat access',
    prey_size = CASE
                    WHEN size_category IN ('Sub-Adult', 'Adult') THEN
                        REPLACE(prey_size, 'large', 'medium')
                    ELSE prey_size
        END
WHERE species_id IN (SELECT id
                     FROM tarantula_species
                     WHERE temperament IN ('Defensive', 'Aggressive'));

-- Adjustments for heavy webbers
UPDATE feeding_schedules
SET notes             = notes || '; Heavy webber - place prey near web structure',
    feeding_frequency = CASE
                            WHEN size_category = 'Adult' THEN 'Every 14-21 days'
                            ELSE feeding_frequency
        END
WHERE species_id IN (SELECT id
                     FROM tarantula_species
                     WHERE scientific_name IN (
                                               'Chromatopelma cyaneopubescens',
                                               'Caribena versicolor',
                                               'Psalmopoeus irminia',
                                               'Neoholothele incei',
                                               'Chilobrachys fimbriatus'
                         ));

-- Create table for feeding frequency definitions
CREATE TABLE IF NOT EXISTS feeding_frequencies
(
    id             INTEGER PRIMARY KEY,
    frequency_name VARCHAR(50) NOT NULL UNIQUE,
    min_days       INTEGER     NOT NULL,
    max_days       INTEGER     NOT NULL,
    description    TEXT
);

UPDATE feeding_schedules
SET frequency_id = (SELECT id
                    FROM feeding_frequencies
                    WHERE frequency_name = feeding_frequency);