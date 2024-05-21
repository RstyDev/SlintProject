use dotenvy::dotenv;

use sqlx::{Executor, Pool, Sqlite};
use tauri::async_runtime::{block_on, spawn};

pub async fn fresh(db: &Pool<Sqlite>) {
    down(db).await;
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
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
    db.execute(sqlx::query("
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
    ")).await.unwrap();
}








