use crate::mods::Res;
use crate::mods::{AppError, Caja, Config, MedioPago};
use chrono::NaiveDateTime;
use sqlx::{query_as, Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Mapper;
impl Mapper {
    pub async fn caja(db: &Pool<Sqlite>, model_caja: Model) -> Res<Caja> {
        let (id, inicio, cierre, monto_inicio, monto_cierre, ventas_totales, cajero) =
            match model_caja {
                Model::Caja {
                    id,
                    inicio,
                    cierre,
                    monto_inicio,
                    monto_cierre,
                    ventas_totales,
                    cajero,
                } => (
                    id,
                    inicio,
                    cierre,
                    monto_inicio,
                    monto_cierre,
                    ventas_totales,
                    cajero,
                ),
                _ => return Err(AppError::IncorrectError("Imposible".to_string())),
            };
        let totales_mod: sqlx::Result<Vec<Model>> = query_as!(
            Model::Total,
            "select medio, monto from totales where caja = ?",
            id
        )
        .fetch_all(db)
        .await;
        let mut totales = HashMap::new();
        for tot in totales_mod? {
            match tot {
                Model::Total { medio, monto } => {
                    totales.insert(Arc::from(medio), monto);
                }
                _ => return Err(AppError::IncorrectError("Imposible".to_string())),
            }
        }
        Ok(Caja::build(
            id,
            inicio,
            cierre,
            ventas_totales,
            monto_inicio,
            monto_cierre,
            cajero.map(|c| Arc::from(c.as_str())),
            totales,
        ))
    }
    pub async fn config(db: &Pool<Sqlite>, model: Model) -> Res<Config> {
        match model {
            Model::Config {
                id,
                politica,
                formato,
                mayus,
                cantidad,
            } => {
                let medios: sqlx::Result<Vec<Model>> =
                    sqlx::query_as!(Model::MedioPago, "select * from medios_pago")
                        .fetch_all(db)
                        .await;
                let medios= medios?.iter().map(|model| match model {
                    Model::MedioPago { id, medio } => Arc::from(medio.to_owned()),
                    _ => panic!("Se esperaba MedioPago"),
                }).collect::<Vec<Arc<str>>>();
                Ok(Config::build(politica, formato, mayus, cantidad, medios))
            }
            _ => Err(AppError::IncorrectError(String::from("Se esperaba config"))),
        }
    }
}
pub enum Model {
    Id {
        id: i64,
    },
    Bool {
        val: bool,
    },
    MedioPago {
        id: i64,
        medio: String,
    },
    CajaParcial {
        id: i64,
        cierre: Option<NaiveDateTime>,
        ventas_totales: f64,
        cajero: Option<String>,
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
        id: i64,
        time: NaiveDateTime,
        monto_total: f64,
        monto_pagado: f64,
        cliente: i64,
        cerrada: bool,
        paga: bool,
        pos: bool,
    },
    Total {
        medio: String,
        monto: f64,
    },
}

// async fn test(db: &Pool<Sqlite>){
//     let res: sqlx::Result<Option<Model>> = query_as!(
//         Model::Venta,
//         "select * from ventas").fetch_optional(db).await;
//     let res= res.unwrap().unwrap();
// }
