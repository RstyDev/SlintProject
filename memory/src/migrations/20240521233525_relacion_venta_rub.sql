-- Add migration script here
CREATE TABLE IF NOT EXISTS relacion_venta_rub (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    venta INTEGER NOT NULL,
    rubro INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    precio REAL NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (venta) REFERENCES ventas(id),
    FOREIGN KEY (rubro) REFERENCES rubros(id)
)