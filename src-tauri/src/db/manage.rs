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
    db.execute(sqlx::query("drop table if exists cajas"))
        .await
        .unwrap();
    db.execute(sqlx::query("drop table if exists clientes"))
        .await
        .unwrap();
    db.execute(sqlx::query("drop table if exists config"))
        .await
        .unwrap();
    db.execute(sqlx::query("drop table if exists medio_pago"))
        .await
        .unwrap();
    db.execute(sqlx::query("drop table if exists proveedor"))
        .await
        .unwrap();
}



enum Deuda {
    Table,
    Id,
    Cliente,
    Pago,
    Monto,
}

enum Movimiento {
    Table,
    Id,
    Caja,
    Tipo,
    Monto,
    Descripcion,
    Time,
}

pub enum Pago {
    Table,
    Id,
    MedioPago,
    Monto,
    Pagado,
    Venta,
}





enum RelacionProdProv {
    Table,
    Id,
    Producto,
    Proveedor,
    Codigo,
}

enum RelacionVentaPes {
    Table,
    Id,
    Cantidad,
    Precio,
    Pesable,
    Venta,
}

enum RelacionVentaProd {
    Table,
    Id,
    Cantidad,
    Precio,
    Producto,
    Venta,
}

enum RelacionVentaRub {
    Table,
    Id,
    Cantidad,
    Rubro,
    Precio,
    Venta,
}




pub enum Venta {
    Table,
    Id,
    Time,
    MontoTotal,
    MontoPagado,
    Cliente,
    Cerrada,
    Paga,
    Pos,
}

