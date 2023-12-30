use entity::proveedor;
use sea_orm::{Set, Database, ActiveModelTrait};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i64,
    nombre: String,
    contacto: Option<i64>,
}

impl Proveedor {
    pub fn new(id: i64, nombre: String, contacto: Option<i64>) -> Self {
        Proveedor {
            id,
            nombre,
            contacto,
        }
    }
    pub async fn save(&self) -> Result<(), String> {
        let model = proveedor::ActiveModel {
            id: Set(self.id),
            nombre: Set(self.nombre.clone()),
            contacto: Set(self.contacto),
        };
        match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
            Ok(db) => {
                println!("conectado");
                if let Err(e) = model.insert(&db).await {
                    Err(e.to_string())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
    pub fn get_nombre(&self)->String{
        self.nombre.clone()
    }
}
impl ToString for Proveedor {
    fn to_string(&self) -> String {
        let res;
        match self.contacto {
            Some(a) => res = format!("{} {}", self.nombre, a),
            None => res = format!("{}", self.nombre),
        }
        res
    }
}
