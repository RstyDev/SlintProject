use chrono::Utc;
use entity::producto::{self, ActiveModel, Model};
use entity::{codigo_barras, pesable};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, Set,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use Valuable as V;

type Res<T> = std::result::Result<T, AppError>;

use crate::mods::valuable::Valuable;

use super::error::AppError;
use super::pesable::Pesable;
use super::producto::Producto;
use super::proveedor::Proveedor;
use super::relacion_prod_prov::RelacionProdProv;
use super::rubro::Rubro;
use super::valuable::Presentacion;

pub trait Save {
    async fn save(&self) -> Result<(), DbErr>;
}

pub async fn save<T: Save>(dato: T) -> Result<(), DbErr> {
    dato.save().await
}
// pub async fn save_many<T: Save>(datos: Vec<T>) -> Result<(), DbErr> {
//     for dato in datos {
//         dato.save().await?;
//     }
//     Ok(())
// }

pub fn crear_file<'a>(path: &str, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    println!("Path que se actualiza: {}", path);
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", format!("{}", buf))?;
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

// pub fn push(pr: Producto, path: &String) {
//     let mut prods = Vec::new();

pub fn redondeo(politica: &f64, numero: f64) -> f64 {
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
} //     if let Err(e) = leer_file(&mut prods, path) {
  //         panic!("{}", e);
  //     }
  //     prods.push(pr);
  //     match crear_file(&path, &prods) {
  //         Ok(_) => (),
  //         Err(e) => panic!("No se pudo pushear porque {}", e),
  //     };
  // }
pub async fn get_codigos_db_filtrado(db: &DatabaseConnection, id: i64) -> Res<Vec<i64>> {
    let a = entity::codigo_barras::Entity::find()
        .filter(Condition::all().add(entity::codigo_barras::Column::Producto.eq(id)))
        .all(db)
        .await?;
    // unifica_codes(&mut a);
    Ok(a.iter().map(|x| x.codigo).collect())
}

