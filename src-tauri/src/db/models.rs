use crate::mods::{
    AppError, Caja, Cli, Cliente, Config, MedioPago, Pesable, Rubro, User, Valuable,
};
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
            "select medio, monto from totales where caja = ? ",
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
                id: _,
                politica,
                formato,
                mayus,
                cantidad,
            } => {
                let medios: sqlx::Result<Vec<Model>> =
                    sqlx::query_as!(Model::MedioPago, "select * from medios_pago ")
                        .fetch_all(db)
                        .await;
                let medios = medios?
                    .iter()
                    .map(|model| match model {
                        Model::MedioPago { id: _, medio } => Arc::from(medio.to_owned()),
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
                    Model::BigInt,
                    "select codigo as int from codigos where producto = ? limit 5",
                    id
                )
                .fetch_all(db)
                .await;
                let codigos = models?
                    .iter()
                    .map(|model| match model {
                        Model::BigInt { int } => *int,
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
                    "select * from medios_pago where id = ? limit 1",
                    medio_pago
                )
                .fetch_optional(db)
                .await;
                let int_id = id;
                match medio? {
                    Some(med) => match med {
                        Model::MedioPago { id, medio } => Ok(Pago::build(
                            int_id,
                            MedioPago::build(medio.as_str(), id),
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
    pub async fn venta(db: &Pool<Sqlite>, model: Model, user: &Option<Arc<User>>) -> Res<Venta> {
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
                let mut productos = Vec::new();
                for model in qres {
                    match model {
                        Model::RelatedProd {
                            id,
                            precio,
                            porcentaje,
                            precio_costo,
                            tipo,
                            marca,
                            variedad,
                            presentacion,
                            size,
                            cantidad,
                        } => {
                            let qres: Vec<Model> = sqlx::query_as!(
                                Model::BigInt,
                                "select codigo as int from codigos where producto = ? limit 5",
                                id
                            )
                            .fetch_all(db)
                            .await?;
                            let codes = qres
                                .iter()
                                .map(|c| match c {
                                    Model::BigInt { int } => *int,
                                    _ => panic!("Se esperana codigo"),
                                })
                                .collect::<Vec<i64>>();
                            productos.push(Valuable::Prod((
                                cantidad as u8,
                                Producto::new(
                                    id,
                                    codes,
                                    precio as f32,
                                    porcentaje as f32,
                                    precio_costo as f32,
                                    tipo.as_str(),
                                    marca.as_str(),
                                    variedad.as_str(),
                                    Presentacion::build(presentacion.as_str(), size),
                                ),
                            )))
                        }
                        _ => {
                            return Err(AppError::IncorrectError(String::from(
                                "Se esperaba related prod",
                            )))
                        }
                    }
                }
                let qres:Vec<Model>=sqlx::query_as!(Model::RelatedPes,"select pesables.id as id,
                    precio_peso, porcentaje, costo_kilo, descripcion, cantidad, updated_at
                    from relacion_venta_pes inner join pesables on relacion_venta_pes.id = pesables.id where venta = ?
                     ",id).fetch_all(db).await?;
                for model in qres {
                    match model {
                        Model::RelatedPes {
                            id,
                            precio_peso,
                            porcentaje,
                            costo_kilo,
                            descripcion,
                            updated_at: _,
                            cantidad,
                        } => {
                            let qres: Option<Model> = sqlx::query_as!(
                                Model::BigInt,
                                "select codigo as int from codigos where pesable = ? limit 1",
                                id
                            )
                            .fetch_optional(db)
                            .await?;
                            match qres {
                                Some(model) => match model {
                                    Model::BigInt { int } => productos.push(Valuable::Pes((
                                        cantidad as f32,
                                        Pesable::build(
                                            id,
                                            int,
                                            precio_peso as f32,
                                            porcentaje as f32,
                                            costo_kilo as f32,
                                            descripcion.as_str(),
                                        ),
                                    ))),
                                    _ => {
                                        return Err(AppError::IncorrectError(String::from(
                                            "se esperaba codigo",
                                        )))
                                    }
                                },
                                None => {
                                    return Err(AppError::IncorrectError(String::from(
                                        "No se encontro codigo de pesable",
                                    )))
                                }
                            }
                        }
                        _ => {
                            return Err(AppError::IncorrectError(String::from(
                                "se esperaba RelatedPes",
                            )))
                        }
                    }
                }
                let qres:Vec<Model>=sqlx::query_as!(Model::RelatedRub,"select rubros.id as id, descripcion, updated_at, cantidad, precio
                    from relacion_venta_rub inner join rubros on relacion_venta_rub.id = rubros.id where venta = ?
                     ",id).fetch_all(db).await?;
                for model in qres {
                    match model {
                        Model::RelatedRub {
                            id,
                            descripcion,
                            updated_at: _,
                            cantidad,
                            precio,
                        } => {
                            let qres: Option<Model> = sqlx::query_as!(
                                Model::BigInt,
                                "select codigo as int from codigos where pesable = ? limit 1",
                                id
                            )
                            .fetch_optional(db)
                            .await?;
                            match qres {
                                Some(model) => match model {
                                    Model::BigInt { int } => productos.push(Valuable::Rub((
                                        cantidad as u8,
                                        Rubro::build(
                                            id,
                                            int,
                                            Some(precio as f32),
                                            Arc::from(descripcion.as_str()),
                                        ),
                                    ))),
                                    _ => {
                                        return Err(AppError::IncorrectError(String::from(
                                            "se esperaba codigo",
                                        )))
                                    }
                                },
                                None => {
                                    return Err(AppError::IncorrectError(String::from(
                                        "No se encontro codigo de pesable",
                                    )))
                                }
                            }
                        }
                        _ => {
                            return Err(AppError::IncorrectError(String::from(
                                "se esperaba RelatedPes",
                            )))
                        }
                    }
                }
                let qres: Vec<Model> =
                    sqlx::query_as!(Model::Pago, "select * from pagos where venta = ? ", id)
                        .fetch_all(db)
                        .await?;
                let mut pagos = Vec::new();
                for pago in qres {
                    match pago {
                        Model::Pago {
                            id,
                            medio_pago,
                            monto,
                            pagado,
                            venta: _,
                        } => {
                            let qres: Option<Model> = sqlx::query_as!(
                                Model::MedioPago,
                                "select * from medios_pago where id = ? limit 1",
                                medio_pago
                            )
                            .fetch_optional(db)
                            .await?;
                            let medio = match qres {
                                Some(model) => match model {
                                    Model::MedioPago { id, medio } => {
                                        MedioPago::build(medio.as_str(), id)
                                    }
                                    _ => {
                                        return Err(AppError::IncorrectError(String::from(
                                            "se esperaba Medio Pago",
                                        )))
                                    }
                                },
                                None => {
                                    return Err(AppError::IncorrectError(String::from(
                                        "no es encontro medio_pago de pago",
                                    )))
                                }
                            };
                            pagos.push(Pago::build(id, medio, monto as f32, pagado as f32))
                        }
                        _ => {
                            return Err(AppError::IncorrectError(String::from("se esperaba pago")))
                        }
                    }
                }
                let qres: Option<Model> = sqlx::query_as!(
                    Model::Cliente,
                    "select * from clientes where id = ? limit 1",
                    cliente
                )
                .fetch_optional(db)
                .await?;
                let cliente = match qres {
                    Some(model) => match model {
                        Model::Cliente {
                            id,
                            nombre,
                            dni,
                            limite,
                            activo,
                            time,
                        } => Cliente::Regular(Cli::build(
                            id,
                            Arc::from(nombre.as_str()),
                            dni as i32,
                            activo,
                            time,
                            limite.map(|l| l as f32),
                        )),
                        _ => {
                            return Err(AppError::IncorrectError(String::from(
                                "Se esperaba cliente",
                            )))
                        }
                    },
                    None => Cliente::Final,
                };
                Ok(Venta::build(
                    id,
                    monto_total as f32,
                    productos,
                    pagos,
                    monto_pagado as f32,
                    user.clone(),
                    cliente,
                    paga,
                    cerrada,
                    time,
                ))
            }
            _ => Err(AppError::IncorrectError("Se esperaba Venta".to_string())),
        }
    }
}
pub enum Model {
    BigInt {
        int: i64,
    },
    Int {
        int: i32,
    },
    Double {
        double: f64,
    },
    Float {
        float: f32,
    },
    Bool {
        val: bool,
    },
    String {
        string: &str,
    },
    MedioPago {
        id: i64,
        medio: &str,
    },
    CajaParcial {
        id: i64,
        cierre: Option<NaiveDateTime>,
        ventas_totales: f32,
        cajero: Option<&str>,
    },
    Caja {
        id: i64,
        inicio: NaiveDateTime,
        cierre: Option<NaiveDateTime>,
        monto_inicio: f32,
        monto_cierre: Option<f32>,
        ventas_totales: f32,
        cajero: Option<&str>,
    },
    Cliente {
        id: i64,
        nombre: &str,
        dni: i32,
        limite: Option<f32>,
        activo: bool,
        time: NaiveDateTime,
    },
    Config {
        id: i64,
        politica: f32,
        formato: &str,
        mayus: &str,
        cantidad: u8,
    },
    Prov {
        id: i64,
        nombre: &str,
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
        precio_peso: f32,
        porcentaje: f32,
        costo_kilo: f32,
        descripcion: &str,
        updated_at: NaiveDateTime,
    },
    RelatedPes {
        id: i64,
        precio_peso: f32,
        porcentaje: f32,
        costo_kilo: f32,
        descripcion: &str,
        updated_at: NaiveDateTime,
        cantidad: f32,
    },
    Rubro {
        id: i64,
        descripcion: &str,
        updated_at: NaiveDateTime,
    },
    RelatedRub {
        id: i64,
        descripcion: &str,
        updated_at: NaiveDateTime,
        cantidad: u8,
        precio: f32,
    },
    Producto {
        id: i64,
        precio_venta: f32,
        porcentaje: f32,
        precio_costo: f32,
        tipo: &str,
        marca: &str,
        variedad: &str,
        presentacion: &str,
        size: f32,
        updated_at: NaiveDateTime,
    },
    RelatedProd {
        id: i64,
        precio: f32,
        porcentaje: f32,
        precio_costo: f32,
        tipo: &str,
        marca: &str,
        variedad: &str,
        presentacion: &str,
        size: f32,
        cantidad: u8,
    },
    User {
        id: i64,
        user_id: &str,
        nombre: &str,
        pass: i64,
        rango: &str,
    },
    Deuda {
        id: i64,
        cliente: i64,
        pago: i64,
        monto: f32,
    },
    Movimiento {
        id: i64,
        caja: i64,
        tipo: bool,
        monto: f32,
        descripcion: Option<&str>,
        time: NaiveDateTime,
    },
    Pago {
        id: i64,
        medio_pago: i64,
        monto: f32,
        pagado: f32,
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
        cantidad: f32,
        precio_kilo: f32,
    },
    RelacionVentaProd {
        id: i64,
        venta: i64,
        producto: i64,
        cantidad: u8,
        precio: f32,
    },
    RelacionVentaRub {
        id: i64,
        venta: i64,
        rubro: i64,
        cantidad: u8,
        precio: f32,
    },
    Venta {
        id: i64,
        time: NaiveDateTime,
        monto_total: f32,
        monto_pagado: f32,
        cliente: Option<i64>,
        cerrada: bool,
        paga: bool,
        pos: bool,
    },
    Total {
        medio: &str,
        monto: f32,
    },
}

// async fn test(db: &Pool<Sqlite>){
//     let res: sqlx::Result<Option<Model>> = query_as!(
//         Model::Venta,
//         "select * from ventas").fetch_optional(db).await;
//     let res= res.unwrap().unwrap();
// }
