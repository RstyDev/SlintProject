use chrono::Utc;
use entity::prelude::{
    CliDB, CodeDB, MedioDB, PagoDB, PesDB, ProdDB, ProdProvDB, ProvDB, RubDB, UserDB, VentaDB,
    VentaPesDB, VentaProdDB, VentaRubDB,
};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, Set,
};
use serde::{de::DeserializeOwned, Serialize};
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
impl Mapper {
    pub async fn map_model_prod(prod: &ProdDB::Model, db: &DatabaseConnection) -> Res<Producto> {
        let cods = CodeDB::Entity::find()
            .filter(CodeDB::Column::Producto.eq(prod.id))
            .all(db)
            .await?
            .iter()
            .map(|c| c.codigo)
            .collect();

        let presentacion = match prod.presentacion.as_str() {
            "Gr" => Presentacion::Gr(prod.cantidad),
            "Un" => Presentacion::Un(prod.cantidad as i16),
            "Lt" => Presentacion::Lt(prod.cantidad),
            "Ml" => Presentacion::Ml(prod.cantidad as i16),
            "CC" => Presentacion::CC(prod.cantidad as i16),
            "Kg" => Presentacion::Kg(prod.cantidad),
            a => return Err(AppError::SizeSelection(a.to_string())),
        };
        Ok(Producto::new(
            prod.id,
            cods,
            prod.precio_de_venta,
            prod.porcentaje,
            prod.precio_de_costo,
            prod.tipo_producto.as_str(),
            prod.marca.as_str(),
            prod.variedad.as_str(),
            presentacion,
        ))
    }
    pub fn map_model_rub(rub: &RubDB::Model, monto: f32) -> Rubro {
        Rubro::new(
            rub.id,
            rub.codigo,
            Some(monto),
            Arc::from(rub.descripcion.as_str()),
        )
    }

    pub fn map_model_pes(pes: &PesDB::Model) -> Pesable {
        Pesable::new(
            pes.id,
            pes.codigo,
            pes.precio_peso,
            pes.porcentaje,
            pes.costo_kilo,
            pes.descripcion.as_str(),
        )
    }
    pub fn map_model_prov(prov: &ProvDB::Model) -> Proveedor {
        Proveedor::new(prov.id, prov.nombre.as_str(), prov.contacto)
    }
    pub async fn map_model_sale(
        venta: &VentaDB::Model,
        db: &DatabaseConnection,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let pagos_mod = PagoDB::Entity::find()
            .filter(PagoDB::Column::Venta.eq(venta.id))
            .all(db)
            .await?;
        let mut pagos = Vec::new();
        for pago_mod in pagos_mod {
            let medio = MedioDB::Entity::find_by_id(pago_mod.medio_pago)
                .one(db)
                .await?
                .unwrap();

            pagos.push(Pago::new(
                MedioPago::new(&medio.medio, medio.id),
                pago_mod.monto,
                Some(pago_mod.pagado),
            ));
        }
        let mut prods = Vec::new();
        for model in VentaPesDB::Entity::find()
            .filter(VentaPesDB::Column::Venta.eq(venta.id))
            .all(db)
            .await?
        {
            let pes_mod = PesDB::Entity::find_by_id(model.pesable)
                .one(db)
                .await?
                .unwrap();
            prods.push(Valuable::Pes((
                model.cantidad,
                Pesable::new(
                    pes_mod.id,
                    pes_mod.codigo,
                    pes_mod.precio_peso,
                    pes_mod.porcentaje,
                    pes_mod.costo_kilo,
                    &pes_mod.descripcion,
                ),
            )))
        }
        for model in VentaProdDB::Entity::find()
            .filter(VentaProdDB::Column::Venta.eq(venta.id))
            .all(db)
            .await?
        {
            let prod = ProdDB::Entity::find_by_id(model.producto)
                .one(db)
                .await?
                .unwrap();
            let codes = CodeDB::Entity::find()
                .filter(CodeDB::Column::Producto.eq(prod.id))
                .all(db)
                .await?
                .iter()
                .map(|c| c.codigo)
                .collect();
            let prod = Producto::new(
                prod.id,
                codes,
                prod.precio_de_venta,
                prod.porcentaje,
                prod.precio_de_costo,
                prod.tipo_producto.as_str(),
                prod.marca.as_str(),
                prod.variedad.as_str(),
                match prod.presentacion.as_str() {
                    "CC" => Presentacion::CC(prod.cantidad as i16),
                    "Gr" => Presentacion::Gr(prod.cantidad),
                    "Kg" => Presentacion::Kg(prod.cantidad),
                    "Lt" => Presentacion::Lt(prod.cantidad),
                    "Ml" => Presentacion::Ml(prod.cantidad as i16),
                    "Un" => Presentacion::Un(prod.cantidad as i16),
                    _ => {
                        return Err(AppError::IncorrectError(
                            "Error de consistencia en DB".to_string(),
                        ))
                    }
                },
            );
            prods.push(Valuable::Prod((model.cantidad, prod)));
        }
        for model in VentaRubDB::Entity::find()
            .filter(VentaRubDB::Column::Venta.eq(venta.id))
            .all(db)
            .await?
        {
            let rub = RubDB::Entity::find_by_id(model.id).one(db).await?.unwrap();
            let rub = Rubro::new(
                rub.id,
                rub.codigo,
                rub.monto,
                Arc::from(rub.descripcion.as_str()),
            );
            prods.push(Valuable::Rub((model.cantidad, rub)));
        }
        let cliente = match venta.cliente {
            None => Cliente::Final,
            Some(c) => {
                let model = CliDB::Entity::find_by_id(c).one(db).await?.unwrap();
                Cliente::Regular(Cli::new(
                    model.id,
                    Arc::from(model.nombre.as_str()),
                    model.dni,
                    model.activo,
                    model.created,
                    model.limite,
                ))
            }
        };

        let venta = Venta::build(
            venta.id,
            venta.monto_total,
            prods,
            pagos,
            venta.monto_pagado,
            user.clone(),
            cliente,
            venta.paga,
            venta.cerrada,
        );
        Ok(venta)
    }
}

