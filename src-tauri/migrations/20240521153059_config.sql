-- Add migration script here
CREATE TABLE IF NOT EXISTS config (
            id integer PRIMARY KEY AUTOINCREMENT not null,
            politica real not null,
            formato string not null,
            mayus string not null,
            cantidad integer not null
        )