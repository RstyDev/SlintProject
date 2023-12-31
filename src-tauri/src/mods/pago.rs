use entity::pago;
use sea_orm::{Database, Set, ActiveModelTrait};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Pago {
    medio_pago: String,
    monto: f64,
}
impl Pago {
    pub fn new(medio_pago: String, monto: f64) -> Pago {
        Pago { medio_pago, monto }
    }
    pub fn get_monto(&self)->f64{
        self.monto
    }
    pub async fn save(&self) -> Result<(), String> {
        match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
            Ok(db) => {
                println!("conectado");
                let model = pago::ActiveModel {
                    medio_pago: Set(self.medio_pago.clone()),
                    monto: Set(self.monto),
                        ..Default::default()
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