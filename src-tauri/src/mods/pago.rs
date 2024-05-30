use rand::random;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tauri::async_runtime;

use crate::db::Model;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i64,
}

impl MedioPago {
    pub fn build(medio: &str, id: i64) -> MedioPago {
        MedioPago {
            medio: Arc::from(medio),
            id,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn desc(&self) -> Arc<str> {
        Arc::clone(&self.medio)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    int_id: i64,
    medio_pago: MedioPago,
    monto: f32,
    pagado: f32,
}

impl Pago {
    pub fn new(medio_pago: MedioPago, monto: f32, pagado: Option<f32>) -> Pago {
        let int_id = random();
        Pago {
            medio_pago,
            monto,
            int_id,
            pagado: match pagado {
                Some(a) => a,
                None => monto,
            },
        }
    }
    pub fn build(int_id: i64, medio_pago: MedioPago, monto: f32, pagado: f32) -> Pago {
        Pago {
            int_id,
            medio_pago,
            monto,
            pagado,
        }
    }
    pub fn medio_pago(&self) -> &MedioPago {
        &self.medio_pago
    }
    pub fn medio(&self) -> Arc<str> {
        Arc::clone(&self.medio_pago.medio)
    }
    pub fn monto(&self) -> f32 {
        self.monto
    }
    pub fn id(&self) -> i64 {
        self.int_id
    }
    pub fn pagado(&self) -> &f32 {
        &self.pagado
    }
    pub fn def(db: &Pool<Sqlite>) -> Self {
        match async_runtime::block_on(medio_from_db("Efectivo", db)) {
            Model::MedioPago { id, medio } => {
                let medio_pago = MedioPago {
                    medio: Arc::from(medio),
                    id,
                };
                let int_id = random();
                Pago {
                    medio_pago,
                    monto: 0.0,
                    int_id,
                    pagado: 0.0,
                }
            }
            _ => panic!("Se esperaba MedioPago"),
        }
    }
}

pub async fn medio_from_db(medio: &str, db: &Pool<Sqlite>) -> Model {
    let model: sqlx::Result<Option<Model>> = sqlx::query_as!(
        Model::MedioPago,
        "select * from medios_pago where medio = ? ",
        medio
    )
    .fetch_optional(db)
    .await;
    model.unwrap().unwrap()
}
