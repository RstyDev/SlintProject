
// use std::error::Error;
// use entity::pago;
// use sea_orm::{Database, Set, ActiveModelTrait};
// type Result<T>=std::result::Result<T,Box<dyn Error>>;
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
    pub fn get_medio(&self)->&String{
        &self.medio_pago
    }
    pub fn get_monto(&self)->f64{
        self.monto
    }
    // pub async fn save(&self) -> Result<()> {
    //     let db= Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;     
    //     println!("conectado");
    //     let model = pago::ActiveModel {
    //         medio_pago: Set(self.medio_pago.clone()),
    //         monto: Set(self.monto),
    //             ..Default::default()
    //     };
    //     model.insert(&db).await?;
    //     Ok(())
    // }
}