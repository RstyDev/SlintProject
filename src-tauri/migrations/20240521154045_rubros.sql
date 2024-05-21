-- Add migration script here
CREATE TABLE IF NOT EXISTS rubros (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    descripcion TEXT NOT NULL,
    updated_at DATETIME NOT NULL
)

