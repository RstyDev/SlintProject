use super::{redondeo, valuable::ValuableTrait, AppError, Res};
use crate::{
    db::map::{BigIntDB, CodeDB},
    RubroFND, SharedString, ValFND,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rubro {
    id: i32,
    codigo: i64,
    monto: Option<f32>,
    descripcion: Arc<str>,
}

impl Rubro {
    pub fn build(id: i32, codigo: i64, monto: Option<f32>, descripcion: &str) -> Rubro {
        Rubro {
            id,
            codigo,
            monto,
            descripcion: Arc::from(descripcion),
        }
    }
    pub async fn new_to_db(
        codigo: i64,
        monto: Option<f32>,
        descripcion: &str,
        db: &Pool<Sqlite>,
    ) -> Res<Rubro> {
        let qres: Option<CodeDB> =
            sqlx::query_as!(CodeDB, r#"select id as "id:_", codigo as "codigo:_", producto as "producto:_", pesable as "pesable:_", rubro as "rubro:_" from codigos where codigo = ?"#, codigo)
                .fetch_optional(db)
                .await?;
        match qres {
            Some(model) => match model.rubro {
                Some(_) => Err(AppError::IncorrectError(String::from("rubro existente"))),
                None => Err(AppError::IncorrectError(String::from(
                    "existe el codigo pero no corresponde a un rubro",
                ))),
            },
            None => {
                let qres = sqlx::query("insert into rubros values (?, ?)")
                    .bind(descripcion)
                    .bind(Utc::now().naive_local())
                    .execute(db)
                    .await?;
                sqlx::query("insert into codigos (codigo, rubro) values (?, ?)")
                    .bind(codigo)
                    .bind(qres.last_insert_rowid())
                    .execute(db)
                    .await?;
                Ok(Rubro::build(
                    qres.last_insert_rowid() as i32,
                    codigo,
                    monto,
                    descripcion,
                ))
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
    pub fn from_fnd(rub: RubroFND) -> Rubro {
        Rubro::build(
            rub.id,
            rub.codigo as i64,
            match rub.monto {
                0.0 => None,
                m => Some(m),
            },
            rub.descripcion.as_str(),
        )
    }
    pub fn to_fnd(&self) -> RubroFND {
        let mut rub = RubroFND::default();
        rub.id = self.id;
        rub.codigo = self.codigo as i32;
        rub.monto = self.monto.unwrap_or(0.0);
        rub.descripcion = SharedString::from(self.descripcion.to_string());
        rub
    }
    pub fn to_val_fnd(&self) -> ValFND {
        let mut val = ValFND::default();
        val.descripcion = SharedString::from(self.descripcion.to_string());
        val.id = self.id;
        val.codigo = self.codigo as i32;
        val.precio = self.monto.unwrap_or(0.0);
        val
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        self.descripcion.to_string()
    }
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from rubros where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                sqlx::query("delete from rubros where id = ?")
                    .bind(self.id)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Rubro"),
                instancia: self.id.to_string(),
            }),
        }
    }
    pub async fn editar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from rubros where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                sqlx::query("update codigos set codigo = ? where rubro = ?")
                    .bind(self.codigo)
                    .bind(self.id)
                    .execute(db)
                    .await?;
                sqlx::query("update rubros set descripcion = ?, updated_at = ? where id = ?")
                    .bind(self.descripcion.as_ref())
                    .bind(Utc::now().naive_local())
                    .bind(self.id)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Rubro"),
                instancia: self.id.to_string(),
            }),
        }
    }
}
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