impl Db {
    pub async fn eliminar_usuario(user: User, db: Arc<DatabaseConnection>) -> Res<()> {
        let model = UserDB::Entity::find()
            .filter(UserDB::Column::UserId.eq(user.id()))
            .one(db.as_ref())
            .await?;
        match model {
            Some(a) => {
                a.into_active_model().delete(db.as_ref()).await?;
            }
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Usuario"),
                    instancia: user.id().to_string(),
                })
            }
        }
        Ok(())
    }
    pub async fn cargar_todos_los_productos(
        productos: &Vec<Producto>,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let mut prods = Vec::new();
        let mut codes = Vec::new();
        for producto in productos {
            prods.push(ProdDB::ActiveModel {
                id: Set(*producto.id()),
                precio_de_venta: Set(*producto.precio_de_venta()),
                porcentaje: Set(*producto.porcentaje()),
                precio_de_costo: Set(*producto.precio_de_costo()),
                tipo_producto: Set(producto.tipo_producto().to_string()),
                marca: Set(producto.marca().to_string()),
                variedad: Set(producto.variedad().to_string()),
                presentacion: Set(producto.presentacion().get_string()),
                cantidad: Set(producto.presentacion().get_cantidad()),
                updated_at: Set(Utc::now().naive_local()),
            });
            let mut code_mod = producto
                .codigos_de_barras()
                .iter()
                .map(|code| CodeDB::ActiveModel {
                    codigo: Set(*code),
                    producto: Set(*producto.id()),
                    ..Default::default()
                })
                .collect::<Vec<CodeDB::ActiveModel>>();
            codes.append(&mut code_mod);

            if prods.len() == 165 {
                if let Err(e) = ProdDB::Entity::insert_many(prods.clone()).exec(db).await {
                    println!("{:#?}", e);
                }
                prods.clear();
                if let Err(e) = CodeDB::Entity::insert_many(codes.clone()).exec(db).await {
                    println!("{:#?}", e);
                }
                codes.clear();
            }
        }
        if prods.len() > 0 {
            if let Err(e) = ProdDB::Entity::insert_many(prods.clone()).exec(db).await {
                println!("{:#?}", e);
            }
        }
        if codes.len() > 0 {
            if let Err(e) = CodeDB::Entity::insert_many(codes.clone()).exec(db).await {
                println!("{:#?}", e);
            }
        }

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
        productos: &Vec<Valuable>,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        for i in productos {
            match i {
                V::Pes(a) => {
                    let model = PesDB::ActiveModel {
                        codigo: Set(*a.1.codigo()),
                        precio_peso: Set(*a.1.precio_peso()),
                        porcentaje: Set(*a.1.porcentaje()),
                        costo_kilo: Set(*a.1.costo_kilo()),
                        descripcion: Set(a.1.descripcion().to_string()),
                        updated_at: Set(Utc::now().naive_local()),
                        id: Set(*a.1.id()),
                    };
                    if PesDB::Entity::find_by_id(*a.1.id())
                        .one(db)
                        .await?
                        .is_some()
                    {
                        model.update(db).await?;
                    } else {
                        model.insert(db).await?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
    pub async fn cargar_todos_los_rubros(
        productos: &Vec<Valuable>,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        for i in productos {
            match i {
                V::Rub(a) => {
                    let model = RubDB::ActiveModel {
                        id: Set(*a.1.id()),
                        monto: Set(a.1.monto().copied()),
                        descripcion: Set(a.1.descripcion().to_string()),
                        updated_at: Set(Utc::now().naive_local()),
                        codigo: Set(*a.1.codigo()),
                    };
                    if RubDB::Entity::find_by_id(*a.1.id())
                        .one(db)
                        .await?
                        .is_some()
                    {
                        model.update(db).await?;
                    } else {
                        model.insert(db).await?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
    pub async fn cargar_todos_los_valuables(productos: Vec<Valuable>) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
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
        Db::cargar_todos_los_pesables(&productos, &db).await?;
        Db::cargar_todos_los_rubros(&productos, &db).await?;
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
    ) -> Result<(), DbErr> {
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
