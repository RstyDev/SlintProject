use std::error::Error;


use chrono::Utc;
use entity::pesable;
use sea_orm::{Database, Set, ActiveModelTrait, DbErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pesable {
    pub id: i64,
    pub codigo: i64,
    pub precio_peso: f64,
    pub porcentaje: f64,
    pub costo_kilo: f64,
    pub descripcion: String,
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
    pub async fn save(&self) -> Result<(),DbErr> {
        let db= Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
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
