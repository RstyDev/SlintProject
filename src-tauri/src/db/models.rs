// use crate::db::map::{
//     BigIntDB, CajaDB, ClienteDB, ConfigDB, MedioPagoDB, PagoDB, ProductoDB, RelatedPesDB,
//     RelatedProdDB, RelatedRubDB, TotalDB, VentaDB,
// };
// use crate::mods::{
//     AppError, Caja, Cli, Cliente, Config, MedioPago, Pesable, Rubro, User, Valuable,
// };
// use crate::mods::{Pago, Presentacion, Producto, Res, Venta};
use sqlx::{query_as, FromRow, Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;

// #[derive(Clone)]
// pub struct Mapper;
// impl Mapper {
//     pub async fn caja(db: &Pool<Sqlite>, caja: CajaDB) -> Res<Caja> {
//         let totales_mod: sqlx::Result<Vec<TotalDB>> = query_as!(
//             TotalDB,
//             "select medio, monto from totales where caja = ? ",
//             caja.id
//         )
//         .fetch_all(db)
//         .await;
//         let mut totales = HashMap::new();
//         for tot in totales_mod? {
//             totales.insert(Arc::from(tot.medio), tot.monto);
//         }
//         Ok(Caja::build(
//             caja.id,
//             caja.inicio,
//             caja.cierre,
//             caja.ventas_totales,
//             caja.monto_inicio,
//             caja.monto_cierre,
//             caja.cajero.map(|c| Arc::from(c)),
//             totales,
//         ))
//     }
//     pub async fn config(db: &Pool<Sqlite>, config: ConfigDB) -> Res<Config> {
//         let medios: sqlx::Result<Vec<MedioPagoDB>> =
//             sqlx::query_as!(MedioPagoDB, "select * from medios_pago ")
//                 .fetch_all(db)
//                 .await;
//         let medios = medios?
//             .iter()
//             .map(|model| Arc::from(model.medio.to_owned()))
//             .collect::<Vec<Arc<str>>>();
//         Ok(Config::build(
//             config.politica,
//             config.formato.as_str(),
//             config.mayus.as_str(),
//             config.cantidad,
//             medios,
//         ))
//     }
//     pub async fn producto(db: &Pool<Sqlite>, prod: ProductoDB) -> Res<Producto> {
//         let models: sqlx::Result<Vec<BigIntDB>> = sqlx::query_as!(
//             BigIntDB,
//             "select codigo as int from codigos where producto = ? limit 5",
//             prod.id
//         )
//         .fetch_all(db)
//         .await;
//         let codigos = models?.iter().map(|model| *model.int).collect::<Vec<i64>>();
//         let presentacion = match prod.presentacion.as_str() {
//             "Gr" => Presentacion::Gr(prod.size),
//             "Un" => Presentacion::Un(prod.size as u16),
//             "Lt" => Presentacion::Lt(prod.size),
//             "Ml" => Presentacion::Ml(prod.size as u16),
//             "CC" => Presentacion::CC(prod.size as u16),
//             "Kg" => Presentacion::Kg(prod.size),
//             a => return Err(AppError::SizeSelection(a.to_string())),
//         };
//         Ok(Producto::new(
//             prod.id,
//             codigos,
//             prod.precio_venta,
//             prod.porcentaje,
//             prod.precio_costo,
//             prod.tipo.as_str(),
//             prod.marca.as_str(),
//             prod.variedad.as_str(),
//             presentacion,
//         ))
//     }
//     pub async fn pago(db: &Pool<Sqlite>, pago: PagoDB) -> Res<Pago> {
//         let medio: sqlx::Result<Option<MedioPagoDB>> = sqlx::query_as!(
//             MedioPagoDB,
//             "select * from medios_pago where id = ? limit 1",
//             pago.medio_pago
//         )
//         .fetch_optional(db)
//         .await;
//         let int_id = pago.id;
//         match medio? {
//             Some(med) => Ok(Pago::build(
//                 int_id,
//                 MedioPago::build(medio.as_str(), med.id),
//                 pago.monto,
//                 pago.pagado,
//             )),
//             None => Err(AppError::IncorrectError(String::from(
//                 "No se encontro el medio pago correspondiente",
//             ))),
//         }
//     }
//     pub async fn venta(db: &Pool<Sqlite>, venta: VentaDB, user: &Option<Arc<User>>) -> Res<Venta> {
//         {
//             let qres:Vec<RelatedProdDB>=sqlx::query_as!(RelatedProdDB,"select productos.id as id,
//                     precio, porcentaje, precio_costo, tipo, marca, variedad, presentacion, size, cantidad
//                     from relacion_venta_prod inner join productos on relacion_venta_prod.id = productos.id where venta = ?
//                      ",venta.id).fetch_all(db).await?;
//             let mut productos = Vec::new();
//             for rel in qres {
//                 let qres: Vec<BigIntDB> = sqlx::query_as!(
//                     BigIntDB,
//                     "select codigo as int from codigos where producto = ? limit 5",
//                     rel.id
//                 )
//                 .fetch_all(db)
//                 .await?;
//                 let codes = qres.iter().map(|c| *c.int).collect::<Vec<i64>>();
//                 productos.push(Valuable::Prod((
//                     rel.cantidad,
//                     Producto::new(
//                         rel.id,
//                         codes,
//                         rel.precio,
//                         rel.porcentaje,
//                         rel.precio_costo,
//                         rel.tipo,
//                         rel.marca,
//                         rel.variedad,
//                         Presentacion::build(rel.presentacion, rel.size),
//                     ),
//                 )))
//             }
//             let qres:Vec<RelatedPesDB>=sqlx::query_as!(RelatedPesDB,"select pesables.id as id,
//                     precio_peso, porcentaje, costo_kilo, descripcion, cantidad, updated_at
//                     from relacion_venta_pes inner join pesables on relacion_venta_pes.id = pesables.id where venta = ?
//                      ",venta.id).fetch_all(db).await?;
//             for rel in qres {
//                 let qres: Option<BigIntDB> = sqlx::query_as!(
//                     BigIntDB,
//                     "select codigo as int from codigos where pesable = ? limit 1",
//                     rel.id
//                 )
//                 .fetch_optional(db)
//                 .await?;
//                 match qres {
//                     Some(model) => productos.push(Valuable::Pes((
//                         rel.cantidad,
//                         Pesable::build(
//                             rel.id,
//                             model.int,
//                             rel.precio_peso,
//                             rel.porcentaje,
//                             rel.costo_kilo,
//                             rel.descripcion,
//                         ),
//                     ))),
//                     None => {
//                         return Err(AppError::IncorrectError(String::from(
//                             "No se encontro codigo de pesable",
//                         )))
//                     }
//                 }
//             }
//             let qres:Vec<RelatedRubDB>=sqlx::query_as!(RelatedRubDB,"select rubros.id as id, descripcion, updated_at, cantidad, precio
//                     from relacion_venta_rub inner join rubros on relacion_venta_rub.id = rubros.id where venta = ?
//                      ",venta.id).fetch_all(db).await?;
//             for rel in qres {
//                 let qres: Option<BigIntDB> = sqlx::query_as!(
//                     BigIntDB,
//                     "select codigo as int from codigos where pesable = ? limit 1",
//                     rel.id
//                 )
//                 .fetch_optional(db)
//                 .await?;
//                 match qres {
//                     Some(model) => productos.push(Valuable::Rub((
//                         rel.cantidad,
//                         Rubro::build(rel.id, model.int, Some(rel.precio), rel.descripcion),
//                     ))),
//                     None => {
//                         return Err(AppError::IncorrectError(String::from(
//                             "No se encontro codigo de pesable",
//                         )))
//                     }
//                 }
//             }
//             let qres: Vec<PagoDB> =
//                 sqlx::query_as!(PagoDB, "select * from pagos where venta = ? ", venta.id)
//                     .fetch_all(db)
//                     .await?;
//             let mut pagos = Vec::new();
//             for pago in qres {
//                 let qres: Option<MedioPagoDB> = sqlx::query_as!(
//                     MedioPagoDB,
//                     "select * from medios_pago where id = ? limit 1",
//                     pago.medio_pago
//                 )
//                 .fetch_optional(db)
//                 .await?;
//                 let medio = match qres {
//                     Some(medio_p) => MedioPago::build(medio_p.medio, medio_p.id),
//                     None => {
//                         return Err(AppError::IncorrectError(String::from(
//                             "no es encontro medio_pago de pago",
//                         )))
//                     }
//                 };
//                 pagos.push(Pago::build(pago.id, medio, pago.monto, pago.pagado))
//             }
//             let qres: Option<ClienteDB> = sqlx::query_as!(
//                 ClienteDB,
//                 "select * from clientes where id = ? limit 1",
//                 venta.cliente
//             )
//             .fetch_optional(db)
//             .await?;
//             let cliente = match qres {
//                 Some(cliente) => Cliente::Regular(Cli::build(
//                     cliente.id,
//                     Arc::from(cliente.nombre),
//                     cliente.dni,
//                     cliente.activo,
//                     cliente.time,
//                     cliente.limite,
//                 )),
//                 None => Cliente::Final,
//             };
//             Ok(Venta::build(
//                 venta.id,
//                 venta.monto_total,
//                 productos,
//                 pagos,
//                 venta.monto_pagado,
//                 user.clone(),
//                 cliente,
//                 venta.paga,
//                 venta.cerrada,
//                 venta.time,
//             ))
//         }
//     }
// }

pub mod map {
    use chrono::NaiveDateTime;
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct BigIntDB {
        pub int: i64,
    }
    #[derive(FromRow)]
    pub struct IntDB {
        pub int: i32,
    }

    #[derive(FromRow)]
    pub struct DoubleDB {
        pub double: f64,
    }

    #[derive(FromRow)]
    pub struct FloatDB {
        pub float: f32,
    }

    #[derive(FromRow)]
    pub struct BoolDB {
        pub val: bool,
    }

    #[derive(FromRow)]
    pub struct StringDB {
        pub string: String,
    }

    #[derive(FromRow)]
    pub struct MedioPagoDB<'a> {
        pub id: i64,
        pub medio: &'a str,
    }
    #[derive(FromRow)]
    pub struct CajaParcialDB<'a> {
        pub id: i64,
        pub cierre: Option<NaiveDateTime>,
        pub ventas_totales: f32,
        pub cajero: Option<&'a str>,
    }

    #[derive(FromRow)]
    pub struct CajaDB<'a> {
        pub id: i64,
        pub inicio: NaiveDateTime,
        pub cierre: Option<NaiveDateTime>,
        pub monto_inicio: f32,
        pub monto_cierre: Option<f32>,
        pub ventas_totales: f32,
        pub cajero: Option<&'a str>,
    }

    #[derive(FromRow)]
    pub struct ClienteDB<'a> {
        pub id: i64,
        pub nombre: &'a str,
        pub dni: i32,
        pub limite: Option<f32>,
        pub activo: bool,
        pub time: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct ConfigDB<'a> {
        pub id: i64,
        pub politica: f32,
        pub formato: &'a str,
        pub mayus: &'a str,
        pub cantidad: u8,
    }
    #[derive(FromRow)]
    pub struct ProvDB<'a> {
        pub id: i64,
        pub nombre: &'a str,
        pub contacto: Option<i64>,
        pub updated: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct CodeDB {
        pub id: i64,
        pub codigo: i64,
        pub producto: Option<i64>,
        pub pesable: Option<i64>,
        pub rubro: Option<i64>,
    }
    #[derive(FromRow)]
    pub struct PesableDB<'a> {
        pub id: i64,
        pub precio_peso: f32,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: &'a str,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedPesDB<'a> {
        pub id: i64,
        pub precio_peso: f32,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: &'a str,
        pub updated_at: NaiveDateTime,
        pub cantidad: f32,
    }
    #[derive(FromRow)]
    pub struct CodedPesDB<'a> {
        pub id: i64,
        pub precio_peso: f32,
        pub codigo: i64,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: &'a str,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RubroDB<'a> {
        pub id: i64,
        pub descripcion: &'a str,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedRubDB<'a> {
        pub id: i64,
        pub descripcion: &'a str,
        pub updated_at: NaiveDateTime,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct CodedRubDB<'a> {
        pub id: i64,
        pub descripcion: &'a str,
        pub updated_at: NaiveDateTime,
        pub codigo: i64,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct ProductoDB<'a> {
        pub id: i64,
        pub precio_venta: f32,
        pub porcentaje: f32,
        pub precio_costo: f32,
        pub tipo: &'a str,
        pub marca: &'a str,
        pub variedad: &'a str,
        pub presentacion: &'a str,
        pub size: f32,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedProdDB<'a> {
        pub id: i64,
        pub precio: f32,
        pub porcentaje: f32,
        pub precio_costo: f32,
        pub tipo: &'a str,
        pub marca: &'a str,
        pub variedad: &'a str,
        pub presentacion: &'a str,
        pub size: f32,
        pub cantidad: u8,
    }
    #[derive(FromRow)]
    pub struct UserDB<'a> {
        pub id: i64,
        pub user_id: &'a str,
        pub nombre: &'a str,
        pub pass: i64,
        pub rango: &'a str,
    }
    #[derive(FromRow)]
    pub struct DeudaDB {
        pub id: i64,
        pub cliente: i64,
        pub pago: i64,
        pub monto: f32,
    }
    #[derive(FromRow)]
    pub struct MovimientoDB<'a> {
        pub id: i64,
        pub caja: i64,
        pub tipo: bool,
        pub monto: f32,
        pub descripcion: Option<&'a str>,
        pub time: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct PagoDB {
        pub id: i64,
        pub medio_pago: i64,
        pub monto: f32,
        pub pagado: f32,
        pub venta: i64,
    }
    #[derive(FromRow)]
    pub struct RelacionProdProvDB {
        pub id: i64,
        pub producto: i64,
        pub proveedor: i64,
        pub codigo: i64,
    }
    #[derive(FromRow)]
    pub struct RelacionVentaPesDB {
        pub id: i64,
        pub venta: i64,
        pub pesable: i64,
        pub cantidad: f32,
        pub precio_kilo: f32,
    }
    #[derive(FromRow)]
    pub struct RelacionVentaProdDB {
        pub id: i64,
        pub venta: i64,
        pub producto: i64,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct RelacionVentaRubDB {
        pub id: i64,
        pub venta: i64,
        pub rubro: i64,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct VentaDB {
        pub id: i64,
        pub time: NaiveDateTime,
        pub monto_total: f64,
        pub monto_pagado: f64,
        pub cliente: Option<i64>,
        pub cerrada: bool,
        pub paga: bool,
        pub pos: bool,
    }
    #[derive(FromRow)]
    pub struct TotalDB<'a> {
        pub medio: &'a str,
        pub monto: f32,
    }
}

// async fn test(db: &Pool<Sqlite>){
//     let res: sqlx::Result<Option<Venta>> = query_as!(
//Venta,
//         "select * from ventas").fetch_optional(db).await;
//     let res= res.unwrap().unwrap();
// }
