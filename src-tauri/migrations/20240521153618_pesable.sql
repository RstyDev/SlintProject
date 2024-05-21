-- Add migration script here
CREATE TABLE IF NOT EXISTS pesable
(
    id integer PRIMARY AUTOINCREMENT not null,
    precio_peso real not null,
    porcentaje real not null,
    costo_kilo real not null,
    descripcion text not null,
    updated_at datetime not null
)
