-- Add migration script here
CREATE TABLE IF NOT EXISTS producto (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    precio_venta REAL NOT NULL,
    porcentaje REAL NOT NULL,
    precio_costo REAL NOT NULL,
    marca TEXT NOT NULL,
    variedad TEXT NOT NULL,
    presentacion TEXT NOT NULL,
    cantidad INTEGER NOT NULL,
    updated_at DATETIME NOT NULL
)

