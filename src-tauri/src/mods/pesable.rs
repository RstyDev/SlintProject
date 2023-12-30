// use entity::pesable;
use sea_orm::{Database, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pesable {
    pub id: i64,
    pub codigo: i64,
    pub precio_peso: f64,
    pub porcentaje: f64,
    pub costo_kilo: f64,
    pub descripcion: String,
}
impl Pesable {
    pub fn new(
        id: i64,
        codigo: i64,
        precio_peso: f64,
        porcentaje: f64,
        costo_kilo: f64,
        descripcion: String,
    ) -> Pesable {
        Pesable {
            id,
            codigo,
            precio_peso,
            porcentaje,
            costo_kilo,
            descripcion,
        }
    }
    // pub async fn save(&self) -> Result<(), String> {
    //     match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
    //         Ok(db) => {
    //             println!("conectado");
    //             let model = pesable::ActiveModel {
    //                 id: Set(self.id),
    //                 codigo: Set(self.codigo),
    //                 precio_peso: Set(self.precio_peso),
    //                 porcentaje: Set(self.porcentaje),
    //                 costo_kilo: Set(self.costo_kilo),
    //                 descripcion: Set(self.descripcion.clone()),
    //             };
    //             if let Err(e) = model.insert(&db).await {
    //                 return Err(e.to_string());
    //             }
    //         }
    //         Err(e) => return Err(e.to_string()),
    //     }

    //     Ok(())
    // }
}
