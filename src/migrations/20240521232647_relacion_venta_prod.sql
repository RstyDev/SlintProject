-- Add migration script here
CREATE TABLE IF NOT EXISTS relacion_venta_prod (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    venta INTEGER NOT NULL,
    producto INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    precio REAL NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (venta) REFERENCES ventas(id),
    FOREIGN KEY (producto) REFERENCES productos(id)
)