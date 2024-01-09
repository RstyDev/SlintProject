use std::error::Error;

use entity::relacion_prod_prov;
use sea_orm::{ActiveModelTrait, Database, Set, DbErr};
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
    pub fn get_id_producto(&self) -> i64 {
        self.id_producto
    }
    pub fn get_id_proveedor(&self) -> i64 {
        self.id_proveedor
    }
    pub fn get_codigo_interno(&self) -> Option<i64> {
        self.codigo_interno
    }
    pub async fn save(&self) -> Result<(),DbErr> {
        let model = relacion_prod_prov::ActiveModel {
            producto: Set(self.id_producto),
            proveedor: Set(self.id_proveedor),
            codigo: Set(self.codigo_interno),
            ..Default::default()
        };
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        println!("conectado");
        model.insert(&db).await?;
        Ok(())
    }
}
