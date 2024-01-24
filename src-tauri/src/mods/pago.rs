use std::sync::Arc;
use entity::pago;
use sea_orm::{Database, Set, ActiveModelTrait, DbErr};
use serde::Serialize;

use super::lib::Save;

#[derive(Debug, Clone,  Serialize)]
pub struct Pago {
    medio_pago: Arc<str>,
    monto: f64,
}
impl Pago {
    pub fn new(medio_pago: &str, monto: f64) -> Pago {
        Pago { medio_pago: Arc::from(medio_pago), monto }
    }
    pub fn get_medio(&self)->Arc<str>{
        Arc::clone(&self.medio_pago)
    }
    pub fn get_monto(&self)->f64{
        self.monto
    }
    
}
impl Save for Pago{
    async fn save(&self) -> Result<(),DbErr> {
        let db= Database::connect("sqlite://db/to/db.sqlite?mode=rwc").await?;     
        println!("conectado");
        let model = pago::ActiveModel {
            medio_pago: Set(self.medio_pago.to_string()),
            monto: Set(self.monto),
                ..Default::default()
        };
        model.insert(&db).await?;
        Ok(())
    }
}
impl Default for Pago{
    fn default() -> Self {
        Pago { medio_pago: Arc::from("Efectivo"), monto: 0.0 }
    }
}