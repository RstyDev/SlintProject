use crate::db::Model;
use super::{AppError, Res};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pesable {
    id: i64,
    codigo: i64,
    precio_peso: f32,
    porcentaje: f32,
    costo_kilo: f32,
    descripcion: Arc<str>,
}
impl Pesable {
    pub fn build(
        id: i64,
        codigo: i64,
        precio_peso: f32,
        porcentaje: f32,
        costo_kilo: f32,
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
        db: &Pool<Sqlite>,
        codigo: i64,
        precio_peso: f32,
        porcentaje: f32,
        costo_kilo: f32,
        descripcion: &str,
    ) -> Res<Pesable> {
        let qres:Option<Model>=sqlx::query_as!(Model::Int,"select pesables.id as i64 from codigos inner join pesables on codigos.pesable = pesables.id where codigo = ?",codigo).fetch_optional(db).await?;
        match qres{
            Some(model)=>return Err(AppError::ExistingError {
                objeto: "Pesable".to_string(),
                instancia: codigo.to_string(),
            }),
            None=>{
                let qres=sqlx::query!("insert into pesables values (?, ?, ?, ?, ?, ?)",codigo,precio_peso,porcentaje,costo_kilo,descripcion,Utc::now().naive_local()).execute(db).await;
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
    pub fn precio_peso(&self) -> &f32 {
        &self.precio_peso
    }
    pub fn porcentaje(&self) -> &f32 {
        &self.porcentaje
    }
    pub fn costo_kilo(&self) -> &f32 {
        &self.costo_kilo
    }
    pub fn descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres :Option<Model>=sqlx::query_as!(Model::Int, "select id as i64 from pesables where id = ?",self.id).fetch_optional(db).await?;
        match qres{
            Some(model)=>sqlx::query("delete from pesables where id = ?").bind(self.id).execute(db).await?,
            None=>return Err(AppError::NotFound {
                objeto: String::from("Pesable"),
                instancia: self.id.to_string(),
            })
        }
        Ok(())
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub async fn editar(self, db: &DatabaseConnection) -> Res<()> {
        let mut model = match PesDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => model.into_active_model(),
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Pesable"),
                    instancia: self.id.to_string(),
                })
            }
        };
        if self.precio_peso == self.costo_kilo * (1.0 + self.porcentaje / 100.0) {
            model.precio_peso = Set(self.precio_peso);
        } else {
            return Err(AppError::IncorrectError(String::from(
                "Cálculo de precio incorrecto",
            )));
        }
        model.codigo = Set(self.codigo);
        model.costo_kilo = Set(self.costo_kilo);
        model.descripcion = Set(self.descripcion.to_string());
        model.porcentaje = Set(self.porcentaje);
        model.updated_at = Set(Utc::now().naive_local());
        model.update(db).await?;
        Ok(())
    }
}
