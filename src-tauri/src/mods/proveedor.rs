use super::{AppError, Res};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::fmt::Display;
use std::sync::Arc;
use crate::db::map::BigIntDB;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i32,
    nombre: Arc<str>,
    contacto: Option<i64>,
}

impl Proveedor {
    pub async fn new_to_db(
        nombre: &str,
        contacto: Option<i64>,
        db: &Pool<Sqlite>,
    ) -> Res<Proveedor> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from proveedores where nombre = ?",
            nombre
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => Err(AppError::ExistingError {
                objeto: String::from("Proveedor"),
                instancia: nombre.to_string(),
            }),
            None => {
                let qres = sqlx::query("insert into proveedores values (?, ?, ?)")
                    .bind(nombre)
                    .bind(contacto)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                Ok(Proveedor::build(
                    qres.last_insert_rowid() as i32,
                    nombre,
                    contacto,
                ))
            }
        }
    }
    pub fn build(id: i32, nombre: &str, contacto: Option<i64>) -> Self {
        Proveedor {
            id,
            nombre: Arc::from(nombre),
            contacto,
        }
    }
    pub fn nombre(&self) -> Arc<str> {
        Arc::clone(&self.nombre)
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn contacto(&self) -> &Option<i64> {
        &self.contacto
    }
}
impl Display for Proveedor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res;
        match self.contacto {
            Some(a) => res = format!("{} {a}", self.nombre),
            None => res = self.nombre.to_string(),
        }
        write!(f, "{}", res)
    }
}
