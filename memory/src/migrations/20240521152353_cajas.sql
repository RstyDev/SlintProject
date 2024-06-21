-- Add migration script here
CREATE TABLE IF NOT EXISTS cajas (
            id integer PRIMARY KEY AUTOINCREMENT not null,
            inicio datetime not null,
            cierre datetime,
            monto_inicio real not null,
            monto_cierre real,
            ventas_totales real not null,
            cajero text
        )