
INSERT INTO tarantula_species (id, scientific_name, common_name, adult_size_cm, temperament,
                               humidity_requirement_percent, temperature_requirement_celsius)
VALUES (1, 'Brachypelma hamorii', 'Mexican Red Knee', 14.0, 'Docile', 65, 24.0);

INSERT INTO tarantulas (id,
                        name,
                        species_id,
                        acquisition_date,
                        estimated_age_months,
                        current_molt_stage_id,
                        current_health_status_id,
                        enclosure_number)
VALUES (1,
        'Marpha',
        1,
        '2024-11-10',
        12,
        1,
        1,
        'REPTO-01');

INSERT INTO cricket_colonies (id,
                              colony_name,
                              size_type_id,
                              current_count,
                              last_count_date,
                              container_number,
                              notes)
VALUES (1,
        'Primary Colony',
        3,
        10,
        date ('now'),
        'CRICKET-01',
        'Основная колония сверчков для B. hamorii');

INSERT INTO feeding_events (id,
                            tarantula_id,
                            feeding_date,
                            cricket_colony_id,
                            number_of_crickets,
                            feeding_status_id,
                            notes)
VALUES (1,
        1,
        datetime('now', '-4 days'),
        1, -- если нет колонии сверчков
        1,
        1, -- Accepted
        'Нормальное кормление, паук активен');

-- Добавляем запись о проверке здоровья
INSERT INTO health_check_records (id,
                                  tarantula_id,
                                  check_date,
                                  health_status_id,
                                  temperature_celsius,
                                  notes)
VALUES (1,
        1,
        date ('now'),
        1, -- Healthy
        19.0,
        'Паук активен, держится возле обогреваемой стенки. Обогрев 5W работает постоянно.');
