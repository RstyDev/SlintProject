use chrono::Utc;
use entity::rubro;
use sea_orm::{ActiveModelTrait, Database, Set, DbErr};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::redondeo;

use super::valuable::ValuableTrait;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rubro {
    pub id: i64,
    pub monto: f64,
    pub descripcion: String,
}

impl Rubro {
    pub fn new(id: i64, monto: f64, descripcion: String) -> Rubro {
        Rubro {
            id,
            monto,
            descripcion,
        }
    }
    pub async fn save(&self) -> Result<(),DbErr> {
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        println!("conectado");
        let model = rubro::ActiveModel {
            id: Set(self.id),
            monto: Set(self.monto),
            descripcion: Set(self.descripcion.clone()),
            updated_at: Set(Utc::now().naive_utc())
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
