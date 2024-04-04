use super::lib::Save;
use entity::{medio_pago::Model, pago};
use rand::random;
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DbErr, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use std::sync::Arc;
use tauri::async_runtime;
#[derive(Debug, Clone, Serialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i64,
}
impl MedioPago {
    pub fn new(medio: &str, id: i64) -> MedioPago {
        MedioPago {
            medio: Arc::from(medio),
            id,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct Pago {
    int_id: u32,
    medio_pago: MedioPago,
    monto: f64,
}

impl Pago {
    pub fn new(medio_pago: MedioPago, monto: f64) -> Pago {
        let int_id = random();
        Pago {
            medio_pago,
            monto,
            int_id,
        }
    }
    pub fn medio_pago(&self) -> &MedioPago {
        &self.medio_pago
    }
    pub fn medio(&self) -> Arc<str> {
        Arc::clone(&self.medio_pago.medio)
    }
    pub fn monto(&self) -> f64 {
        self.monto
    }
    pub fn id(&self)->u32{
        self.int_id
    }
}
impl Save for Pago {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        let medio_id = entity::medio_pago::Entity::find()
            .filter(entity::medio_pago::Column::Medio.eq(self.medio().to_string()))
            .one(&db)
            .await?
            .unwrap();
        let model = pago::ActiveModel {
            medio_pago: Set(medio_id.id),
            monto: Set(self.monto),
            ..Default::default()
        };
        model.insert(&db).await?;
        Ok(())
    }
}

pub async fn medio_from_db(medio: &str) -> Model {
    let db = Database::connect("sqlite://db.sqlite?mode=ro")
        .await
        .unwrap();
    entity::medio_pago::Entity::find()
        .filter(entity::medio_pago::Column::Medio.eq(medio))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
}
impl Default for Pago {
    fn default() -> Self {
        let res = async_runtime::block_on(medio_from_db("Efectivo"));
        let medio_pago = MedioPago {
            medio: Arc::from(res.medio),
            id: res.id,
        };
        let int_id = random();
        Pago {
            medio_pago,
            monto: 0.0,
            int_id,
        }
    }
}
