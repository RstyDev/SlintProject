use super::{AppError, Res};
use crate::db::map::BigIntDB;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::fmt::Display;
use std::sync::Arc;

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
        db: &Pool<Sqlite>,
    ) -> Res<User> {
        let id_2 = id.as_ref();
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from users where user_id = ?",
            id_2
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => Err(AppError::IncorrectError(String::from("Usuario existente"))),
            None => {
                sqlx::query("insert into users values (?, ?, ?, ?)")
                    .bind(id.as_ref())
                    .bind(nombre.as_ref())
                    .bind(pass)
                    .bind(rango)
                    .execute(db)
                    .await?;
                Ok(User::build(id, nombre, pass, rango))
            }
        }
    }
    pub fn build(id: Arc<str>, nombre: Arc<str>, pass: i64, rango: &str) -> User {
        let rango = match rango {
            "Admin" => Rango::Admin,
            "Cajero" => Rango::Cajero,
            _ => panic!("No existe"),
        };
        User {
            id,
            pass,
            rango,
            nombre,
        }
    }
    pub fn rango(&self) -> &Rango {
        &self.rango
    }
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    //pub fn pass(&self) -> &i64 {
    //        &self.pass
    //}
    pub fn nombre(&self) -> Arc<str> {
        Arc::clone(&self.nombre)
    }
}
impl Display for Rango {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Rango::Admin => String::from("Admin"),
            Rango::Cajero => String::from("Cajero"),
        };
        write!(f, "{}", str)
    }
}
