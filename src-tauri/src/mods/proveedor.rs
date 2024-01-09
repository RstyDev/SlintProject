use std::error::Error;

use entity::proveedor;
use sea_orm::{ActiveModelTrait, Database, Set, DbErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i64,
    nombre: String,
    contacto: Option<i64>,
}

impl Proveedor {
    pub fn new(id: i64, nombre: String, contacto: Option<i64>) -> Self {
        Proveedor {
            id,
            nombre,
            contacto,
        }
    }
    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }
    pub fn get_id(&self) -> &i64 {
        &self.id
    }
    pub fn get_contacto(&self) -> &Option<i64> {
        &self.contacto
    }
    pub async fn save(&self) -> Result<(),DbErr> {
        let model = proveedor::ActiveModel {
            id: Set(self.id),
            nombre: Set(self.nombre.clone()),
            contacto: Set(self.contacto),
        };
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
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
