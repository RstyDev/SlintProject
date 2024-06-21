-- Add migration script here
CREATE TABLE IF NOT EXISTS deudas (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    cliente INTEGER NOT NULL,
    pago INTEGER NOT NULL,
    monto REAL NOT NULL,
    FOREIGN KEY (cliente) REFERENCES clientes(id)
)
