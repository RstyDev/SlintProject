-- Add migration script here
CREATE TABLE IF NOT EXISTS medios_pago (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    medio TEXT NOT NULL
)