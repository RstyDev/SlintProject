use chrono::Utc;
<<<<<<< HEAD
use entity::prelude::RubDB;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
=======
type Res<T> = std::result::Result<T, AppError>;
use entity::prelude::RubDB;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

<<<<<<< HEAD
use super::{redondeo, valuable::ValuableTrait, AppError, Res};
=======
use super::{
    error::AppError,
    lib::{redondeo, Save},
    valuable::ValuableTrait,
};
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rubro {
    id: i32,
    codigo: i64,
    monto: Option<f32>,
    descripcion: Arc<str>,
}

impl Rubro {
    pub fn new(id: i32, codigo: i64, monto: Option<f32>, descripcion: Arc<str>) -> Rubro {
        Rubro {
            id,
            codigo,
            monto,
            descripcion,
        }
    }
    pub async fn new_to_db(
        codigo: i64,
        monto: Option<f32>,
        descripcion: &str,
        db: &DatabaseConnection,
    ) -> Res<Rubro> {
        match RubDB::Entity::find()
            .filter(RubDB::Column::Codigo.eq(codigo))
            .one(db)
            .await?
        {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: String::from("Rubro"),
<<<<<<< HEAD
                    instancia: codigo.to_string(),
=======
                    instancia: format!("{}", codigo),
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
                })
            }
            None => {
                let model = RubDB::ActiveModel {
                    codigo: Set(codigo),
                    monto: Set(monto),
                    descripcion: Set(descripcion.to_string()),
                    updated_at: Set(Utc::now().naive_local()),
                    ..Default::default()
                };
                let res = RubDB::Entity::insert(model).exec(db).await?;
                Ok(Rubro {
                    id: res.last_insert_id,
                    codigo,
                    monto,
                    descripcion: Arc::from(descripcion),
                })
            }
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn monto(&self) -> Option<&f32> {
        self.monto.as_ref()
    }
    pub fn codigo(&self) -> &i64 {
        &self.codigo
    }
    pub fn descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
<<<<<<< HEAD
    #[cfg(test)]
    pub fn desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub async fn eliminar(self, db: &DatabaseConnection) -> Res<()> {
        let model = match RubDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => model.into_active_model(),
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Rubro"),
                    instancia: self.id.to_string(),
                })
            }
=======
    pub async fn eliminar(self, db:&DatabaseConnection) -> Res<()>{
        let model= match RubDB::Entity::find_by_id(self.id).one(db).await?{
            Some(model)=>model.into_active_model(),
            None=>return Err(AppError::NotFound { objeto: String::from("Rubro"), instancia: format!("{}",self.id) }),
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
        };
        model.delete(db).await?;
        Ok(())
    }
<<<<<<< HEAD
    pub async fn editar(self, db: &DatabaseConnection) -> Res<()> {
        let mut model = match RubDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => model.into_active_model(),
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Rubro"),
                    instancia: self.id.to_string(),
                })
            }
=======
    pub async fn editar(self, db: &DatabaseConnection)->Res<()>{
        let mut model= match RubDB::Entity::find_by_id(self.id).one(db).await?{
            Some(model)=>model.into_active_model(),
            None=>return Err(AppError::NotFound { objeto: String::from("Rubro"), instancia: format!("{}",self.id) }),
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
        };
        model.codigo = Set(self.codigo);
        model.descripcion = Set(self.descripcion.to_string());
        model.updated_at = Set(Utc::now().naive_local());
        model.update(db).await?;
        Ok(())
    }
}
<<<<<<< HEAD
=======
impl Save for Rubro {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        println!("conectado");
        let model = RubDB::ActiveModel {
            id: Set(self.id),
            monto: Set(self.monto),
            descripcion: Set(self.descripcion.to_string()),
            updated_at: Set(Utc::now().naive_local()),
            codigo: Set(self.codigo),
        };
        model.insert(&db).await?;
        Ok(())
    }
}
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
impl ValuableTrait for Rubro {
    fn redondear(&self, politica: &f32) -> Rubro {
        match &self.monto {
            Some(a) => Rubro {
                id: self.id,
                codigo: self.codigo,
                monto: Some(redondeo(politica, *a)),
                descripcion: self.descripcion.clone(),
            },
            None => self.clone(),
        }
    }
}
