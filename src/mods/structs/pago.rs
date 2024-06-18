use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct MedioPago {
    medio: String,
    id: i64,
}

impl MedioPago {
    pub fn build(medio: &str, id: i64) -> MedioPago {
        MedioPago {
            medio: medio.to_string(),
            id,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn desc(&self) -> String {
        self.medio.clone()
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
    pub fn medio(&self) -> String {
        self.medio_pago.medio.clone()
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

}