use super::{AppError, Res};
use crate::{db::map::BigIntDB, ProveedorFND};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i32,
    nombre: Arc<str>,
    contacto: Option<i32>,
}

impl Proveedor {
    pub async fn new_to_db(
        nombre: &str,
        contacto: Option<i32>,
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
    pub fn build(id: i32, nombre: &str, contacto: Option<i32>) -> Self {
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
    pub fn contacto(&self) -> &Option<i32> {
        &self.contacto
    }
    pub fn to_fnd(&self) -> ProveedorFND {
        let mut prov = ProveedorFND::default();
        prov.contacto = self.contacto.unwrap_or(0);
        prov
    }
    pub fn from_fnd(prov: ProveedorFND) -> Self {
        Proveedor::build(
            prov.id,
            prov.nombre.as_str(),
            match prov.contacto {
                0 => None,
                c => Some(c),
            },
        )
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
