use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use serde::de::IntoDeserializer;

use super::{error::AppError, venta::Venta};

pub struct Caja {
    id: i64,
    inicio: NaiveDateTime,
    cierre: Option<NaiveDateTime>,
    ventas_totales: f64,
    monto_inicio: f64,
    monto_cierre: Option<f64>,
}

impl Caja{
    pub async fn new(db:&DatabaseConnection, monto_inicio:f64)->Result<Caja,AppError>{
        if let Some(res)=entity::caja::Entity::find().order_by_desc(entity::caja::Column::Id).one(db).await?{
            if let Some(_)=res.cierre{
                Ok(Caja{
                    id: res.id+1,
                    inicio: Utc::now().naive_local(),
                    cierre: None,
                    ventas_totales: 0.0,
                    monto_inicio,
                    monto_cierre: None,
                })
            }else{
                Ok(Caja{
                    id: res.id,
                    inicio: NaiveDateTime::from_str(res.inicio.as_str())?,
                    cierre: None,
                    ventas_totales: res.ventas_totales,
                    monto_inicio: res.monto_inicio,
                    monto_cierre: res.monto_cierre,
                })
            }
        }else{
            Ok(Caja{
                id: todo!(),
                inicio: todo!(),
                cierre: todo!(),
                ventas_totales: todo!(),
                monto_inicio: todo!(),
                monto_cierre: todo!(),
            })
        }

    }
}