use crate::db::map::BigIntDB;
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Pool, Sqlite};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use Valuable as V;

use super::{AppError, Pesable, Producto, Proveedor, Res, Rubro, User};
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

impl Db {
    pub async fn eliminar_usuario(user: User, db: &Pool<Sqlite>) -> Res<()> {
        let id = user.id();
        let qres: Option<BigIntDB> =
            sqlx::query_as!(BigIntDB, "select id as int from users where id = ?", id)
                .fetch_optional(db)
                .await?;
        match qres {
            Some(model) => {
                sqlx::query("delete from users where id = ?")
                    .bind(model.int)
                    .execute(db)
                    .await?;
                Ok(())
            }
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
        if productos.len() > 0 {
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
        }
        Ok(())
    }

    pub async fn cargar_todos_los_pesables(
        pesables: Vec<&Pesable>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        if pesables.len() > 0 {
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
                    .bind(pesable.descripcion().to_string())
                    .bind(Utc::now().naive_local());
                codigos_query = codigos_query.bind(*pesable.codigo()).bind(pesable.id());
            }
            pesables_query.execute(db).await?;
            codigos_query.execute(db).await?;
        }
        Ok(())
    }
    pub async fn cargar_todos_los_rubros(
        rubros: Vec<&Rubro>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        if rubros.len() > 0 {
            let mut rubros_inicio =
                String::from("insert into rubros (id, descripcion, updated_at) values (?, ?, ?)");
            let mut codigos_inicio =
                String::from("insert into codigos (codigo, rubro) values (?, ?)");
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
        }
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
        Db::cargar_todos_los_rubros(
            productos
                .iter()
                .filter_map(|val| match val {
                    V::Rub((_, rub)) => Some(rub),
                    _ => None,
                })
                .collect::<Vec<&Rubro>>(),
            &db,
        )
        .await?;
        Ok(())
    }
    pub async fn cargar_todos_los_provs(
        proveedores: Vec<Proveedor>,
        db: &Pool<Sqlite>,
    ) -> Result<(), AppError> {
        let mut query = String::from("insert into proveedores values (?, ?, ?, ?)"); //id, nombre, contacto, updated
        let row = ", (?, ?, ?, ?)";
        for _ in 0..proveedores.len() {
            query.push_str(row);
        }
        let mut sql = sqlx::query(query.as_str());
        for prov in proveedores {
            sql = sql
                .bind(*prov.id())
                .bind(prov.nombre().to_string())
                .bind(*prov.contacto())
                .bind(Utc::now().naive_local());
        }
        sql.execute(db).await?;
        Ok(())
    }

    // pub async fn cargar_todas_las_relaciones_prod_prov(
    //     relaciones: Vec<RelacionProdProv>,
    //     db: &Pool<Sqlite>,
    // ) -> Result<(), AppError> {
    //     let mut query = String::from("insert into relacion_prod_prov values (?, ?, ?)"); //producto, proveedor, codigo
    //     let row = ", (?, ?, ?)";
    //     for _ in 0..relaciones.len() {
    //         query.push_str(row);
    //     }
    //     let mut sql = sqlx::query(query.as_str());
    //     for rel in relaciones {
    //         sql = sql
    //             .bind(*rel.id_producto())
    //             .bind(*rel.id_proveedor())
    //             .bind(rel.codigo_interno());
    //     }
    //     sql.execute(db).await?;
    //     Ok(())
    // }
}
