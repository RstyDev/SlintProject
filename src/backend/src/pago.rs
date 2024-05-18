use super::get_thread;
use entity::prelude::MedioDB;
use rand::random;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize,PartialEq, Deserialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i32,
}

impl MedioPago {
    pub fn new(medio: &str, id: i32) -> MedioPago {
        MedioPago {
            medio: Arc::from(medio),
            id,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn desc(&self) -> Arc<str> {
        Arc::clone(&self.medio)
    }
}
#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub struct Pago {
    int_id: u32,
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
    pub fn medio_pago(&self) -> &MedioPago {
        &self.medio_pago
    }
    pub fn medio(&self) -> Arc<str> {
        Arc::clone(&self.medio_pago.medio)
    }
    pub fn monto(&self) -> f32 {
        self.monto
    }
    pub fn id(&self) -> u32 {
        self.int_id
    }
    pub fn pagado(&self) -> &f32 {
        &self.pagado
    }
    pub fn def(db: &DatabaseConnection) -> Self {
        let res = get_thread().block_on(medio_from_db("Efectivo", db));
        let medio_pago = MedioPago {
            medio: Arc::from(res.medio),
            id: res.id,
        };
        let int_id = random();
        Pago {
            medio_pago,
            monto: 0.0,
            int_id,
            pagado: 0.0,
        }
    }
}

pub async fn medio_from_db(medio: &str, db: &DatabaseConnection) -> MedioDB::Model {
    MedioDB::Entity::find()
        .filter(MedioDB::Column::Medio.eq(medio))
        .one(db)
        .await
        .unwrap()
        .unwrap()
}
