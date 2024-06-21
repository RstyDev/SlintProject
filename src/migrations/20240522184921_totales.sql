-- Add migration script here
CREATE TABLE IF NOT EXISTS totales (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    caja INTEGER NOT NULL,
    medio TEXT NOT NULL,
    monto REAL NOT NULL,
    FOREIGN KEY (caja) REFERENCES cajas(id)
)