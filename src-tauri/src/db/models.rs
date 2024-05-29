use crate::mods::{AppError, Caja, Config, MedioPago, Valuable};
use crate::mods::{Pago, Presentacion, Producto, Res, Venta};
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
                    totales.insert(Arc::from(medio), monto as f32);
                }
                _ => return Err(AppError::IncorrectError("Imposible".to_string())),
            }
        }
        Ok(Caja::build(
            id,
            inicio,
            cierre,
            ventas_totales as f32,
            monto_inicio as f32,
            monto_cierre.map(|m| m as f32),
            cajero.map(|c| Arc::from(c.as_str())),
            totales,
        ))
    }
    pub async fn config(db: &Pool<Sqlite>, model: Model) -> Res<Config> {
        match model {
            Model::Config {
                id:_,
                politica,
                formato,
                mayus,
                cantidad,
            } => {
                let medios: sqlx::Result<Vec<Model>> =
                    sqlx::query_as!(Model::MedioPago, "select * from medios_pago")
                        .fetch_all(db)
                        .await;
                let medios = medios?
                    .iter()
                    .map(|model| match model {
                        Model::MedioPago { id:_, medio } => Arc::from(medio.to_owned()),
                        _ => panic!("Se esperaba MedioPago"),
                    })
                    .collect::<Vec<Arc<str>>>();
                Ok(Config::build(
                    politica as f32,
                    formato.as_str(),
                    mayus.as_str(),
                    cantidad as u8,
                    medios,
                ))
            }
            _ => Err(AppError::IncorrectError(String::from("Se esperaba config"))),
        }
    }
    pub async fn producto(db: &Pool<Sqlite>, model: Model) -> Res<Producto> {
        match model {
            Model::Producto {
                id,
                precio_venta,
                porcentaje,
                precio_costo,
                tipo,
                marca,
                variedad,
                presentacion,
                size,
                updated_at,
            } => {
                let models: sqlx::Result<Vec<Model>> = sqlx::query_as!(
                    Model::Codigo,
                    "select codigo from codigos where producto = ?",
                    id
                )
                .fetch_all(db)
                .await;
                let codigos = models?
                    .iter()
                    .map(|model| match model {
                        Model::Codigo { codigo } => *codigo,
                        _ => panic!("Se esperaba codigo"),
                    })
                    .collect::<Vec<i64>>();
                let presentacion = match presentacion.as_str() {
                    "Gr" => Presentacion::Gr(size as f32),
                    "Un" => Presentacion::Un(size as u16),
                    "Lt" => Presentacion::Lt(size as f32),
                    "Ml" => Presentacion::Ml(size as u16),
                    "CC" => Presentacion::CC(size as u16),
                    "Kg" => Presentacion::Kg(size as f32),
                    a => return Err(AppError::SizeSelection(a.to_string())),
                };
                Ok(Producto::new(
                    id,
                    codigos,
                    precio_venta as f32,
                    porcentaje as f32,
                    precio_costo as f32,
                    tipo.as_str(),
                    marca.as_str(),
                    variedad.as_str(),
                    presentacion,
                ))
            }
            _ => Err(AppError::IncorrectError("Se esperaba Producto".to_string())),
        }
    }
    pub async fn pago(db: &Pool<Sqlite>, model: Model) -> Res<Pago> {
        match model {
            Model::Pago {
                id,
                medio_pago,
                monto,
                pagado,
                venta: _,
            } => {
                let medio: sqlx::Result<Option<Model>> = sqlx::query_as!(
                    Model::MedioPago,
                    "select * from medios_pago where id = ?",
                    medio_pago
                )
                .fetch_optional(db)
                .await;
                let int_id = id;
                match medio? {
                    Some(med) => match med {
                        Model::MedioPago { id, medio } => Ok(Pago::build(
                            int_id,
                            MedioPago::new(medio.as_str(), id),
                            monto as f32,
                            pagado as f32,
                        )),
                        _ => Err(AppError::IncorrectError(String::from(
                            "se esperaba MedioPago",
                        ))),
                    },
                    None => Err(AppError::IncorrectError(String::from(
                        "No se encontro el medio pago correspondiente",
                    ))),
                }
            }
            _ => Err(AppError::IncorrectError("Se esperaba Pago".to_string())),
        }
    }
    pub async fn venta(db: &Pool<Sqlite>, model: Model) -> Res<Venta> {
        match model {
            Model::Venta {
                id,
                time,
                monto_total,
                monto_pagado,
                cliente,
                cerrada,
                paga,
                pos,
            } => {
                let qres:Vec<Model>=sqlx::query_as!(Model::RelatedProd,"select productos.id as id,
                    precio, porcentaje, precio_costo, tipo, marca, variedad, presentacion, size, cantidad
                    from relacion_venta_prod inner join productos on relacion_venta_prod.id = productos.id where venta = ?
                    ",id).fetch_all(db).await?;
                let mut productos=Vec::new();
                for model in qres{
                    match model{
                        Model::RelatedProd { id, precio, porcentaje, precio_costo, tipo, marca, variedad, presentacion, size, cantidad }        =>{
                            let qres:Vec<Model>=sqlx::query_as!(Model::Codigo,"select codigo from codigos where producto = ?",id).fetch_all(db).await?;
                            let codes = qres.iter().map(|c|match c{
                                Model::Codigo { codigo }=>*codigo,
                                _=>panic!("Se esperana codigo")
                            }).collect::<Vec<i64>>();
                            productos.push(Valuable::Prod((cantidad as u8,Producto::new(id, codes, precio as f32, porcentaje as f32, precio_costo as f32, tipo.as_str(), marca.as_str(), variedad.as_str(), Presentacion::build(presentacion.as_str(),size)))))
                        },
                        _=>return Err(AppError::IncorrectError(String::from("Se esperaba related prod")))
                    }
                }
                
            }
            _ => panic!("se esperaba Venta"),
        }
        Err(AppError::IncorrectError("asfd".to_string()))
    }
}
pub enum Model {
    Id {
        id: i64,
    },
    Monto {
        monto: f64,
    },
    Codigo {
        codigo: i64,
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
        nombre: String,
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
        tipo: String,
        marca: String,
        variedad: String,
        presentacion: String,
        size: f64,
        updated_at: NaiveDateTime,
    },
    RelatedProd {
        id: i64,
        precio: f64,
        porcentaje: f64,
        precio_costo: f64,
        tipo: String,
        marca: String,
        variedad: String,
        presentacion: String,
        size: f64,
        cantidad: i64,
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
        pagado: f64,
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

async fn test(db: &Pool<Sqlite>){
    let res: sqlx::Result<Option<Model>> = query_as!(
        Model::Venta,
        "select * from ventas").fetch_optional(db).await;
    let res= res.unwrap().unwrap();
}
