-- Add migration script here
CREATE TABLE IF NOT EXISTS relacion_venta_pes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    venta INTEGER NOT NULL,
    pesable INTEGER NOT NULL,
    cantidad REAL NOT NULL,
    precio_kilo REAL NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (venta) REFERENCES ventas(id),
    FOREIGN KEY (pesable) REFERENCES pesables(id)
)