use chrono::Utc;
use entity::pesable;
use sea_orm::{ActiveModelTrait, Database, DbErr, Set};
use serde::{Deserialize, Serialize};

use super::lib::Save;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pesable {
    id: i64,
    codigo: i64,
    precio_peso: f64,
    porcentaje: f64,
    costo_kilo: f64,
    descripcion: String,
}
impl Pesable {
    pub fn new(
        id: i64,
        codigo: i64,
        precio_peso: f64,
        porcentaje: f64,
        costo_kilo: f64,
        descripcion: String,
    ) -> Pesable {
        Pesable {
            id,
            codigo,
            precio_peso,
            porcentaje,
            costo_kilo,
            descripcion,
        }
    }
    pub fn get_id(&self) -> &i64 {
        &self.id
    }
    pub fn get_codigo(&self) -> &i64 {
        &self.codigo
    }
    pub fn get_precio_peso(&self) -> &f64 {
        &self.precio_peso
    }
    pub fn get_porcentaje(&self) -> &f64 {
        &self.porcentaje
    }
    pub fn get_costo_kilo(&self) -> &f64 {
        &self.costo_kilo
    }
    pub fn get_descripcion(&self) -> &String {
        &self.descripcion
    }
}

impl Save for Pesable {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        println!("conectado");
        let model = pesable::ActiveModel {
            id: Set(self.id),
            codigo: Set(self.codigo),
            precio_peso: Set(self.precio_peso),
            porcentaje: Set(self.porcentaje),
            costo_kilo: Set(self.costo_kilo),
            descripcion: Set(self.descripcion.clone()),
            updated_at: Set(Utc::now().naive_utc()),
        };
        model.insert(&db).await?;
        Ok(())
    }
}
