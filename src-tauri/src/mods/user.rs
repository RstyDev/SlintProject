use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::error::AppError;
type Res<T> = std::result::Result<T, AppError>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Arc<str>,
    nombre: Arc<str>,
    pass: i64,
    rango: Rango,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Rango {
    Admin,
    Cajero,
}
impl User {
    pub async fn new_to_db(
        id: Arc<str>,
        nombre: Arc<str>,
        pass: i64,
        rango: &str,
        db: &DatabaseConnection,
    ) -> Res<User> {
        let rango2 = match rango {
            "Admin" => Rango::Admin,
            "Cajero" => Rango::Cajero,
            _ => panic!("No existe"),
        };
        match entity::user::Entity::find()
            .filter(entity::user::Column::UserId.eq(id.as_ref()))
            .one(db)
            .await?
        {
            Some(_) => Ok(User {
                id,
                nombre,
                pass,
                rango: rango2,
            }),
            None => {
                entity::user::ActiveModel {
                    user_id: Set(id.to_string()),
                    pass: Set(pass),
                    rango: Set(rango.to_string()),
                    ..Default::default()
                }
                .insert(db)
                .await?;
                Ok(User {
                    id,
                    nombre,
                    pass,
                    rango: rango2,
                })
            }
        }
    }
    pub fn new(id: Arc<str>, nombre: Arc<str>, pass: i64, rango: &str) -> User {
        let rango = match rango {
            "Admin" => Rango::Admin,
            "Cajero" => Rango::Cajero,
            _ => panic!("No existe"),
        };
        User {
            id,
            pass,
            rango,
            nombre: Arc::from(nombre),
        }
    }
    pub fn rango(&self) -> &Rango {
        &self.rango
    }
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    pub fn pass(&self) -> &i64 {
        &self.pass
    }
    pub fn nombre(&self) -> &str {
        self.nombre.as_ref()
    }
}
impl ToString for Rango {
    fn to_string(&self) -> String {
        match self {
            Rango::Admin => String::from("Admin"),
            Rango::Cajero => String::from("Cajero"),
        }
    }
}
