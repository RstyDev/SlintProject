use std::sync::Arc;
use chrono::Utc;
use entity::proveedor;
use sea_orm::{ActiveModelTrait, Database, DbErr, Set};
use serde::{Deserialize, Serialize};

use super::lib::Save;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i64,
    nombre: Arc<str>,
    contacto: Option<i64>,
}

impl Proveedor {
    pub fn new(id: i64, nombre: &str, contacto: Option<i64>) -> Self {
        Proveedor {
            id,
            nombre: Arc::from(nombre),
            contacto,
        }
    }
    pub fn get_nombre(&self) -> Arc<str> {
        Arc::clone(&self.nombre)
    }
    pub fn get_id(&self) -> &i64 {
        &self.id
    }
    pub fn get_contacto(&self) -> &Option<i64> {
        &self.contacto
    }
}
impl Save for Proveedor {
    async fn save(&self) -> Result<(), DbErr> {
        let model = proveedor::ActiveModel {
            id: Set(self.id),
            nombre: Set(self.nombre.to_string()),
            contacto: Set(self.contacto),
            updated_at: Set(Utc::now().naive_utc()),
        };
        let db = Database::connect("sqlite://db/to/db.sqlite?mode=rwc").await?;
        println!("conectado");
        model.insert(&db).await?;
        Ok(())
    }
}
impl ToString for Proveedor {
    fn to_string(&self) -> String {
        let res;
        match self.contacto {
            Some(a) => res = format!("{} {}", self.nombre, a),
            None => res = format!("{}", self.nombre),
        }
        res
    }
}
