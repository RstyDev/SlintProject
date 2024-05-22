use chrono::NaiveDateTime;
use sqlx::{query_as, Pool, Sqlite};


pub enum Model {
    MedioPago {
        id: i64,
        medio: String,
    },
    Caja {
        id: i64,
        inicio: NaiveDateTime,
        cierre: Option<NaiveDateTime>,
        monto_inicio: f64,
        monto_cierre: Option<f64>,
        ventas_totales: f64,
        cajero: Option<String>,
    },
    Cliente {
        id: i64,
        nombre: Option<String>,
        dni: i64,
        limite: Option<f64>,
        activo: bool,
        time: NaiveDateTime,
    },
    Config {
        id: i64,
        politica: f64,
        formato: String,
        mayus: String,
        cantidad: i64,
    },
    Prov {
        id: i64,
        nombre: String,
        contacto: Option<i64>,
        updated: NaiveDateTime,
    },
    Code {
        id: i64,
        codigo: i64,
        producto: Option<i64>,
        pesable: Option<i64>,
        rubro: Option<i64>,
    },
    Pesable {
        id: i64,
        precio_peso: f64,
        porcentaje: f64,
        costo_kilo: f64,
        descripcion: String,
        updated_at: NaiveDateTime,
    },
    Rubro {
        id: i64,
        descripcion: String,
        updated_at: NaiveDateTime,
    },
    Producto {
        id: i64,
        precio_venta: f64,
        porcentaje: f64,
        precio_costo: f64,
        marca: String,
        variedad: String,
        presentacion: String,
        cantidad: i64,
        updated_at: NaiveDateTime,
    },
    User {
        id: i64,
        user_id: String,
        nombre: String,
        pass: i64,
        rango: String,
    },
    Deuda {
        id: i64,
        cliente: i64,
        pago: i64,
        monto: f64,
    },
    Movimiento {
        id: i64,
        caja: i64,
        tipo: bool,
        monto: f64,
        descripcion: Option<String>,
        time: NaiveDateTime,
    },
    Pago {
        id: i64,
        medio_pago: i64,
        monto: f64,
        pagado: bool,
        venta: i64,
    },
    RelacionProdProv {
        id: i64,
        producto: i64,
        proveedor: i64,
        codigo: i64,
    },
    RelacionVentaPes {
        id: i64,
        venta: i64,
        pesable: i64,
        cantidad: f64,
        precio_kilo: f64,
    },
    RelacionVentaProd {
        id: i64,
        venta: i64,
        producto: i64,
        cantidad: i64,
        precio: f64,
    },
    RelacionVentaRub {
        id: i64,
        venta: i64,
        rubro: i64,
        cantidad: i64,
        precio: f64,
    },
    Venta {
        id:i64,
        time: NaiveDateTime,
        monto_total: f64,
        monto_pagado: f64,
        cliente: i64,
        cerrada: bool,
        paga: bool,
        pos: bool,
    },
}

// CREATE TABLE IF NOT EXISTS ventas (
//     id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
//     time DATETIME NOT NULL,
//     monto_total REAL NOT NULL,
//     monto_pagado REAL NOT NULL,
//     cliente INTEGER NOT NULL,
//     cerrada BOOLEAN NOT NULL,
//     paga BOOLEAN NOT NULL,
//     pos BOOLEAN NOT NULL,
//     FOREIGN KEY (cliente) REFERENCES clientes(id)
// )

async fn test(db: &Pool<Sqlite>){
    let res: sqlx::Result<Option<Model>> = query_as!(
        Model::Venta,
        "select * from ventas").fetch_optional(db).await;
    let res= res.unwrap().unwrap();
}