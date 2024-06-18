use super::Res;
use crate::db::map::{ConfigDB, MedioPagoDB};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::fmt::Display;
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
        let res: Option<ConfigDB> = sqlx::query_as!(ConfigDB, r#"select id, politica as "politica:_", formato, mayus, cantidad as "cantidad:_" from config limit 1"#)
            .fetch_optional(db)
            .await?;
        match res {
            Some(conf) => {
                let medios: sqlx::Result<Vec<MedioPagoDB>> =
                    sqlx::query_as!(MedioPagoDB, r#"select * from medios_pago "#)
                        .fetch_all(db)
                        .await;
                let medios = medios?
                    .iter()
                    .map(|med| Arc::from(med.medio.to_owned()))
                    .collect::<Vec<Arc<str>>>();
                Ok(Config::build(
                    conf.politica,
                    conf.formato.as_str(),
                    conf.mayus.as_str(),
                    conf.cantidad,
                    medios,
                ))
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
impl Display for Formato {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Formato::Tmv => String::from("Tmv"),
            Formato::Mtv => String::from("Mtv"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mayusculas {
    #[default]
    Upper,
    Lower,
    Camel,
}
impl Display for Mayusculas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Mayusculas::Upper => String::from("Upper"),
            Mayusculas::Lower => String::from("Lower"),
            Mayusculas::Camel => String::from("Camel"),
        };
        write!(f, "{}", str)
    }
}
