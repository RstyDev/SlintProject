use super::{AppError, Res};
use crate::{ValFND,SharedString,db::map::BigIntDB};
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
        let qres:Option<BigIntDB>=sqlx::query_as!(BigIntDB,"select pesables.id as int from codigos inner join pesables on codigos.pesable = pesables.id where codigo = ?",codigo).fetch_optional(db).await?;
        match qres {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Pesable".to_string(),
                    instancia: codigo.to_string(),
                })
            }
            None => {
                let qres = sqlx::query("insert into pesables values (?, ?, ?, ?, ?, ?)")
                    .bind(codigo)
                    .bind(precio_peso)
                    .bind(porcentaje)
                    .bind(costo_kilo)
                    .bind(descripcion)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                Ok(Pesable {
                    id: qres.last_insert_rowid(),
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
    pub fn to_fnd(&self)->ValFND{
        let mut val=ValFND::default();
        val.descripcion = SharedString::from(self.descripcion.to_string());
        val.id = self.id as i32;
        val.codigo = self.codigo as i32;
        val.precio = self.precio_peso;
        val
    }
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from pesables where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                sqlx::query("delete from pesables where id = ?")
                    .bind(self.id)
                    .execute(db)
                    .await?;
            }
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Pesable"),
                    instancia: self.id.to_string(),
                })
            }
        }
        Ok(())
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub async fn editar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from pesables where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                if self.precio_peso == self.costo_kilo * (1.0 + self.porcentaje / 100.0) {
                    sqlx::query(
                        "update pesables set precio_peso = ?, costo_kilo = ?,
                    descripcion =?,
                    porcentaje=?,
                    updated_at=? where id = ?",
                    )
                    .bind(self.precio_peso)
                    .bind(self.costo_kilo)
                    .bind(self.descripcion.as_ref())
                    .bind(self.porcentaje)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                    Ok(())
                } else {
                    Err(AppError::IncorrectError(String::from(
                        "Cálculo de precio incorrecto",
                    )))
                }
            }
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Pesable"),
                    instancia: self.id.to_string(),
                })
            }
        }
    }
}
