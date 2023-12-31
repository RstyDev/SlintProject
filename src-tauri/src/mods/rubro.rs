use entity::rubro;
use sea_orm::{Database, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};

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
    pub async fn save(&self) -> Result<(), String> {
        match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
            Ok(db) => {
                println!("conectado");
                let model = rubro::ActiveModel {
                    id: Set(self.id),
                    monto: Set(self.monto),
                    descripcion: Set(self.descripcion.clone()),
                };
                if let Err(e) = model.insert(&db).await {
                    return Err(e.to_string());
                }
            }
            Err(e) => return Err(e.to_string()),
        }

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
