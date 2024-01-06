use entity::relacion_prod_prov;
use sea_orm::{Set, Database, ActiveModelTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RelacionProdProv {
    id_producto: i64,
    id_proveedor: i64,
    codigo_interno: Option<i64>,
}

impl RelacionProdProv {
    pub fn new(id_producto: i64, id_proveedor: i64, codigo_interno: Option<i64>) -> Self {
        RelacionProdProv {
            id_producto,
            id_proveedor,
            codigo_interno,
        }
    }
    pub fn get_id_producto(&self)->i64{
        self.id_producto
    }
    pub fn get_id_proveedor(&self)->i64{
        self.id_proveedor
    }
    pub fn get_codigo_interno(&self)->Option<i64>{
        self.codigo_interno
    }
    pub async fn save(&self) -> Result<(), String> {
        let model = relacion_prod_prov::ActiveModel {
            producto: Set(self.id_producto),
            proveedor: Set(self.id_proveedor),
            codigo: Set(self.codigo_interno),
            ..Default::default()
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
}
