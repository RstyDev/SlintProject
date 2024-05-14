<<<<<<< HEAD
use serde::{Deserialize, Serialize};

=======
use entity::prelude::ProdProvDB;
use sea_orm::{ActiveModelTrait, Database, DbErr, Set};
use serde::{Deserialize, Serialize};

use super::lib::Save;

>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
#[derive(Clone, Serialize, Deserialize)]
pub struct RelacionProdProv {
    id_producto: i32,
    id_proveedor: i32,
    codigo_interno: Option<i64>,
}

impl RelacionProdProv {
    pub fn new(id_producto: i32, id_proveedor: i32, codigo_interno: Option<i64>) -> Self {
        RelacionProdProv {
            id_producto,
            id_proveedor,
            codigo_interno,
        }
    }
    pub fn id_producto(&self) -> &i32 {
        &self.id_producto
    }
    pub fn id_proveedor(&self) -> &i32 {
        &self.id_proveedor
    }
    pub fn codigo_interno(&self) -> Option<i64> {
        self.codigo_interno
    }
}
<<<<<<< HEAD
=======
impl Save for RelacionProdProv {
    async fn save(&self) -> Result<(), DbErr> {
        let model = ProdProvDB::ActiveModel {
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
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
