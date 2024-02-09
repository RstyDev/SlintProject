use chrono::Utc;
type Res<T> = std::result::Result<T, AppError>;
use entity::rubro;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{
    error::AppError,
    lib::{redondeo, Save},
    valuable::ValuableTrait,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rubro {
    id: i64,
    codigo: i64,
    monto: Option<f64>,
    descripcion: Arc<str>,
}

impl Rubro {
    pub fn new(id: i64, codigo: i64, monto: Option<f64>, descripcion: Arc<str>) -> Rubro {
        Rubro {
            id,
            codigo,
            monto,
            descripcion,
        }
    }
    pub async fn new_to_db(
        codigo: i64,
        monto: Option<f64>,
        descripcion: &str,
        db: &DatabaseConnection,
    ) -> Res<Rubro> {
        let model = entity::rubro::ActiveModel {
            codigo: Set(codigo),
            monto: Set(monto),
            descripcion: Set(descripcion.to_string()),
            updated_at: Set(Utc::now().naive_local()),
            ..Default::default()
        };
        let res = entity::rubro::Entity::insert(model).exec(db).await?;
        Ok(Rubro {
            id: res.last_insert_id,
            codigo,
            monto,
            descripcion: Arc::from(descripcion),
        })
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn monto(&self) -> Option<&f64> {
        self.monto.as_ref()
    }
    pub fn codigo(&self) -> &i64 {
        &self.codigo
    }
    pub fn descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
}
impl Save for Rubro {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        println!("conectado");
        let model = rubro::ActiveModel {
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
impl ValuableTrait for Rubro {
    fn redondear(&self, politica: &f64) -> Rubro {
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
