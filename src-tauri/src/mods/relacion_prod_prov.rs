use entity::relacion_prod_prov;
use sea_orm::{ActiveModelTrait, Database, DbErr, Set};
use serde::{Deserialize, Serialize};

use super::lib::Save;

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
    pub fn id_producto(&self) -> &i64 {
        &self.id_producto
    }
    pub fn id_proveedor(&self) -> &i64 {
        &self.id_proveedor
    }
    pub fn codigo_interno(&self) -> Option<i64> {
        self.codigo_interno
    }
}
impl Save for RelacionProdProv {
    async fn save(&self) -> Result<(), DbErr> {
        let model = relacion_prod_prov::ActiveModel {
            producto: Set(*self.id_producto()),
            proveedor: Set(*self.id_producto()),
            codigo: Set(self.codigo_interno),
            ..Default::default()
        };
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        println!("conectado");
        model.insert(&db).await?;
        Ok(())
    }
}
