pub mod config;
pub mod lib;
pub mod pago;
pub mod pesable;
pub mod producto;
pub mod proveedor;
pub mod relacion_prod_prov;
pub mod rubro;
pub mod sistema;
pub mod valuable;
pub mod venta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Formato {
    #[default]
    Tmv,
    Mtv,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mayusculas {
    #[default]
    Upper,
    Lower,
    Camel,
}

//-----------------------------------Implementations---------------------------------
