use chrono::Utc;

use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Execute, Pool, Sqlite};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::sync::Arc;
use Valuable as V;

use super::{
    AppError, Cli, Cliente, MedioPago, Pago, Pesable, Presentacion, Producto, Proveedor,
    RelacionProdProv, Res, Rubro, User, Venta,
};
use crate::db::Model;
use crate::mods::valuable::Valuable;
pub struct Db;
pub struct Mapper;
pub fn get_hash(pass: &str) -> i64 {
    let mut h = DefaultHasher::new();
    pass.hash(&mut h);
    h.finish() as i64
}
pub fn crear_file<'a>(path: &str, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    println!("Path que se actualiza: {}", path);
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", buf)?;
    Ok(())
}

pub fn leer_file<T: DeserializeOwned + Clone + Serialize>(
    buf: &mut T,
    path: &str,
) -> std::io::Result<()> {
    let file2 = File::open(path);
    let mut file2 = match file2 {
        Ok(file) => file,
        Err(_) => {
            let esc: Vec<String> = Vec::new();
            crear_file(path, &esc)?;
            File::open(path)?
        }
    };

    let mut buf2 = String::new();
    file2.read_to_string(&mut buf2)?;
    match serde_json::from_str::<T>(&buf2.clone()) {
        Ok(a) => *buf = a.clone(),
        Err(e) => panic!("No se pudo porque {}", e),
    }
    Ok(())
}
pub fn redondeo(politica: &f32, numero: f32) -> f32 {
    let mut res = numero;
    let dif = numero % politica;
    if dif != 0.0 {
        if dif < politica / 2.0 {
            res = numero - dif;
        } else {
            res = numero + politica - dif;
        }
    }
    res
}
// impl Mapper {
//     pub async fn map_model_prod(prod: &ProdDB::Model, db: &DatabaseConnection) -> Res<Producto> {
//         let cods = CodeDB::Entity::find()
//             .filter(CodeDB::Column::Producto.eq(prod.id))
//             .all(db)
//             .await?
//             .iter()
//             .map(|c| c.codigo)
//             .collect();

//         let presentacion = match prod.presentacion.as_str() {
//             "Gr" => Presentacion::Gr(prod.cantidad),
//             "Un" => Presentacion::Un(prod.cantidad as i16),
//             "Lt" => Presentacion::Lt(prod.cantidad),
//             "Ml" => Presentacion::Ml(prod.cantidad as i16),
//             "CC" => Presentacion::CC(prod.cantidad as i16),
//             "Kg" => Presentacion::Kg(prod.cantidad),
//             a => return Err(AppError::SizeSelection(a.to_string())),
//         };
//         Ok(Producto::new(
//             prod.id,
//             cods,
//             prod.precio_de_venta,
//             prod.porcentaje,
//             prod.precio_de_costo,
//             prod.tipo_producto.as_str(),
//             prod.marca.as_str(),
//             prod.variedad.as_str(),
//             presentacion,
//         ))
//     }
//     pub fn map_model_rub(rub: &RubDB::Model, monto: f32) -> Rubro {
//         Rubro::new(
//             rub.id,
//             rub.codigo,
//             Some(monto),
//             Arc::from(rub.descripcion.as_str()),
//         )
//     }

//     pub fn map_model_pes(pes: &PesDB::Model) -> Pesable {
//         Pesable::new(
//             pes.id,
//             pes.codigo,
//             pes.precio_peso,
//             pes.porcentaje,
//             pes.costo_kilo,
//             pes.descripcion.as_str(),
//         )
//     }
//     pub fn map_model_prov(prov: &ProvDB::Model) -> Proveedor {
//         Proveedor::new(prov.id, prov.nombre.as_str(), prov.contacto)
//     }
//     pub async fn map_model_sale(
//         venta: &VentaDB::Model,
//         db: &DatabaseConnection,
//         user: &Option<Arc<User>>,
//     ) -> Res<Venta> {
//         let pagos_mod = PagoDB::Entity::find()
//             .filter(PagoDB::Column::Venta.eq(venta.id))
//             .all(db)
//             .await?;
//         let mut pagos = Vec::new();
//         for pago_mod in pagos_mod {
//             let medio = MedioDB::Entity::find_by_id(pago_mod.medio_pago)
//                 .one(db)
//                 .await?
//                 .unwrap();

