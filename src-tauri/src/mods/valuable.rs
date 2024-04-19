use super::{
    config::{Config, Formato},
    lib::{redondeo, Save},
    pesable::Pesable,
    producto::Producto,
    rubro::Rubro,
};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use Valuable as V;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Valuable {
    Prod((u8, Producto)),
    Pes((f32, Pesable)),
    Rub((u8, Rubro)),
}

impl Valuable {
    pub fn price(&self, politica: &f64) -> Option<f64> {
        match self {
            V::Pes(a) => Some(redondeo(politica, a.0 as f64 * a.1.precio_peso())),
            V::Prod(a) => Some(*a.1.redondear(politica).precio_de_venta()),
            V::Rub(a) => a.1.redondear(politica).monto().cloned(),
        }
    }
    // pub fn unifica_codes(&mut self) {
    //     match self {
    //         V::Prod(a) => a.1.unifica_codes(),
    //         _ => (),
    //     }
    // }
    pub fn descripcion(&self, conf: &Config) -> String {
        let res = match self {
            V::Pes(a) => a.1.descripcion().to_string(),
            V::Rub(a) => a.1.descripcion().to_string(),
            V::Prod(a) => match conf.formato() {
                Formato::Mtv => match a.1.presentacion() {
                    Presentacion::Gr(cant) => format!(
                        "{} {} {} {} Gr",
                        a.1.marca(),
                        a.1.tipo_producto(),
                        a.1.variedad(),
                        cant
                    ),
                    Presentacion::CC(cant) => format!(
                        "{} {} {} {} CC",
                        a.1.marca(),
                        a.1.tipo_producto(),
                        a.1.variedad(),
                        cant
                    ),
                    Presentacion::Kg(cant) => format!(
                        "{} {} {} {} Kg",
                        a.1.marca(),
                        a.1.tipo_producto(),
                        a.1.variedad(),
                        cant
                    ),
                    Presentacion::Lt(cant) => format!(
                        "{} {} {} {} Lt",
                        a.1.marca(),
                        a.1.tipo_producto(),
                        a.1.variedad(),
                        cant
                    ),
                    Presentacion::Ml(cant) => format!(
                        "{} {} {} {} Ml",
                        a.1.marca(),
                        a.1.tipo_producto(),
                        a.1.variedad(),
                        cant
                    ),
                    Presentacion::Un(cant) => format!(
                        "{} {} {} {} Un",
                        a.1.marca(),
                        a.1.tipo_producto(),
                        a.1.variedad(),
                        cant
                    ),
                },
                Formato::Tmv => {
                    format!("{} {} {}", a.1.tipo_producto(), a.1.marca(), a.1.variedad())
                }
            },
        };

        res
    }
}
impl Save for Valuable {
    async fn save(&self) -> Result<(), DbErr> {
        match self {
            V::Pes(a) => a.1.save().await,
            V::Prod(a) => a.1.save().await,
            V::Rub(a) => a.1.save().await,
        }
    }
}
// impl Default for Valuable {
//     fn default() -> Self {
//         V::Prod((1, Producto::default()))
//     }
// }
impl PartialEq for Valuable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (V::Pes(a), V::Pes(b)) => *a.1.id() == *b.1.id(),
            (V::Prod(a), V::Prod(b)) => a.1.id() == b.1.id(),
            (V::Rub(a), V::Rub(b)) => a.1.id() == b.1.id(),
            (_, _) => false,
        }
    }
}

pub trait ValuableTrait {
    fn redondear(&self, politica: &f64) -> Self;
}

impl ValuableTrait for Valuable {
    fn redondear(&self, politica: &f64) -> Valuable {
        match self {
            V::Pes(a) => V::Pes(a.clone()),
            V::Prod(a) => V::Prod((a.0, a.1.redondear(politica))),
            V::Rub(a) => V::Rub((a.0, a.1.redondear(politica))),
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

impl Presentacion {
    pub fn get_cantidad(&self) -> f32 {
        match self {
            Presentacion::Gr(c) => *c,
            Presentacion::Un(c) => *c as f32,
            Presentacion::Lt(c) => *c,
            Presentacion::Ml(c) => *c as f32,
            Presentacion::CC(c) => *c as f32,
            Presentacion::Kg(c) => *c,
        }
    }
    pub fn get_string(&self) -> String {
        match self {
            Presentacion::Gr(_) => String::from("Gr"),
            Presentacion::Un(_) => String::from("Un"),
            Presentacion::Lt(_) => String::from("Lt"),
            Presentacion::Ml(_) => String::from("Ml"),
            Presentacion::CC(_) => String::from("CC"),
            Presentacion::Kg(_) => String::from("Kg"),
        }
    }
}
