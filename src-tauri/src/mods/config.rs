use super::{AppError, Res};
use crate::db::Model;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    politica_redondeo: f32,
    formato_producto: Formato,
    modo_mayus: Mayusculas,
    cantidad_productos: u8,
    medios_pago: Vec<Arc<str>>,
}

impl Config {
    pub async fn get_or_def(db: &Pool<Sqlite>) -> Res<Config> {
        let res: sqlx::Result<Option<Model>> =
            sqlx::query_as!(Model::Config, "select * from config")
                .fetch_optional(db)
                .await;
        match res? {
            Some(conf) => {
                let medios: sqlx::Result<Vec<Model>> =
                    sqlx::query_as!(Model::MedioPago, "select * from medios_pago")
                        .fetch_all(db)
                        .await;
                let medios = medios?
                    .iter()
                    .map(|med| match med {
                        Model::MedioPago { id: _, medio } => Arc::from(medio.as_str()),
                        _ => panic!("Imposible, se esperaba medio pago"),
                    })
                    .collect::<Vec<Arc<str>>>();
                match conf {
                    Model::Config {
                        id: _,
                        politica,
                        formato,
                        mayus,
                        cantidad,
                    } => Ok(Config::build(
                        politica,
                        formato.as_str(),
                        mayus.as_str(),
                        cantidad as u8,
                        medios,
                    )),
                    _ => Err(AppError::IncorrectError(String::from("Se esperaba Config"))),
                }
            }
            None => {
                let conf = Config::default();
                sqlx::query("insert into config values (?, ?, ?, ?)")
                    .bind(conf.politica())
                    .bind(conf.formato().to_string())
                    .bind(conf.modo_mayus().to_string())
                    .bind(conf.cantidad_productos())
                    .execute(db)
                    .await?;
                Ok(conf)
            }
        }
    }
    pub fn build(
        politica_redondeo: f32,
        formato_producto: &str,
        modo_mayus: &str,
        cantidad_productos: u8,
        medios_pago: Vec<Arc<str>>,
    ) -> Config {
        let formato_producto = match formato_producto {
            "Tmv" => Formato::Tmv,
            "Mtv" => Formato::Mtv,
            _ => panic!("solo hay Tmv y Mtv"),
        };
        let modo_mayus = match modo_mayus {
            "Upper" => Mayusculas::Upper,
            "Lower" => Mayusculas::Lower,
            "Camel" => Mayusculas::Camel,
            _ => panic!("solo hay Upper, Lower y Camel"),
        };
        Config {
            politica_redondeo,
            formato_producto,
            modo_mayus,
            cantidad_productos,
            medios_pago,
        }
    }
    pub fn cantidad_productos(&self) -> &u8 {
        &self.cantidad_productos
    }
    pub fn medios_pago(&self) -> &Vec<Arc<str>> {
        &self.medios_pago
    }
    pub fn politica(&self) -> f32 {
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
            cantidad_productos: 20,
            medios_pago: vec![
                Arc::from("Efectivo"),
                Arc::from("Crédito"),
                Arc::from("Débito"),
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
