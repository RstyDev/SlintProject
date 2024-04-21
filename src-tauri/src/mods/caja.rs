use chrono::{NaiveDateTime, Utc};
use core::fmt;
use entity::prelude::{CajaDB, MovDB};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use super::{AppError, Config, Pago, Res};
#[derive(Clone, Serialize, Deserialize)]
pub struct Totales(HashMap<String, f64>);
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
        db: Arc<DatabaseConnection>,
        monto_inicio: Option<f32>,
        config: &Config,
    ) -> Result<Caja, AppError> {
        let caja;
        let mut totales = HashMap::new();
        for medio in config.medios_pago() {
            totales.insert(Arc::clone(medio), 0.0);
        }

        caja = match CajaDB::Entity::find()
            .order_by_desc(CajaDB::Column::Id)
            .one(db.as_ref())
            .await?
        {
            Some(res) => match res.cierre {
                Some(_) => match monto_inicio {
                    Some(monto_inicio) => Ok(Caja {
                        id: res.id + 1,
                        inicio: Utc::now().naive_local(),
                        cierre: None,
                        ventas_totales: 0.0,
                        monto_inicio,
                        monto_cierre: None,
                        cajero: None,
                        totales,
                    }),
                    None => Err(AppError::InicialationError(
                        "Nueva caja requiere un monto de inicio".to_string(),
                    )),
                },
                None => Ok(Caja {
                    id: res.id,
                    inicio: res.inicio,
                    cierre: None,
                    ventas_totales: res.ventas_totales,
                    monto_inicio: res.monto_inicio,
                    monto_cierre: None,
                    cajero: None,
                    totales,
                }),
            },
            None => match monto_inicio {
                Some(monto_inicio) => Ok(Caja {
                    id: 0,
                    inicio: Utc::now().naive_local(),
                    cierre: None,
                    ventas_totales: 0.0,
                    monto_inicio,
                    monto_cierre: None,
                    cajero: None,
                    totales,
                }),
                None => Err(AppError::InicialationError(
                    "Nueva caja requiere monto de inicio".to_string(),
                )),
            },
        };
        let aux = match caja {
            Ok(a) => a,
            Err(e) => return Err(e),
        };
        if CajaDB::Entity::find_by_id(aux.id)
            .one(db.as_ref())
            .await?
            .is_none()
        {
            let model = CajaDB::ActiveModel {
                id: Set(aux.id),
                inicio: Set(aux.inicio),
                cierre: Set(match aux.cierre {
                    Some(a) => Some(a),
                    None => None,
                }),
                monto_inicio: Set(aux.monto_inicio),
                monto_cierre: Set(aux.monto_cierre),
                ventas_totales: Set(aux.ventas_totales),
                cajero: match &aux.cajero {
                    Some(a) => Set(Some(a.to_string())),
                    None => NotSet,
                },
            };
            model.insert(db.as_ref()).await?;
        }
        Ok(aux)
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
        // let act=self.totales.remove(&medio).unwrap();
        // self.totales.insert(medio,act+monto);

        for pago in pagos {
            //     println!("{:#?}",pago.medio_pago());
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
