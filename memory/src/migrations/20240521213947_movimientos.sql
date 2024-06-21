-- Add migration script here
CREATE TABLE IF NOT EXISTS movimientos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    caja INTEGER NOT NULL,
    tipo BOOLEAN NOT NULL,
    monto REAL NOT NULL,
    descripcion TEXT,
    time DATETIME NOT NULL,
    FOREIGN KEY (caja) REFERENCES cajas(id)
)