//             pagos.push(Pago::new(
//                 MedioPago::new(&medio.medio, medio.id),
//                 pago_mod.monto,
//                 Some(pago_mod.pagado),
//             ));
//         }
//         let mut prods = Vec::new();
//         for model in VentaPesDB::Entity::find()
//             .filter(VentaPesDB::Column::Venta.eq(venta.id))
//             .all(db)
//             .await?
//         {
//             let pes_mod = PesDB::Entity::find_by_id(model.pesable)
//                 .one(db)
//                 .await?
//                 .unwrap();
//             prods.push(Valuable::Pes((
//                 model.cantidad,
//                 Pesable::new(
//                     pes_mod.id,
//                     pes_mod.codigo,
//                     pes_mod.precio_peso,
//                     pes_mod.porcentaje,
//                     pes_mod.costo_kilo,
//                     &pes_mod.descripcion,
//                 ),
//             )))
//         }
//         for model in VentaProdDB::Entity::find()
//             .filter(VentaProdDB::Column::Venta.eq(venta.id))
//             .all(db)
//             .await?
//         {
//             let prod = ProdDB::Entity::find_by_id(model.producto)
//                 .one(db)
//                 .await?
//                 .unwrap();
//             let codes = CodeDB::Entity::find()
//                 .filter(CodeDB::Column::Producto.eq(prod.id))
//                 .all(db)
//                 .await?
//                 .iter()
//                 .map(|c| c.codigo)
//                 .collect();
//             let prod = Producto::new(
//                 prod.id,
//                 codes,
//                 prod.precio_de_venta,
//                 prod.porcentaje,
//                 prod.precio_de_costo,
//                 prod.tipo_producto.as_str(),
//                 prod.marca.as_str(),
//                 prod.variedad.as_str(),
//                 match prod.presentacion.as_str() {
//                     "CC" => Presentacion::CC(prod.cantidad as i16),
//                     "Gr" => Presentacion::Gr(prod.cantidad),
//                     "Kg" => Presentacion::Kg(prod.cantidad),
//                     "Lt" => Presentacion::Lt(prod.cantidad),
//                     "Ml" => Presentacion::Ml(prod.cantidad as i16),
//                     "Un" => Presentacion::Un(prod.cantidad as i16),
//                     _ => {
//                         return Err(AppError::IncorrectError(
//                             "Error de consistencia en DB".to_string(),
//                         ))
//                     }
//                 },
//             );
//             prods.push(Valuable::Prod((model.cantidad, prod)));
//         }
//         for model in VentaRubDB::Entity::find()
//             .filter(VentaRubDB::Column::Venta.eq(venta.id))
//             .all(db)
//             .await?
//         {
//             let rub = RubDB::Entity::find_by_id(model.id).one(db).await?.unwrap();
//             let rub = Rubro::new(
//                 rub.id,
//                 rub.codigo,
//                 rub.monto,
//                 Arc::from(rub.descripcion.as_str()),
//             );
//             prods.push(Valuable::Rub((model.cantidad, rub)));
//         }
//         let cliente = match venta.cliente {
//             None => Cliente::Final,
//             Some(c) => {
//                 let model = CliDB::Entity::find_by_id(c).one(db).await?.unwrap();
//                 Cliente::Regular(Cli::new(
//                     model.id,
//                     Arc::from(model.nombre.as_str()),
//                     model.dni,
//                     model.activo,
//                     model.created,
//                     model.limite,
//                 ))
//             }
//         };

//         let venta = Venta::build(
//             venta.id,
//             venta.monto_total,
//             prods,
//             pagos,
//             venta.monto_pagado,
//             user.clone(),
//             cliente,
//             venta.paga,
//             venta.cerrada,
//         );
//         Ok(venta)
//     }
// }

