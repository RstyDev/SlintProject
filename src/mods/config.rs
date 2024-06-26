use super::Res;
use crate::{
    db::map::{ConfigDB, MedioPagoDB},
    ConfigFND, SharedString,
};
use serde::{Deserialize, Serialize};
use slint::Model;
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
        let res: Option<ConfigDB> = sqlx::query_as!(ConfigDB, r#"select id as "id:_", politica as "politica:_", formato, mayus, cantidad as "cantidad:_" from config limit 1"#)
            .fetch_optional(db)
            .await?;
        match res {
            Some(conf) => {
                let medios: sqlx::Result<Vec<MedioPagoDB>> = sqlx::query_as!(
                    MedioPagoDB,
                    r#"select id as "id:_", medio from medios_pago "#
                )
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
                sqlx::query("insert into config values (?, ?, ?, ?, ?)")
                    .bind(1)
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
    pub fn to_fnd(&self) -> ConfigFND {
        let mut conf = ConfigFND::default();
        conf.cantidad_productos = self.cantidad_productos as i32;
        conf.formato_producto = SharedString::from(self.formato_producto.to_string());
        conf.modo_mayus = SharedString::from(self.modo_mayus.to_string());
        conf.politica_redondeo = self.politica_redondeo;
        conf
    }
    pub fn from_fnd(conf: ConfigFND) -> Self {
        Config::build(
            conf.politica_redondeo,
            conf.formato_producto.as_str(),
            conf.modo_mayus.as_str(),
            conf.cantidad_productos as u8,
            conf.medios
                .iter()
                .map(|med| Arc::from(med.as_str()))
                .collect::<Vec<Arc<str>>>(),
        )
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
