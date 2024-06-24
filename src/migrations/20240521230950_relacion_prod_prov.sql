-- Add migration script here
CREATE TABLE IF NOT EXISTS relacion_prod_prov (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    producto INTEGER NOT NULL,
    proveedor INTEGER NOT NULL,
    codigo BIGINT,
    FOREIGN KEY (producto) REFERENCES productos(id),
    FOREIGN KEY (proveedor) REFERENCES proveedores(id)
)