-- Add migration script here
CREATE TABLE IF NOT EXISTS codigo (
                id integer PRIMARY KEY AUTOINCREMENT not null,
                codigo bigint not null,
                producto integer,
                foreign key (producto) references producto(id),
                pesable integer,
                foreign key (pesable) references pesable(id),
                rubro integer,
                foreign key (rubro) references rubro(id)
            )