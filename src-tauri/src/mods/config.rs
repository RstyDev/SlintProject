use serde::{Deserialize, Serialize};

use super::{Formato, Mayusculas};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    politica_redondeo: f64,
    formato_producto: Formato,
    modo_mayus: Mayusculas,
    cantidad_productos: usize,
    medios_pago: Vec<String>,
}

impl Config {
    pub fn get_cantidad_productos(&self) -> usize {
        self.cantidad_productos
    }
    pub fn get_medios_pago(&self) -> Vec<String> {
        self.medios_pago.clone()
    }
    pub fn get_politica(&self) -> f64 {
        self.politica_redondeo
    }
    pub fn get_formato(&self) -> Formato {
        self.formato_producto.clone()
    }
    pub fn get_modo_mayus(&self) -> Mayusculas {
        self.modo_mayus.clone()
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            politica_redondeo: 10.0,
            formato_producto: Formato::default(),
            modo_mayus: Mayusculas::default(),
            cantidad_productos: 10,
            medios_pago: vec![
                "Efectivo".to_owned(),
                "Crédito".to_owned(),
                "Débito".to_owned(),
            ],
        }
    }
}
