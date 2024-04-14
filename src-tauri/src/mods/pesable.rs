use super::{error::AppError, lib::Save};
use chrono::Utc;
type Res<T> = std::result::Result<T, AppError>;
use entity::prelude::PesDB;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pesable {
    id: i64,
    codigo: i64,
    precio_peso: f64,
    porcentaje: f64,
    costo_kilo: f64,
    descripcion: Arc<str>,
}
impl Pesable {
    pub fn new(
        id: i64,
        codigo: i64,
        precio_peso: f64,
        porcentaje: f64,
        costo_kilo: f64,
        descripcion: &str,
    ) -> Pesable {
        Pesable {
            id,
            codigo,
            precio_peso,
            porcentaje,
            costo_kilo,
            descripcion: Arc::from(descripcion),
        }
    }
    pub async fn new_to_db(
        db: &DatabaseConnection,
        codigo: i64,
        precio_peso: f64,
        porcentaje: f64,
        costo_kilo: f64,
        descripcion: &str,
    ) -> Res<Pesable> {
        match PesDB::Entity::find()
            .filter(PesDB::Column::Codigo.eq(codigo))
            .one(db)
            .await?
        {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Pesable".to_string(),
                    instancia: format!("{}", codigo),
                })
            }
            None => {
                let model = PesDB::ActiveModel {
                    codigo: Set(codigo),
                    precio_peso: Set(precio_peso),
                    porcentaje: Set(porcentaje),
                    costo_kilo: Set(costo_kilo),
                    descripcion: Set(descripcion.to_string()),
                    updated_at: Set(Utc::now().naive_local()),
                    ..Default::default()
                };
                let res = PesDB::Entity::insert(model).exec(db).await?;
                Ok(Pesable {
                    id: res.last_insert_id,
                    codigo,
                    precio_peso,
                    porcentaje,
                    costo_kilo,
                    descripcion: Arc::from(descripcion),
                })
            }
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn codigo(&self) -> &i64 {
        &self.codigo
    }
    pub fn precio_peso(&self) -> &f64 {
        &self.precio_peso
    }
    pub fn porcentaje(&self) -> &f64 {
        &self.porcentaje
    }
    pub fn costo_kilo(&self) -> &f64 {
        &self.costo_kilo
    }
    pub fn descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
    pub async fn eliminar(self, db: &DatabaseConnection)->Res<()>{
        let model=match PesDB::Entity::find_by_id(self.id).one(db).await?{
            Some(model) => model.into_active_model(),
            None => return Err(AppError::NotFound { objeto: String::from("Pesable"), instancia: format!("{}",self.id) }),
        };
        model.delete(db).await?;
        Ok(())
    }
}

impl Save for Pesable {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        println!("conectado");
        let model = PesDB::ActiveModel {
            id: Set(self.id),
            codigo: Set(self.codigo),
            precio_peso: Set(self.precio_peso),
            porcentaje: Set(self.porcentaje),
            costo_kilo: Set(self.costo_kilo),
            descripcion: Set(self.descripcion.to_string()),
            updated_at: Set(Utc::now().naive_local()),
        };
        model.insert(&db).await?;
        Ok(())
    }
}
