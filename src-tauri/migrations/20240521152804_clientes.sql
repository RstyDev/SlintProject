-- Add migration script here
CREATE TABLE IF NOT EXISTS clientes (
            id integer PRIMARY KEY AUTOINCREMENT not null,
            nombre string,
            dni integer not null,
            limite real,
            activo boolean not null,
            time datetime not null
        )