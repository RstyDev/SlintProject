use crate::{db::map::MedioPagoDB, MedioPagoFND, PagoFND, SharedString};
use rand::random;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i32,
}

impl MedioPago {
    pub fn build(medio: &str, id: i32) -> MedioPago {
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
    pub fn to_fnd(&self) -> MedioPagoFND {
        let mut st = MedioPagoFND::default();
        st.medio = SharedString::from(self.medio.as_ref());
        st.id = self.id;
        st
    }
    pub fn from_fnd(medio: MedioPagoFND) -> Self {
        MedioPago::build(medio.medio.as_str(), medio.id)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    int_id: i32,
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
            pagado: pagado.unwrap_or(monto),
        }
    }
    pub fn build(int_id: i32, medio_pago: MedioPago, monto: f32, pagado: f32) -> Pago {
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
    pub fn id(&self) -> i32 {
        self.int_id
    }
    pub fn pagado(&self) -> &f32 {
        &self.pagado
    }
    pub fn to_fnd(&self) -> PagoFND {
        let mut st = PagoFND::default();
        st.int_id = self.int_id;
        st.monto = self.monto;
        st.pagado = self.pagado;
        st.medio_pago = self.medio_pago.to_fnd();
        st
    }
    pub fn from_fnd(pago: PagoFND) -> Self {
        Pago::build(
            pago.int_id,
            MedioPago::from_fnd(pago.medio_pago),
            pago.monto,
            pago.pagado,
        )
    }
    pub fn def(db: &Pool<Sqlite>) -> Self {
        let medio = Runtime::new()
            .unwrap()
            .block_on(async { medio_from_db("Efectivo", db).await });

        let medio_pago = MedioPago {
            medio: Arc::from(medio.medio),
            id: medio.id,
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

pub async fn medio_from_db(medio: &str, db: &Pool<Sqlite>) -> MedioPagoDB {
    let model: sqlx::Result<Option<MedioPagoDB>> = sqlx::query_as!(
        MedioPagoDB,
        r#"select id as "id:_", medio from medios_pago where medio = ? "#,
        medio
    )
    .fetch_optional(db)
    .await;
    model.unwrap().unwrap()
}
