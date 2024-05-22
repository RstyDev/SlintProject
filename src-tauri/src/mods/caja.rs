use chrono::{NaiveDateTime, Utc};
use core::fmt;
use sqlx::{
    query, query_as, sqlite::SqliteConnectOptions, Connection, Pool, Sqlite, SqliteConnection,
};
use tauri::async_runtime::block_on;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::db::{Mapper, Model};

use super::{AppError, Config, Pago, Res};

#[derive(Clone, Serialize, Deserialize)]
pub struct Totales(HashMap<String, f64>);
#[derive(Clone, Serialize, Deserialize)]
pub struct Caja {
    id: i64,
    inicio: NaiveDateTime,
    cierre: Option<NaiveDateTime>,
    ventas_totales: f64,
    monto_inicio: f64,
    monto_cierre: Option<f64>,
    cajero: Option<Arc<str>>,
    totales: HashMap<Arc<str>, f64>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Movimiento {
    Ingreso {
        descripcion: Option<Arc<str>>,
        monto: f32,
    },
    Egreso {
        descripcion: Option<Arc<str>>,
        monto: f32,
    },
}
impl fmt::Debug for Caja {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Caja")
            .field("id", &self.id)
            .field("inicio", &self.inicio)
            .field("cierre", &self.cierre)
            .field("ventas_totales", &self.ventas_totales)
            .field("monto_inicio", &self.monto_inicio)
            .field("monto_cierre", &self.monto_cierre)
            .field("cajero", &self.cajero)
            .finish()
    }
}

impl Caja {
    pub async fn new(
        db: &Pool<Sqlite>,
        monto_de_inicio: Option<f64>,
        config: &Config,
    ) -> Result<Caja, AppError> {
        let options = SqliteConnectOptions::new();
        let connection = block_on(SqliteConnection::connect("url")).unwrap();

        let caja;
        let mut totales = HashMap::new();
        for medio in config.medios_pago() {
            totales.insert(Arc::clone(medio), 0.0);
        }
        let caja_mod: sqlx::Result<Option<Model>> =
            query_as!(Model::Caja, "select * from cajas order by id desc")
                .fetch_optional(db)
                .await;
        caja = match caja_mod? {
            Some(model_caja) => match &model_caja {
                Model::Caja {
                    id,
                    inicio,
                    cierre,
                    monto_inicio,
                    monto_cierre,
                    ventas_totales,
                    cajero,
                } => match cierre {
                    Some(_) => match monto_de_inicio {
                        Some(monto) => {
                           sqlx::query(
                                    "insert into cajas (inicio, ventas_totales, monto_inicio, cajero) values (?, ?, ?, ?, ?, ?, ?)")
                                    .bind(Utc::now().naive_local()).bind(ventas_totales).bind(monto).bind(cajero.clone()).execute(db).await?;
                            Ok(Caja::build(
                                id+1,
                                Utc::now().naive_local(),
                                None,
                                *ventas_totales,
                                monto,
                                None,
                                cajero.as_ref().map(|c| Arc::from(c.as_str())),
                                totales,
                            ))
                        }
                        None => Err(AppError::InicializationError(
                            "Se requiere monto de inicio".to_string(),
                        )),
                    },
                    None => Mapper::caja(db, model_caja).await,
                },
                _ => Err(AppError::IncorrectError("No posible".to_string())),
            },
            None => match monto_de_inicio {
                Some(monto) => {
                    let inicio=Utc::now().naive_local();
                    sqlx::query("insert into cajas (inicio, ventas_totales, monto_inicio) values (?, ?, ?, ?)")
                    .bind(inicio).bind(0.0).bind(monto).execute(db).await?;
                    Ok(Caja::build(
                    0,
                    Utc::now().naive_local(),
                    None,
                    0.0,
                    monto,
                    None,
                    None,
                    HashMap::new(),
                ))},
                None => Err(AppError::InicializationError(
                    "Se requiere monto de inicio".to_string(),
                )),
            },
        };

        Ok(caja?)
    }
    pub fn build(
        id: i64,
        inicio: NaiveDateTime,
        cierre: Option<NaiveDateTime>,
        ventas_totales: f64,
        monto_inicio: f64,
        monto_cierre: Option<f64>,
        cajero: Option<Arc<str>>,
        totales: HashMap<Arc<str>, f64>,
    ) -> Caja {
        Caja {
            id,
            inicio,
            cierre,
            ventas_totales,
            monto_inicio,
            monto_cierre,
            cajero,
            totales,
        }
    }
    pub async fn hacer_movimiento(&self, mov: Movimiento, db: &DatabaseConnection) -> Res<()> {
        let monto_model;
        let tipo;
        let desc;
        match mov {
            Movimiento::Ingreso { descripcion, monto } => {
                monto_model = Set(monto);
                tipo = Set(true);
                desc = match descripcion {
                    Some(d) => Set(Some(d.to_string())),
                    None => Set(None),
                }
            }
            Movimiento::Egreso { descripcion, monto } => {
                monto_model = Set(monto);
                tipo = Set(false);
                desc = match descripcion {
                    Some(d) => Set(Some(d.to_string())),
                    None => Set(None),
                }
            }
        }
        MovDB::ActiveModel {
            caja: Set(self.id),
            tipo,
            monto: monto_model,
            time: Set(Utc::now().naive_local()),
            descripcion: desc,
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }
    pub fn set_cajero(&mut self, cajero: Arc<str>) {
        self.cajero = Some(cajero);
    }
    pub async fn set_n_save(&mut self, db: &DatabaseConnection, monto: f32) -> Res<()> {
        self.monto_cierre = Some(monto);
        self.cierre = Some(Utc::now().naive_local());
        match CajaDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => {
                let mut model = model.into_active_model();
                model.cierre = Set(self.cierre);
                model.monto_cierre = Set(Some(monto));
                model.update(db).await?;
                Ok(())
            }
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Caja"),
                    instancia: self.id.to_string(),
                })
            }
        }
    }

    pub async fn update_total(
        &mut self,
        db: &DatabaseConnection,
        monto: f32,
        pagos: &Vec<Pago>,
    ) -> Result<(), AppError> {
        for pago in pagos {
            let act = self.totales.remove(&pago.medio_pago().desc()).unwrap();
            self.totales
                .insert(pago.medio_pago().desc(), pago.monto() + act);
        }
        self.ventas_totales += monto;
        let model = CajaDB::Entity::find_by_id(self.id).one(db).await?.unwrap();
        let mut model = model.into_active_model();
        model.ventas_totales = Set(self.ventas_totales);
        model.update(db).await?;
        Ok(())
    }
}
