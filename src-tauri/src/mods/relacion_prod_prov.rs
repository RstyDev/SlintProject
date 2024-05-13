use serde::{Deserialize, Serialize};

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