pub async fn map_model_prod(
    prod: &entity::producto::Model,
    db: &DatabaseConnection,
) -> Res<Producto> {
    let mut parts = prod.presentacion.split(' ');
    let cods = entity::codigo_barras::Entity::find()
        .filter(entity::codigo_barras::Column::Producto.eq(prod.id))
        .all(db)
        .await?
        .iter()
        .map(|c| c.codigo)
        .collect();
    let p1 = parts.next().unwrap();
    let p2 = parts.next().unwrap();
    let presentacion = match p2 {
        "Gr" => Presentacion::Gr(p1.parse()?),
        "Un" => Presentacion::Un(p1.parse()?),
        "Lt" => Presentacion::Lt(p1.parse()?),
        "Ml" => Presentacion::Ml(p1.parse()?),
        "CC" => Presentacion::CC(p1.parse()?),
        "Kg" => Presentacion::Kg(p1.parse()?),
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
fn map_model_rub(rub: &entity::rubro::Model) -> Rubro {
    Rubro::new(rub.id, rub.monto, rub.descripcion.as_str())
}

fn map_model_pes(pes: &entity::pesable::Model) -> Pesable {
    Pesable::new(
        pes.id,
        pes.codigo,
        pes.precio_peso,
        pes.porcentaje,
        pes.costo_kilo,
        pes.descripcion.as_str(),
    )
}
pub fn map_model_prov(prov: &entity::proveedor::Model) -> Proveedor {
    Proveedor::new(prov.id, prov.nombre.as_str(), prov.contacto)
}
pub async fn cargar_todos_los_productos(
    productos: &Vec<Producto>,
    db: &DatabaseConnection,
) -> Result<(), DbErr> {
    for producto in productos {
        let encontrado = entity::producto::Entity::find_by_id(*producto.id())
            .one(db)
            .await?;
        let mut model: ActiveModel;
        let codigo_prod: i64;
        match encontrado {
            Some(m) => {
                codigo_prod = m.id;
                model = m.into();
                model.marca = Set(producto.marca().to_string());
                model.porcentaje = Set(*producto.porcentaje());
                model.precio_de_costo = Set(*producto.precio_de_costo());
                model.precio_de_venta = Set(*producto.precio_de_venta());
                model.presentacion = Set(producto.presentacion().to_string());
                model.tipo_producto = Set(producto.tipo_producto().to_string());
                model.updated_at = Set(Utc::now().naive_local().to_string());
                model.variedad = Set(producto.variedad().to_string());
                model.update(db).await?;
            }
            None => {
                model = producto::ActiveModel {
                    precio_de_venta: Set(*producto.precio_de_venta()),
                    porcentaje: Set(*producto.porcentaje()),
                    precio_de_costo: Set(*producto.precio_de_costo()),
                    tipo_producto: Set(producto.tipo_producto().to_string()),
                    marca: Set(producto.marca().to_string()),
                    variedad: Set(producto.variedad().to_string()),
                    presentacion: Set(producto.presentacion().to_string()),
                    updated_at: Set(Utc::now().naive_local().to_string()),
                    ..Default::default()
                };

                codigo_prod = entity::producto::Entity::insert(model)
                    .exec(db)
                    .await?
                    .last_insert_id;
            }
        }

        entity::codigo_barras::Entity::delete_many()
            .filter(Condition::all().add(entity::codigo_barras::Column::Producto.eq(codigo_prod)))
            .exec(db)
            .await?;

        let codigos_model: Vec<codigo_barras::ActiveModel> = producto
            .codigos_de_barras()
            .iter()
            .map(|x| codigo_barras::ActiveModel {
                codigo: Set(*x),
                producto: Set(codigo_prod),
                ..Default::default()
            })
            .collect();

        if codigos_model.len() > 1 {
            entity::codigo_barras::Entity::insert_many(codigos_model)
                .exec(db)
                .await?;
        } else if codigos_model.len() == 1 {
            entity::codigo_barras::Entity::insert(codigos_model[0].to_owned())
                .exec(db)
                .await?;
        }
    }

    Ok(())
}
pub async fn cargar_todos_los_pesables(
    productos: &Vec<Valuable>,
    db: &DatabaseConnection,
) -> Result<(), DbErr> {
    for i in productos {
        match i {
            V::Pes(a) => {
                let model = pesable::ActiveModel {
                    codigo: Set(*a.1.codigo()),
                    precio_peso: Set(*a.1.precio_peso()),
                    porcentaje: Set(*a.1.porcentaje()),
                    costo_kilo: Set(*a.1.costo_kilo()),
                    descripcion: Set(a.1.descripcion().to_string()),
                    updated_at: Set(Utc::now().naive_local().to_string()),
                    id: Set(*a.1.id()),
                };
                if entity::pesable::Entity::find_by_id(*a.1.id())
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
                let model = entity::rubro::ActiveModel {
                    id: Set(*a.1.id()),
                    monto: Set(*a.1.monto()),
                    descripcion: Set(a.1.descripcion().to_string()),
                    updated_at: Set(Utc::now().naive_local().to_string()),
                };
                if entity::rubro::Entity::find_by_id(*a.1.id())
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
    println!("Guardando productos en DB");
    let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
    cargar_todos_los_productos(
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
    cargar_todos_los_pesables(&productos, &db).await?;
    cargar_todos_los_rubros(&productos, &db).await?;
    Ok(())
}
pub async fn cargar_todos_los_provs(proveedores: Vec<Proveedor>) -> Result<(), DbErr> {
    let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
    for prov in proveedores {
        let model = entity::proveedor::ActiveModel {
            id: Set(*prov.id()),
            updated_at: Set(Utc::now().naive_local().to_string()),
            nombre: Set(prov.nombre().to_string()),
            contacto: Set(prov.contacto().clone()),
        };
        if entity::proveedor::Entity::find_by_id(model.clone().id.unwrap())
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
        if let Some(rel) = entity::relacion_prod_prov::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::relacion_prod_prov::Column::Producto.eq(*x.id_producto()))
                    .add(entity::relacion_prod_prov::Column::Proveedor.eq(*x.id_proveedor())),
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
            let model = entity::relacion_prod_prov::ActiveModel {
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

// pub fn unifica_codes(codes: &mut Vec<entity::codigo_barras::Model>) {
//     let mut i = 0;
//     while i < codes.len() {
//         let act = codes[i].clone();
//         let mut j = i + 1;
//         while j < codes.len() {
//             if act.codigo == codes[j].codigo {
//                 codes.remove(j);
//             } else {
//                 j += 1;
//             }
//         }
//         i += 1;
//     }
// }
