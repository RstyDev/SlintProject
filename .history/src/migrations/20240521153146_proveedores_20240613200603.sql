-- Add migration script here
CREATE TABLE IF NOT EXISTS proveedores (
            id integer PRIMARY KEY AUTOINCREMENT not null,
            nombre string not null,
            contacto bigint,
            updated datetime
        )