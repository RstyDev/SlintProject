use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder, Set};
use std::str::FromStr;
use std::sync::Arc;

use super::error::AppError;
#[derive(Clone)]
pub struct Caja {
    id: i64,
    inicio: NaiveDateTime,
    cierre: Option<NaiveDateTime>,
    ventas_totales: f64,
    monto_inicio: f64,
    monto_cierre: Option<f64>,
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
            };
            model.insert(db.as_ref()).await?;
        }
        Ok(aux)
    }
    pub async fn update_total(&mut self,db:&DatabaseConnection, monto:f64)->Result<(),AppError>{
        self.ventas_totales+=monto;
        let model=entity::caja::Entity::find_by_id(self.id).one(db).await?.unwrap();
        let mut model=model.into_active_model();
        model.ventas_totales=Set(self.ventas_totales);
        model.update(db).await?;
        Ok(())
    }
}
