use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use super::error::AppError;
type Res<T> = std::result::Result<T, AppError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    politica_redondeo: f64,
    formato_producto: Formato,
    modo_mayus: Mayusculas,
    cantidad_productos: u8,
    medios_pago: Vec<String>,
}

impl Config {
    pub async fn get_or_def(db: &DatabaseConnection) -> Res<Config> {
        match entity::config::Entity::find().one(db).await? {
            Some(a) => {
                let medios = entity::medio_pago::Entity::find()
                    .all(db)
                    .await?
                    .iter()
                    .map(|x| x.medio.clone())
                    .collect::<Vec<String>>();
                Ok(Config {
                    politica_redondeo: a.politica_redondeo,
                    formato_producto: match a.formato_producto.as_str() {
                        "Mtv" => Formato::Mtv,
                        "Tmv" => Formato::Tmv,
                        _ => panic!("no existe mas que mtv y tmv"),
                    },
                    modo_mayus: match a.modo_mayus.as_str() {
                        "Upper" => Mayusculas::Upper,
                        "Lower" => Mayusculas::Lower,
                        "Camel" => Mayusculas::Camel,
                        _ => panic!("no existe mas que lower, camel y upper"),
                    },
                    cantidad_productos: a.cantidad_productos,
                    medios_pago: medios,
                })
            }
            None => {
                let conf = Config::default();
                let model = entity::config::ActiveModel {
                    cantidad_productos: Set(conf.cantidad_productos),
                    formato_producto: Set(conf.formato_producto.to_string()),
                    id: Set(0),
                    modo_mayus: Set(conf.modo_mayus.to_string()),
                    politica_redondeo: Set(conf.politica_redondeo),
                };
                entity::config::Entity::insert(model).exec(db).await?;
                Ok(conf)
            }
        }
    }
    pub fn cantidad_productos(&self) -> &u8 {
        &self.cantidad_productos
    }
    pub fn medios_pago(&self) -> &Vec<String> {
        &self.medios_pago
    }
    pub fn politica(&self) -> f64 {
        self.politica_redondeo
    }
    pub fn formato(&self) -> &Formato {
        &self.formato_producto
    }
    pub fn modo_mayus(&self) -> Mayusculas {
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Formato {
    #[default]
    Tmv,
    Mtv,
}
impl ToString for Formato {
    fn to_string(&self) -> String {
        match self {
            Formato::Tmv => String::from("Tmv"),
            Formato::Mtv => String::from("Mtv"),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mayusculas {
    #[default]
    Upper,
    Lower,
    Camel,
}
impl ToString for Mayusculas {
    fn to_string(&self) -> String {
        match self {
            Mayusculas::Upper => String::from("Upper"),
            Mayusculas::Lower => String::from("Lower"),
            Mayusculas::Camel => String::from("Camel"),
        }
    }
}