impl Db {
    pub async fn eliminar_usuario(user: User, db: &Pool<Sqlite>) -> Res<()> {
        let id = user.id();
        let qres: Option<Model> =
            sqlx::query_as!(Model::Int, "select id as int from users where id = ?", id)
                .fetch_optional(db)
                .await?;
        match qres {
            Some(model) => match model {
                Model::Int { int } => {
                    sqlx::query("delete from users where id = ?")
                        .bind(int)
                        .execute(db)
                        .await?;
                    Ok(())
                }
                _ => Err(AppError::IncorrectError(String::from("se esperaba Int"))),
            },
            None => Err(AppError::NotFound {
                objeto: String::from("Usuario"),
                instancia: user.id().to_string(),
            }),
        }
    }
    pub async fn cargar_todos_los_productos(
        productos: &Vec<Producto>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        let mut codigos_query =
            String::from("insert into codigos (codigo, producto) values (?, ?)");
        let mut productos_query = String::from("insert into productos (id, precio_venta, porcentaje, precio_costo, tipo, marca, variedad, presentacion, size, updated_at) values (?, ?, ?, ?, ?, ?, ?, ?, ?)");
        let codigos_row = ", (?, ?)";
        let productos_row = ", (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        for _ in 1..productos[0].codigos_de_barras().len() {
            codigos_query.push_str(codigos_row);
        }
        for i in 1..productos.len() {
            productos_query.push_str(productos_row);
            for _ in 0..productos[i].codigos_de_barras().len() {
                codigos_query.push_str(codigos_row);
            }
        }
        let mut sqlx_productos = sqlx::query(productos_query.as_str());
        let mut sqlx_codigos = sqlx::query(codigos_query.as_str());
        for producto in productos {
            sqlx_productos = sqlx_productos
                .bind(*producto.id())
                .bind(*producto.precio_de_venta())
                .bind(*producto.porcentaje())
                .bind(*producto.precio_de_costo())
                .bind(producto.tipo_producto().to_string())
                .bind(producto.marca().to_string())
                .bind(producto.variedad().to_string())
                .bind(producto.presentacion().get_string())
                .bind(producto.presentacion().get_cantidad())
                .bind(Utc::now().naive_local());
            for codigo in producto.codigos_de_barras() {
                sqlx_codigos = sqlx_codigos.bind(*codigo).bind(*producto.id());
            }
        }
        sqlx_productos.execute(db).await?;
        sqlx_codigos.execute(db).await?;
        Ok(())
    }
    // pub async fn cargar_actualizar_todos_los_productos(
    //     productos: &Vec<Producto>,
    //     db: &DatabaseConnection,
    // ) -> Result<(), DbErr> {
    //     for producto in productos {
    //         let encontrado = ProdDB::Entity::find_by_id(*producto.id()).one(db).await?;
    //         let mut model: ProdDB::ActiveModel;
    //         let codigo_prod: i64;
    //         match encontrado {
    //             Some(m) => {
    //                 codigo_prod = m.id;
    //                 model = m.into();
    //                 model.marca = Set(producto.marca().to_string());
    //                 model.porcentaje = Set(*producto.porcentaje());
    //                 model.precio_de_costo = Set(*producto.precio_de_costo());
    //                 model.precio_de_venta = Set(*producto.precio_de_venta());
    //                 model.presentacion = Set(producto.presentacion().to_string());
    //                 model.tipo_producto = Set(producto.tipo_producto().to_string());
    //                 model.updated_at = Set(Utc::now().naive_local());
    //                 model.variedad = Set(producto.variedad().to_string());
    //                 model.update(db).await?;
    //             }
    //             None => {
    //                 model = ProdDB::ActiveModel {
    //                     precio_de_venta: Set(*producto.precio_de_venta()),
    //                     porcentaje: Set(*producto.porcentaje()),
    //                     precio_de_costo: Set(*producto.precio_de_costo()),
    //                     tipo_producto: Set(producto.tipo_producto().to_string()),
    //                     marca: Set(producto.marca().to_string()),
    //                     variedad: Set(producto.variedad().to_string()),
    //                     presentacion: Set(producto.presentacion().get_string()),
    //                     updated_at: Set(Utc::now().naive_local()),
    //                     cantidad: Set(producto.presentacion().get_cantidad()),
    //                     ..Default::default()
    //                 };

    //                 codigo_prod = ProdDB::Entity::insert(model).exec(db).await?.last_insert_id;
    //             }
    //         }

    //         CodeDB::Entity::delete_many()
    //             .filter(Condition::all().add(CodeDB::Column::Producto.eq(codigo_prod)))
    //             .exec(db)
    //             .await?;

    //         let codigos_model: Vec<CodeDB::ActiveModel> = producto
    //             .codigos_de_barras()
    //             .iter()
    //             .map(|x| CodeDB::ActiveModel {
    //                 codigo: Set(*x),
    //                 producto: Set(codigo_prod),
    //                 ..Default::default()
    //             })
    //             .collect();

    //         if codigos_model.len() > 1 {
    //             CodeDB::Entity::insert_many(codigos_model).exec(db).await?;
    //         } else if codigos_model.len() == 1 {
    //             CodeDB::Entity::insert(codigos_model[0].to_owned())
    //                 .exec(db)
    //                 .await?;
    //         }
    //     }

    //     Ok(())
    // }
    pub async fn cargar_todos_los_pesables(
        pesables: Vec<&Pesable>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        let mut pesables_inicio=String::from("insert into pesables (id, precio_peso, porcentaje, costo_kilo, descripcion, updated_at) values (?, ?, ?, ?, ?, ?)");
        let mut codigos_inicio =
            String::from("insert into codigos (codigo, pesable) values (?, ?)");
        let pes_row = ", (?, ?, ?, ?, ?, ?)";
        let codigos_row = ", (?, ?)";
        for _ in 1..pesables.len() {
            pesables_inicio.push_str(pes_row);
            codigos_inicio.push_str(codigos_row);
        }
        let mut pesables_query = sqlx::query(pesables_inicio.as_str());
        let mut codigos_query = sqlx::query(codigos_inicio.as_str());
        for pesable in pesables {
            pesables_query = pesables_query
                .bind(*pesable.id())
                .bind(*pesable.precio_peso())
                .bind(*pesable.porcentaje())
                .bind(*pesable.costo_kilo())
                .bind(pesable.desc())
                .bind(Utc::now().naive_local());
            codigos_query = codigos_query.bind(*pesable.codigo()).bind(pesable.id());
        }
        pesables_query.execute(db).await?;
        codigos_query.execute(db).await?;
        Ok(())
    }
    pub async fn cargar_todos_los_rubros(
        rubros: Vec<&Rubro>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        let mut rubros_inicio =
            String::from("insert into rubros (id, descripcion, updated_at) values (?, ?, ?)");
        let mut codigos_inicio = String::from("insert into codigos (codigo, rubro) values (?, ?)");
        let rub_row = ", (?, ?, ?)";
        let codigos_row = ", (?, ?)";
        for _ in 1..rubros.len() {
            rubros_inicio.push_str(rub_row);
            codigos_inicio.push_str(codigos_row);
        }
        let mut rubros_query = sqlx::query(rubros_inicio.as_str());
        let mut codigos_query = sqlx::query(codigos_inicio.as_str());
        for rubro in rubros {
            rubros_query = rubros_query
                .bind(*rubro.id())
                .bind(rubro.descripcion().to_string())
                .bind(Utc::now().naive_local());
            codigos_query = codigos_query.bind(*rubro.codigo()).bind(rubro.id());
        }
        rubros_query.execute(db).await?;
        codigos_query.execute(db).await?;
        Ok(())
    }
    pub async fn cargar_todos_los_valuables(
        productos: Vec<Valuable>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        Db::cargar_todos_los_productos(
            &productos
                .iter()
                .filter_map(|x| match x {
                    V::Prod(a) => Some(a.1.clone()),
                    _ => None,
                })
                .collect::<Vec<Producto>>(),
            &db,
        )
        .await?;
        Db::cargar_todos_los_pesables(
            productos
                .iter()
                .filter_map(|val| match val {
                    V::Pes((_, pes)) => Some(pes),
                    _ => None,
                })
                .collect::<Vec<&Pesable>>(),
            &db,
        )
        .await?;
        Db::cargar_todos_los_rubros(productos.iter().filter_map(|val|{
            match val{
                V::Rub((_,rub))=>Some(rub),
                _=>None,
            }
        }).collect::<Vec<&Rubro>>(), &db).await?;
        Ok(())
    }
    pub async fn cargar_todos_los_provs(proveedores: Vec<Proveedor>) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        for prov in proveedores {
            let model = ProvDB::ActiveModel {
                id: Set(*prov.id()),
                updated_at: Set(Utc::now().naive_local()),
                nombre: Set(prov.nombre().to_string()),
                contacto: Set(prov.contacto().clone()),
            };
            if ProvDB::Entity::find_by_id(model.clone().id.unwrap())
                .one(&db)
                .await?
                .is_some()
            {
                model.update(&db).await?;
            } else {
                model.insert(&db).await?;
            }
        }
        Ok(())
    }

    pub async fn cargar_todas_las_relaciones_prod_prov(
        relaciones: Vec<RelacionProdProv>,
    ) -> Result<(), AppError> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        for x in relaciones {
            if let Some(rel) = ProdProvDB::Entity::find()
                .filter(
                    Condition::all()
                        .add(ProdProvDB::Column::Producto.eq(*x.id_producto()))
                        .add(ProdProvDB::Column::Proveedor.eq(*x.id_proveedor())),
                )
                .one(&db)
                .await?
            {
                if rel.codigo != x.codigo_interno() {
                    let mut act = rel.into_active_model();
                    act.codigo = Set(x.codigo_interno());
                    act.clone().update(&db).await?;
                    println!("updating {:?}", act);
                }
            } else {
                let model = ProdProvDB::ActiveModel {
                    producto: Set(*x.id_producto()),
                    proveedor: Set(*x.id_proveedor()),
                    codigo: Set(x.codigo_interno()),
                    ..Default::default()
                };
                println!("inserting {:?}", model);
                model.insert(&db).await?;
            }
        }

        Ok(())
    }
}
