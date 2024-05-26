-- Add migration script here
CREATE TABLE IF NOT EXISTS pagos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    medio_pago INTEGER NOT NULL,
    monto REAL NOT NULL,
    pagado REAL NOT NULL,
    venta INTEGER NOT NULL,
    FOREIGN KEY (medio_pago) REFERENCES medios_pago(id),
    FOREIGN KEY (venta) REFERENCES ventas(id)
)