use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};
use serde::Serialize;
use std::sync::Arc;

use super::error::AppError;
#[derive(Debug, Serialize, Clone)]
pub struct Vendedor {
    id: i64,
    nombre: Arc<str>,
}

impl Default for Vendedor {
    fn default() -> Self {
        Vendedor {
            id: 0,
            nombre: Arc::from("Vendedor 1"),
        }
    }
}

impl Vendedor {
    pub fn new(id: i64, nombre: Arc<str>) -> Vendedor {
        Vendedor { id, nombre }
    }
    pub async fn get_or_def(db: Arc<DatabaseConnection>) -> Result<Vendedor, AppError> {
        let vend = entity::vendedor::Entity::find()
            .order_by_desc(entity::vendedor::Column::Id)
            .one(db.as_ref())
            .await?;
        let vendedor = match vend {
            Some(a) => Vendedor {
                id: a.id,
                nombre: Arc::from(a.nombre),
            },
            None => {
                let vend = Vendedor {
                    id: 0,
                    nombre: Arc::from("Vendedor 1"),
                };
                let model = entity::vendedor::ActiveModel {
                    id: Set(vend.id),
                    nombre: Set(vend.nombre.to_string()),
                };
                model.insert(db.as_ref()).await?;
                vend
            }
        };

        Ok(vendedor)
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn nombre(&self) -> &str {
        &self.nombre
    }
}
