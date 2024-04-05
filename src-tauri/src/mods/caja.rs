use chrono::{NaiveDateTime, Utc};
use core::fmt;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
type Res<T> = std::result::Result<T, AppError>;
use super::error::AppError;
#[derive(Clone, Serialize, Deserialize)]
pub struct Caja {
    id: i64,
    inicio: NaiveDateTime,
    cierre: Option<NaiveDateTime>,
    ventas_totales: f64,
    monto_inicio: f64,
    monto_cierre: Option<f64>,
    cajero: Option<Arc<str>>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Movimiento {
    id: i64,
    caja: i64,
    tipo: TipoMovimiento,
}
#[derive(Debug, Clone, Serialize)]
pub enum TipoMovimiento {
    Ingreso(f64),
    Egreso(f64),
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
        monto_inicio: Option<f64>,
    ) -> Result<Caja, AppError> {
        let caja;
        caja = match entity::caja::Entity::find()
            .order_by_desc(entity::caja::Column::Id)
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
        if entity::caja::Entity::find_by_id(aux.id)
            .one(db.as_ref())
            .await?
            .is_none()
        {
            let model = entity::caja::ActiveModel {
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

    pub fn set_cajero(&mut self, cajero: Arc<str>) {
        self.cajero = Some(cajero);
    }
    pub async fn set_n_save(&mut self, db: &DatabaseConnection, monto: f64) -> Res<()> {
        self.monto_cierre = Some(monto);
        self.cierre = Some(Utc::now().naive_local());
        match entity::caja::Entity::find_by_id(self.id).one(db).await? {
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
        monto: f64,
    ) -> Result<(), AppError> {
        self.ventas_totales += monto;
        let model = entity::caja::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .unwrap();
        let mut model = model.into_active_model();
        model.ventas_totales = Set(self.ventas_totales);
        model.update(db).await?;
        Ok(())
    }
}
