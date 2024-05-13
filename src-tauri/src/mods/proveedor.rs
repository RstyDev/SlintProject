use chrono::Utc;
use entity::prelude::ProvDB;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{AppError, Mapper, Res};

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
        db: &DatabaseConnection,
    ) -> Res<Proveedor> {
        match ProvDB::Entity::find()
            .filter(ProvDB::Column::Nombre.eq(nombre))
            .one(db)
            .await?
        {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: String::from("Proveedor"),
                    instancia: nombre.to_string(),
                })
            }
            None => {
                let model = ProvDB::ActiveModel {
                    updated_at: Set(Utc::now().naive_local()),
                    nombre: Set(nombre.to_string()),
                    contacto: Set(contacto),
                    ..Default::default()
                }
                .insert(db)
                .await?;
                Ok(Mapper::map_model_prov(&model))
            }
        }
    }
    pub fn new(id: i32, nombre: &str, contacto: Option<i64>) -> Self {
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
impl ToString for Proveedor {
    fn to_string(&self) -> String {
        let res;
        match self.contacto {
            Some(a) => res = format!("{} {a}", self.nombre),
            None => res = self.nombre.to_string(),
        }
        res
    }
}
