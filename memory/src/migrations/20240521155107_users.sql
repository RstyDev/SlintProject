-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id TEXT NOT NULL,
    nombre TEXT NOT NULL,
    pass BIGINT NOT NULL,
    rango TEXT NOT NULL
)

