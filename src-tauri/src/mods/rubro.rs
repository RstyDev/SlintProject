use std::sync::Arc;
use chrono::Utc;
use entity::rubro;
use sea_orm::{ActiveModelTrait, Database, DbErr, Set};
use serde::{Deserialize, Serialize};

use crate::redondeo;

use super::{lib::Save, valuable::ValuableTrait};

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Rubro {
    id: i64,
    monto: f64,
    descripcion: Arc<str>,
}

impl Rubro {
    pub fn new(id: i64, monto: f64, descripcion: &str) -> Rubro {
        Rubro {
            id,
            monto,
            descripcion: Arc::from(descripcion),
        }
    }
    pub fn get_id(&self) -> &i64 {
        &self.id
    }
    pub fn get_monto(&self) -> &f64 {
        &self.monto
    }
    pub fn get_descripcion(&self) -> Arc<str> {
        Arc::clone(&self.descripcion)
    }
}
impl Save for Rubro {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db/to/db.sqlite?mode=rwc").await?;
        println!("conectado");
        let model = rubro::ActiveModel {
            id: Set(self.id),
            monto: Set(self.monto),
            descripcion: Set(self.descripcion.to_string()),
            updated_at: Set(Utc::now().naive_utc()),
        };
        model.insert(&db).await?;
        Ok(())
    }
}
impl ValuableTrait for Rubro {
    fn redondear(&self, politica: f64) -> Rubro {
        Rubro {
            id: self.id,
            monto: redondeo(politica, self.monto),
            descripcion: self.descripcion.clone(),
        }
    }
}
