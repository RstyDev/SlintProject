-- Add migration script here
CREATE TABLE IF NOT EXISTS ventas (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    time DATETIME NOT NULL,
    monto_total REAL NOT NULL,
    monto_pagado REAL NOT NULL,
    cliente INTEGER,
    cerrada BOOLEAN NOT NULL,
    paga BOOLEAN NOT NULL,
    pos BOOLEAN NOT NULL,
    FOREIGN KEY (cliente) REFERENCES clientes(id)
)