UPDATE tarantulas
SET acquisition_date =
        REPLACE(acquisition_date, '/', '-')
WHERE acquisition_date LIKE '%/%';


DROP TABLE IF EXISTS feeding_status_types;

DROP INDEX IF EXISTS idx_feeding_status_types;

CREATE TABLE feeding_events_new (
                                    id INTEGER PRIMARY KEY,
                                    tarantula_id INTEGER REFERENCES tarantulas(id),
                                    feeding_date TIMESTAMP NOT NULL,
                                    cricket_colony_id INTEGER REFERENCES cricket_colonies(id),
                                    number_of_crickets INTEGER NOT NULL,
                                    feeding_status_id INTEGER REFERENCES feeding_statuses(id),
                                    notes TEXT,
                                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                    user_id bigint
);

INSERT INTO feeding_events_new
SELECT * FROM feeding_events;

DROP TABLE feeding_events;
ALTER TABLE feeding_events_new RENAME TO feeding_events;

CREATE INDEX idx_feeding_events_tarantula ON feeding_events (tarantula_id);
CREATE INDEX idx_feeding_events_colony ON feeding_events (cricket_colony_id);
CREATE INDEX idx_feeding_events_date ON feeding_events (feeding_date);
CREATE INDEX idx_feeding_events_status ON feeding_events (feeding_status_id);
