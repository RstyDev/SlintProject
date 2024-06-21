-- Add migration script here
CREATE TABLE IF NOT EXISTS pesables (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    precio_peso real not null,
    porcentaje real not null,
    costo_kilo real not null,
    descripcion text not null,
    updated_at datetime not null
)
