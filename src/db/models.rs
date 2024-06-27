use crate::db::map::{
    BigIntDB, CajaDB, ClienteDB, ConfigDB, MedioPagoDB, PagoDB, ProductoDB, RelacionProdProvDB,
    RelatedPesDB, RelatedProdDB, RelatedRubDB, TotalDB, VentaDB,
};
use crate::mods::{
    AppError, Caja, Cli, Cliente, Config, MedioPago, Pesable, RelacionProdProv, Rubro, User,
    Valuable,
};
use crate::mods::{Pago, Presentacion, Producto, Res, Venta};
use sqlx::{query_as, Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Mapper;
impl Mapper {
    pub async fn caja<'a>(db: &'a Pool<Sqlite>, caja: CajaDB) -> Res<Caja> {
        let totales_mod: sqlx::Result<Vec<TotalDB>> = query_as!(
            TotalDB,
            r#"select medio, monto as "monto: _" from totales where caja = ? "#,
            caja.id
        )
        .fetch_all(db)
        .await;
        let mut totales = HashMap::new();
        for tot in totales_mod? {
            totales.insert(Arc::from(tot.medio), tot.monto);
        }
        Ok(Caja::build(
            caja.id,
            caja.inicio,
            caja.cierre,
            caja.ventas_totales,
            caja.monto_inicio,
            caja.monto_cierre,
            caja.cajero.map(|c| Arc::from(c)),
            totales,
        ))
    }
    pub async fn config(db: &Pool<Sqlite>, config: ConfigDB) -> Res<Config> {
        let medios: sqlx::Result<Vec<MedioPagoDB>> =
            sqlx::query_as!(MedioPagoDB, "select * from medios_pago ")
                .fetch_all(db)
                .await;
        let medios = medios?
            .iter()
            .map(|model| Arc::from(model.medio.to_owned()))
            .collect::<Vec<Arc<str>>>();
        Ok(Config::build(
            config.politica,
            config.formato.as_str(),
            config.mayus.as_str(),
            config.cantidad,
            medios,
        ))
    }
    pub fn rel_prod_prov(rel: &RelacionProdProvDB) -> RelacionProdProv {
        RelacionProdProv::new(rel.proveedor, rel.codigo)
    }
    pub async fn producto(db: &Pool<Sqlite>, prod: ProductoDB) -> Res<Producto> {
        let models: sqlx::Result<Vec<BigIntDB>> = sqlx::query_as!(
            BigIntDB,
            "select codigo as int from codigos where producto = ? limit 5",
            prod.id
        )
        .fetch_all(db)
        .await;
        let rels: Vec<RelacionProdProvDB> = sqlx::query_as!(
            RelacionProdProvDB,
            "select * from relacion_prod_prov where producto = ?",
            prod.id
        )
        .fetch_all(db)
        .await?;
        let rels = rels
            .iter()
            .map(|r| Mapper::rel_prod_prov(r))
            .collect::<Vec<RelacionProdProv>>();
        let codigos = models?.iter().map(|model| model.int).collect::<Vec<i64>>();
        let presentacion = match prod.presentacion.as_str() {
            "Gr" => Presentacion::Gr(prod.size),
            "Un" => Presentacion::Un(prod.size as u16),
            "Lt" => Presentacion::Lt(prod.size),
            "Ml" => Presentacion::Ml(prod.size as u16),
            "CC" => Presentacion::CC(prod.size as u16),
            "Kg" => Presentacion::Kg(prod.size),
            a => return Err(AppError::SizeSelection(a.to_string())),
        };
        Ok(Producto::build(
            prod.id,
            codigos,
            prod.precio_venta,
            prod.porcentaje,
            prod.precio_costo,
            prod.tipo.as_str(),
            prod.marca.as_str(),
            prod.variedad.as_str(),
            presentacion,
            rels,
        ))
    }
    pub async fn pago(db: &Pool<Sqlite>, pago: PagoDB) -> Res<Pago> {
        let medio: Option<MedioPagoDB> = sqlx::query_as!(
            MedioPagoDB,
            "select * from medios_pago where id = ? limit 1",
            pago.medio_pago
        )
        .fetch_optional(db)
        .await?;
        let int_id = pago.id;
        match medio {
            Some(med) => Ok(Pago::build(
                int_id,
                MedioPago::build(&med.medio, med.id),
                pago.monto,
                pago.pagado,
            )),
            None => Err(AppError::IncorrectError(String::from(
                "No se encontro el medio pago correspondiente",
            ))),
        }
    }
    pub async fn venta(db: &Pool<Sqlite>, venta: VentaDB, user: &Option<Arc<User>>) -> Res<Venta> {
        {
            let qres:Vec<RelatedProdDB>=sqlx::query_as!(RelatedProdDB,r#"select productos.id as id,
                    precio as "precio: _",porcentaje as "porcentaje: _", precio_costo as "precio_costo: _", tipo, marca, variedad, presentacion, size as "size: _", cantidad as "cantidad: _"
                    from relacion_venta_prod inner join productos on relacion_venta_prod.id = productos.id where venta = ?
                     "#,venta.id).fetch_all(db).await?;
            let mut productos = Vec::new();
            for rel in qres {
                let qres: Vec<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select codigo as int from codigos where producto = ? limit 5",
                    rel.id
                )
                .fetch_all(db)
                .await?;
                let rels: Vec<RelacionProdProvDB> = sqlx::query_as!(
                    RelacionProdProvDB,
                    "select * from relacion_prod_prov where producto = ?",
                    rel.id
                )
                .fetch_all(db)
                .await?;
                let rels = rels
                    .iter()
                    .map(|r| Mapper::rel_prod_prov(r))
                    .collect::<Vec<RelacionProdProv>>();
                let codes = qres.iter().map(|c| c.int).collect::<Vec<i64>>();
                productos.push(Valuable::Prod((
                    rel.cantidad,
                    Producto::build(
                        rel.id,
                        codes,
                        rel.precio,
                        rel.porcentaje,
                        rel.precio_costo,
                        &rel.tipo,
                        &rel.marca,
                        &rel.variedad,
                        Presentacion::build(&rel.presentacion, rel.size),
                        rels,
                    ),
                )))
            }
            let qres:Vec<RelatedPesDB>=sqlx::query_as!(RelatedPesDB,r#"select pesables.id as id,
                    precio_peso as "precio_peso: _", porcentaje as "porcentaje: _", costo_kilo as "costo_kilo: _", descripcion, cantidad as "cantidad: _", updated_at
                    from relacion_venta_pes inner join pesables on relacion_venta_pes.id = pesables.id where venta = ?
                     "#,venta.id).fetch_all(db).await?;
            for rel in qres {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select codigo as int from codigos where pesable = ? limit 1",
                    rel.id
                )
                .fetch_optional(db)
                .await?;
                match qres {
                    Some(model) => productos.push(Valuable::Pes((
                        rel.cantidad,
                        Pesable::build(
                            rel.id,
                            model.int,
                            rel.precio_peso,
                            rel.porcentaje,
                            rel.costo_kilo,
                            &rel.descripcion,
                        ),
                    ))),
                    None => {
                        return Err(AppError::IncorrectError(String::from(
                            "No se encontro codigo de pesable",
                        )))
                    }
                }
            }
            let qres:Vec<RelatedRubDB>=sqlx::query_as!(RelatedRubDB,r#"select rubros.id as id, descripcion, updated_at, cantidad as "cantidad: _", precio as "precio: _"
                    from relacion_venta_rub inner join rubros on relacion_venta_rub.id = rubros.id where venta = ?
                     "#,venta.id).fetch_all(db).await?;
            for rel in qres {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select codigo as int from codigos where pesable = ? limit 1",
                    rel.id
                )
                .fetch_optional(db)
                .await?;
                match qres {
                    Some(model) => productos.push(Valuable::Rub((
                        rel.cantidad,
                        Rubro::build(
                            rel.id,
                            model.int,
                            Some(rel.precio),
                            rel.descripcion.as_str(),
                        ),
                    ))),
                    None => {
                        return Err(AppError::IncorrectError(String::from(
                            "No se encontro codigo de pesable",
                        )))
                    }
                }
            }
            let qres: Vec<PagoDB> = sqlx::query_as!(
                PagoDB,
                r#"select id, medio_pago, monto as "monto: _", pagado as "pagado: f32",
    venta from pagos where venta = ? "#,
                venta.id
            )
            .fetch_all(db)
            .await?;
            let mut pagos = Vec::new();
            for pago in qres {
                let qres: Option<MedioPagoDB> = sqlx::query_as!(
                    MedioPagoDB,
                    "select * from medios_pago where id = ? limit 1",
                    pago.medio_pago
                )
                .fetch_optional(db)
                .await?;
                let medio = match qres {
                    Some(medio_p) => MedioPago::build(medio_p.medio.as_str(), medio_p.id),
                    None => {
                        return Err(AppError::IncorrectError(String::from(
                            "no es encontro medio_pago de pago",
                        )))
                    }
                };
                pagos.push(Pago::build(pago.id, medio, pago.monto, pago.pagado))
            }
            let qres: Option<ClienteDB> = sqlx::query_as!(
                ClienteDB,
                r#"select id, nombre, dni as "dni: _", limite as "limite: _", activo, time from clientes where id = ? limit 1"#,
                venta.cliente
            )
            .fetch_optional(db)
            .await?;
            let cliente = match qres {
                Some(cliente) => Cliente::Regular(Cli::build(
                    cliente.id,
                    Arc::from(cliente.nombre),
                    cliente.dni,
                    cliente.activo,
                    cliente.time,
                    cliente.limite,
                )),
                None => Cliente::Final,
            };
            Ok(Venta::build(
                venta.id,
                venta.monto_total,
                productos,
                pagos,
                venta.monto_pagado,
                user.clone(),
                cliente,
                venta.paga,
                venta.cerrada,
                venta.time,
            ))
        }
    }
}

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
    pub struct MedioPagoDB {
        pub id: i64,
        pub medio: String,
    }
    #[derive(FromRow)]
    pub struct CajaParcialDB {
        pub id: i64,
        pub cierre: Option<NaiveDateTime>,
        pub ventas_totales: f32,
        pub cajero: Option<String>,
    }

    #[derive(FromRow)]
    pub struct CajaDB {
        pub id: i64,
        pub inicio: NaiveDateTime,
        pub cierre: Option<NaiveDateTime>,
        pub monto_inicio: f32,
        pub monto_cierre: Option<f32>,
        pub ventas_totales: f32,
        pub cajero: Option<String>,
    }

    #[derive(FromRow)]
    pub struct ClienteDB {
        pub id: i64,
        pub nombre: String,
        pub dni: i32,
        pub limite: Option<f32>,
        pub activo: bool,
        pub time: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct ConfigDB {
        pub id: i64,
        pub politica: f32,
        pub formato: String,
        pub mayus: String,
        pub cantidad: u8,
    }
    #[derive(FromRow)]
    pub struct ProvDB {
        pub id: i64,
        pub nombre: String,
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
    pub struct PesableDB {
        pub id: i64,
        pub precio_peso: f32,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedPesDB {
        pub id: i64,
        pub precio_peso: f32,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
        pub cantidad: f32,
    }
    #[derive(FromRow)]
    pub struct CodedPesDB {
        pub id: i64,
        pub precio_peso: f32,
        pub codigo: i64,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RubroDB {
        pub id: i64,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedRubDB {
        pub id: i64,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct CodedRubDB {
        pub id: i64,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
        pub codigo: i64,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct ProductoDB {
        pub id: i64,
        pub precio_venta: f32,
        pub porcentaje: f32,
        pub precio_costo: f32,
        pub tipo: String,
        pub marca: String,
        pub variedad: String,
        pub presentacion: String,
        pub size: f32,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedProdDB {
        pub id: i64,
        pub precio: f32,
        pub porcentaje: f32,
        pub precio_costo: f32,
        pub tipo: String,
        pub marca: String,
        pub variedad: String,
        pub presentacion: String,
        pub size: f32,
        pub cantidad: u8,
    }
    #[derive(FromRow)]
    pub struct UserDB {
        pub id: i64,
        pub user_id: String,
        pub nombre: String,
        pub pass: i64,
        pub rango: String,
    }
    #[derive(FromRow)]
    pub struct DeudaDB {
        pub id: i64,
        pub cliente: i64,
        pub pago: i64,
        pub monto: f32,
    }
    #[derive(FromRow)]
    pub struct MovimientoDB {
        pub id: i64,
        pub caja: i64,
        pub tipo: bool,
        pub monto: f32,
        pub descripcion: Option<String>,
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
        pub codigo: Option<i64>,
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
        pub monto_total: f32,
        pub monto_pagado: f32,
        pub cliente: Option<i64>,
        pub cerrada: bool,
        pub paga: bool,
        pub pos: bool,
    }
    #[derive(FromRow)]
    pub struct TotalDB {
        pub medio: String,
        pub monto: f32,
    }
}

// async fn test(db: &Pool<Sqlite>) {
//     let res: sqlx::Result<Option<Venta>> = query_as!(Venta, "select * from ventas")
//         .fetch_optional(db)
//         .await;
//     let res = res.unwrap().unwrap();
// }
