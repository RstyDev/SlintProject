-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    user_id TEXT NOT NULL,
    nombre TEXT NOT NULL,
    pass BIGINT NOT NULL,
    rango TEXT NOT NULL,
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL
)

