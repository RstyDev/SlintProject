use serde::{Serialize, Deserialize};

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
}
