use super::{AppError, Config, Pago, Res};
use crate::{
    db::{
        map::{BigIntDB, CajaDB},
        Mapper,
    },
    CajaFND, SharedString, VecModel,
};
use chrono::{NaiveDateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use slint::Model;
use sqlx::{query_as, Pool, Sqlite};
use std::{collections::HashMap, rc::Rc, sync::Arc};

#[derive(Clone, Serialize, Deserialize)]
pub struct Caja {
    id: i32,
    inicio: NaiveDateTime,
    cierre: Option<NaiveDateTime>,
    ventas_totales: f32,
    monto_inicio: f32,
    monto_cierre: Option<f32>,
    cajero: Option<Arc<str>>,
    totales: HashMap<Arc<str>, f32>,
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
        monto_de_inicio: Option<f32>,
        config: &Config,
    ) -> Result<Caja, AppError> {
        let caja;
        let mut totales = HashMap::new();
        for medio in config.medios_pago() {
            totales.insert(Arc::clone(medio), 0.0);
        }
        let caja_mod: sqlx::Result<Option<CajaDB>> =
            query_as!(CajaDB, r#"select id as "id: _", inicio, cierre, monto_inicio as "monto_inicio: _", monto_cierre as "monto_cierre: _", ventas_totales as "ventas_totales: _", cajero from cajas order by id desc"#)
                .fetch_optional(db)
                .await;
        caja = match caja_mod? {
            Some(caja) => match caja.cierre {
                Some(_) => match monto_de_inicio {
                    Some(monto) => {
                        sqlx::query(
                            "insert into cajas (inicio, ventas_totales, monto_inicio, cajero) values (?, ?, ?, ?)")
                            .bind(Utc::now().naive_local()).bind(caja.ventas_totales).bind(monto).bind(caja.cajero.clone()).execute(db).await?;
                        Ok(Caja::build(
                            caja.id + 1,
                            Utc::now().naive_local(),
                            None,
                            caja.ventas_totales,
                            monto,
                            None,
                            caja.cajero.map(|c| Arc::from(c.as_str())),
                            totales,
                        ))
                    }
                    None => Err(AppError::InicializationError(
                        "Se requiere monto de inicio".to_string(),
                    )),
                },
                None => Mapper::caja(db, caja).await,
            },
            None => {
                match monto_de_inicio {
                    Some(monto) => {
                        let inicio = Utc::now().naive_local();
                        sqlx::query("insert into cajas (inicio, ventas_totales, monto_inicio) values (?, ?, ?)")
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
                        ))
                    }
                    None => Err(AppError::InicializationError(
                        "Se requiere monto de inicio".to_string(),
                    )),
                }
            }
        };

        Ok(caja?)
    }
    pub fn build(
        id: i32,
        inicio: NaiveDateTime,
        cierre: Option<NaiveDateTime>,
        ventas_totales: f32,
        monto_inicio: f32,
        monto_cierre: Option<f32>,
        cajero: Option<Arc<str>>,
        totales: HashMap<Arc<str>, f32>,
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
    pub async fn hacer_movimiento(&self, mov: Movimiento, db: &Pool<Sqlite>) -> Res<()> {
        match mov {
            Movimiento::Ingreso { descripcion, monto } => {
                sqlx::query(
                    "insert into movimientos (caja, tipo, monto, descripcion, time) values (?, ?, ?, ?, ?))")
                    .bind(self.id)
                    .bind(true)
                    .bind(monto)
                    .bind(descripcion.map(|d|d.to_string()))
                    .bind(Utc::now().naive_local()).execute(db).await?;
            }
            Movimiento::Egreso { descripcion, monto } => {
                sqlx::query(
                    "insert into movimientos (caja, tipo, monto, descripcion, time) values (?, ?, ?, ?, ?))")
                    .bind(self.id)
                    .bind(false)
                    .bind(monto)
                    .bind(descripcion.map(|d|d.to_string()))
                    .bind(Utc::now().naive_local()).execute(db).await?;
            }
        }
        Ok(())
    }
    pub fn set_cajero(&mut self, cajero: Arc<str>) {
        self.cajero = Some(cajero);
    }
    pub async fn set_n_save(&mut self, db: &Pool<Sqlite>, monto: f32) -> Res<()> {
        self.monto_cierre = Some(monto);
        self.cierre = Some(Utc::now().naive_local());
        let res: sqlx::Result<Option<BigIntDB>> = sqlx::query_as!(
            BigIntDB,
            "select id as int from cajas where id = ? limit 1",
            self.id
        )
        .fetch_optional(db)
        .await;
        match res? {
            Some(_) => {
                sqlx::query("update cajas set cierre = ?, monto_cierre = ? where id = (?)")
                    .bind(self.cierre)
                    .bind(self.monto_cierre)
                    .bind(self.id)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Caja"),
                instancia: self.id.to_string(),
            }),
        }
    }

    pub async fn update_total(
        &mut self,
        db: &Pool<Sqlite>,
        monto: f32,
        pagos: &Vec<Pago>,
    ) -> Result<(), AppError> {
        for pago in pagos {
            let act = self.totales.remove(&pago.medio_pago().desc()).unwrap();
            self.totales
                .insert(pago.medio_pago().desc(), pago.monto() + act);
        }
        self.ventas_totales += monto;
        sqlx::query("update cajas set ventas_totales = ? where id = ?")
            .bind(self.id)
            .bind(self.ventas_totales)
            .execute(db)
            .await?;
        Ok(())
    }
    pub fn to_fnd(&self) -> CajaFND {
        let mut caja = CajaFND::default();
        caja.cajero = SharedString::from(match &self.cajero {
            None => SharedString::from(String::new()),
            Some(s) => SharedString::from(s.to_string()),
        });
        caja.id = self.id;
        caja.cierre = SharedString::from(match self.cierre {
            None => SharedString::from(String::new()),
            Some(s) => SharedString::from(s.to_string()),
        });
        caja.inicio = SharedString::from(self.inicio.to_string());
        //let mut medios=Vec::new();
        caja.monto_cierre = self.monto_cierre.unwrap_or(0.0);
        caja.monto_inicio = self.monto_inicio;
        let totales = self
            .totales
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect::<Vec<(String, f32)>>();
        caja.totales = Rc::new(VecModel::from(
            totales.iter().map(|(_, v)| *v).collect::<Vec<f32>>(),
        ))
        .into();
        caja.medios = Rc::new(VecModel::from(
            totales
                .iter()
                .map(|(k, _)| SharedString::from(k))
                .collect::<Vec<SharedString>>(),
        ))
        .into();
        caja.ventas_totales = self.ventas_totales;
        caja
    }
    pub fn from_fnd(caja: CajaFND) -> Self {
        let totales = caja
            .medios
            .iter()
            .zip(caja.totales.iter())
            .map(|(a, b)| (Arc::from(a.as_str()), b))
            .collect::<HashMap<Arc<str>, f32>>();
        Caja::build(
            caja.id,
            caja.inicio.parse().unwrap(),
            match caja.cierre.as_str() {
                "" => None,
                _ => Some(caja.cierre.parse().unwrap()),
            },
            caja.ventas_totales,
            caja.monto_inicio,
            match caja.monto_cierre {
                0.0 => None,
                _ => Some(caja.monto_cierre),
            },
            match caja.cajero.as_str() {
                "" => None,
                _ => Some(Arc::from(caja.cajero.as_str())),
            },
            totales,
        )
    }
}
