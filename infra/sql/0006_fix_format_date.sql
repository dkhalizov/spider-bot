UPDATE main.tarantulas
SET last_molt_date = date(last_molt_date)
WHERE last_molt_date IS NOT NULL;