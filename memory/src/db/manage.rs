use dotenvy::dotenv;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};
use std::{env, str::FromStr};

use sqlx::{Executor, Pool, Sqlite};
pub async fn db(write: bool) -> Pool<Sqlite> {
    println!("{:#?}", dotenv().unwrap());
    println!("{:#?}", env::current_dir().unwrap().display());
    dotenv().unwrap();
    let dir = match write {
        true => std::env::var("WRITE_URL").unwrap(),
        false => std::env::var("READ_URL").unwrap(),
    };
    let conn = SqliteConnectOptions::from_str(dir.as_ref())
        .unwrap()
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true);

    let db = SqlitePool::connect(dir.as_ref())
        .await
        .expect("Error conectando a la DB");
    db.set_connect_options(conn);

    db
}

pub async fn fresh(db: &Pool<Sqlite>) {
    down(db).await;
    sqlx::query(QUERY).execute(db).await.unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("migrations");
    println!("{:#?}", migrations);
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    println!("migration: {:?}", migration_results);
}

pub async fn down(db: &Pool<Sqlite>) {
    dotenv().ok();
    db.execute(sqlx::query(
        "
    drop table if exists medios_pago;
    drop table if exists cajas;
    drop table if exists clientes;
    drop table if exists config;
    drop table if exists proveedores;
    drop table if exists codigos;
    drop table if exists pesables;
    drop table if exists rubros;
    drop table if exists productos;
    drop table if exists users;
    drop table if exists deudas;
    drop table if exists movimientos;
    drop table if exists pagos;
    drop table if exists relacion_prod_prov;
    drop table if exists relacion_venta_pes;
    drop table if exists relacion_venta_prod;
    drop table if exists relacion_venta_rub;
    ",
    ))
    .await
    .unwrap();
}

const QUERY: &str = "
CREATE TABLE IF NOT EXISTS medios_pago (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    medio TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS cajas (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    inicio datetime not null,
    cierre datetime,
    monto_inicio real not null,
    monto_cierre real,
    ventas_totales real not null,
    cajero text
);
CREATE TABLE IF NOT EXISTS clientes (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    nombre TEXT not null,
    dni integer not null,
    limite real,
    activo boolean not null,
    time datetime not null
);
CREATE TABLE IF NOT EXISTS config (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    politica real not null,
    formato TEXT not null,
    mayus TEXT not null,
    cantidad integer not null
);
CREATE TABLE IF NOT EXISTS proveedores (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    nombre TEXT not null,
    contacto bigint,
    updated datetime not null
);
CREATE TABLE IF NOT EXISTS codigos (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    codigo bigint not null,
    producto integer,
    pesable integer,
    rubro integer,
    foreign key (producto) references productos(id),
    foreign key (pesable) references pesables(id),
    foreign key (rubro) references rubros(id)
);
CREATE TABLE IF NOT EXISTS pesables (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    precio_peso real not null,
    porcentaje real not null,
    costo_kilo real not null,
    descripcion text not null,
    updated_at datetime not null
);
CREATE TABLE IF NOT EXISTS rubros (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    descripcion TEXT NOT NULL,
    updated_at DATETIME NOT NULL
);
CREATE TABLE IF NOT EXISTS productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    precio_venta REAL NOT NULL,
    porcentaje REAL NOT NULL,
    precio_costo REAL NOT NULL,
    tipo TEXT NOT NULL,
    marca TEXT NOT NULL,
    variedad TEXT NOT NULL,
    presentacion TEXT NOT NULL,
    size REAL NOT NULL,
    updated_at DATETIME NOT NULL
);
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id TEXT NOT NULL,
    nombre TEXT NOT NULL,
    pass BIGINT NOT NULL,
    rango TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS deudas (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    cliente INTEGER NOT NULL,
    pago INTEGER NOT NULL,
    monto REAL NOT NULL,
    FOREIGN KEY (cliente) REFERENCES clientes(id)
);
CREATE TABLE IF NOT EXISTS movimientos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    caja INTEGER NOT NULL,
    tipo BOOLEAN NOT NULL,
    monto REAL NOT NULL,
    descripcion TEXT,
    time DATETIME NOT NULL,
    FOREIGN KEY (caja) REFERENCES cajas(id)
);
CREATE TABLE IF NOT EXISTS pagos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    medio_pago INTEGER NOT NULL,
    monto REAL NOT NULL,
    pagado REAL NOT NULL,
    venta INTEGER NOT NULL,
    FOREIGN KEY (medio_pago) REFERENCES medios_pago(id),
    FOREIGN KEY (venta) REFERENCES ventas(id)
);
CREATE TABLE IF NOT EXISTS relacion_prod_prov (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    producto INTEGER NOT NULL,
    proveedor INTEGER NOT NULL,
    codigo BIGINT NOT NULL,
    FOREIGN KEY (producto) REFERENCES productos(id),
    FOREIGN KEY (proveedor) REFERENCES proveedores(id)
);
CREATE TABLE IF NOT EXISTS relacion_venta_pes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    venta INTEGER NOT NULL,
    pesable INTEGER NOT NULL,
    cantidad REAL NOT NULL,
    precio_kilo REAL NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (venta) REFERENCES ventas(id),
    FOREIGN KEY (pesable) REFERENCES pesables(id)
);
CREATE TABLE IF NOT EXISTS relacion_venta_prod (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    venta INTEGER NOT NULL,
    producto INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    precio REAL NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (venta) REFERENCES ventas(id),
    FOREIGN KEY (producto) REFERENCES productos(id)
);
CREATE TABLE IF NOT EXISTS relacion_venta_rub (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    venta INTEGER NOT NULL,
    rubro INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    precio REAL NOT NULL,
    pos INTEGER NOT NULL,
    FOREIGN KEY (venta) REFERENCES ventas(id),
    FOREIGN KEY (rubro) REFERENCES rubros(id)
);
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
);
CREATE TABLE IF NOT EXISTS totales (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    caja INTEGER NOT NULL,
    medio TEXT NOT NULL,
    monto REAL NOT NULL,
    FOREIGN KEY (caja) REFERENCES cajas(id)
);
 ";
