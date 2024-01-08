type Result<T> = std::result::Result<T, Box<dyn Error>>;
use super::{
    config::{Config, Formato, Mayusculas},
    lib::camalize,
    pesable::Pesable,
    producto::Producto,
    rubro::Rubro,
};
use serde::{Deserialize, Serialize};
use std::{fmt::{self, Display}, error::Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Valuable {
    Prod((u16, Producto)),
    Pes((f32, Pesable)),
    Rub((u16, Rubro)),
}

impl Valuable {
    // pub fn get_price(&self, politica: f64) -> f64 {
    //     match self {
    //         Valuable::Pes(a) => redondeo(politica, a.0 as f64 * a.1.precio_peso),
    //         Valuable::Prod(a) => a.1.redondear(politica).precio_de_venta,
    //         Valuable::Rub(a) => a.1.redondear(politica).monto,
    //     }
    // }
    pub fn unifica_codes(&mut self) {
        match self {
            Valuable::Prod(a) => a.1.unifica_codes(),
            _ => (),
        }
    }
    pub fn get_descripcion(&self, conf: Config) -> String {
        let mut res = match self {
            Valuable::Pes(a) => a.1.descripcion.clone(),
            Valuable::Rub(a) => a.1.descripcion.clone(),
            Valuable::Prod(a) => match conf.get_formato() {
                Formato::Mtv => match a.1.presentacion {
                    Presentacion::Gr(cant) => format!(
                        "{} {} {} {} Gr",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::CC(cant) => format!(
                        "{} {} {} {} CC",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Kg(cant) => format!(
                        "{} {} {} {} Kg",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Lt(cant) => format!(
                        "{} {} {} {} Lt",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Ml(cant) => format!(
                        "{} {} {} {} Ml",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Un(cant) => format!(
                        "{} {} {} {} Un",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                },
                Formato::Tmv => format!("{} {} {}", a.1.tipo_producto, a.1.marca, a.1.variedad),
            },
        };
        match conf.get_modo_mayus() {
            Mayusculas::Lower => res = res.to_lowercase(),
            Mayusculas::Upper => res = res.to_uppercase(),
            Mayusculas::Camel => res= camalize(res.as_str()).to_string(),
        }
        res
    }
    pub async fn save(&self) -> Result<()> {
        match self {
            Valuable::Pes(a) => a.1.save().await,
            Valuable::Prod(a) => a.1.save().await,
            Valuable::Rub(a) => a.1.save().await,
        }
    }
}
impl Default for Valuable {
    fn default() -> Self {
        Valuable::Prod((1, Producto::default()))
    }
}
impl PartialEq for Valuable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Valuable::Pes(a), Valuable::Pes(b)) => a.1.id == b.1.id,
            (Valuable::Prod(a), Valuable::Prod(b)) => a.1.get_id() == b.1.get_id(),
            (Valuable::Rub(a), Valuable::Rub(b)) => a.1.id == b.1.id,
            (_, _) => false,
        }
    }
}

pub trait ValuableTrait {
    fn redondear(&self, politica: f64) -> Self;
}

impl ValuableTrait for Valuable {
    fn redondear(&self, politica: f64) -> Valuable {
        match self {
            Valuable::Pes(a) => Valuable::Pes(a.clone()),
            Valuable::Prod(a) => Valuable::Prod((a.0, a.1.redondear(politica))),
            Valuable::Rub(a) => Valuable::Rub((a.0, a.1.redondear(politica))),
        }
    }
}

impl Display for Presentacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gr(a) => write!(f, "{} Gr", a),
            Self::Lt(a) => write!(f, "{} Lt", a),
            Self::Un(a) => write!(f, "{} Un", a),
            Self::Ml(a) => write!(f, "{} Ml", a),
            Self::CC(a) => write!(f, "{} CC", a),
            Self::Kg(a) => write!(f, "{} Kg", a),
        }
    }
}
impl Default for Presentacion {
    fn default() -> Self {
        Presentacion::Un(i16::default())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Gr(f32),
    Un(i16),
    Lt(f32),
    Ml(i16),
    CC(i16),
    Kg(f32),
}
