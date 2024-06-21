-- Add migration script here
CREATE TABLE IF NOT EXISTS codigos (
                id integer PRIMARY KEY AUTOINCREMENT not null,
                codigo bigint not null,
                producto integer,
                pesable integer,
                rubro integer,
                foreign key (producto) references productos(id),
                foreign key (pesable) references pesables(id),
                foreign key (rubro) references rubros(id)
            )