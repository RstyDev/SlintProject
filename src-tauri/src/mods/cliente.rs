use chrono::NaiveDateTime;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::App;

use super::{error::AppError, lib::Mapper, user::User, venta::Venta};
type Res<T> = std::result::Result<T, AppError>;
#[derive(Serialize, Clone, Debug)]
pub enum Cliente {
    Final,
    Regular(Cli),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cli {
    id: i64,
    nombre: Arc<str>,
    dni: i64,
    credito: bool,
    activo: bool,
    created: NaiveDateTime,
    limite: Cuenta,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Cuenta {
    Auth(Option<f64>),
    Unauth,
}
impl Cli {
    pub async fn new_to_db(
        db: &DatabaseConnection,
        nombre: &str,
        dni: i64,
        credito: bool,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f64>,
    ) -> Res<Cli> {
        match entity::cliente::Entity::find()
            .filter(entity::cliente::Column::Dni.eq(dni))
            .one(db)
            .await?
        {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Cliente".to_string(),
                    instancia: format!("{}", dni),
                })
            }
            None => {
                let model = entity::cliente::ActiveModel {
                    nombre: Set(nombre.to_string()),
                    dni: Set(dni),
                    credito: Set(credito),
                    activo: Set(activo),
                    created: Set(created),
                    limite: Set(limite),
                    ..Default::default()
                };
                let res = entity::cliente::Entity::insert(model).exec(db).await?;
                Ok(Cli {
                    id: res.last_insert_id,
                    nombre: Arc::from(nombre),
                    dni,
                    activo,
                    credito,
                    created,
                    limite: match credito {
                        true => Cuenta::Auth(limite),
                        false => Cuenta::Unauth,
                    },
                })
            }
        }
    }
    pub fn new(
        id: i64,
        nombre: Arc<str>,
        dni: i64,
        credito: bool,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f64>,
    ) -> Cli {
        Cli {
            id,
            nombre,
            dni,
            credito,
            limite: match credito {
                true => Cuenta::Auth(limite),
                false => Cuenta::Unauth,
            },
            activo,
            created,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn credito(&self) -> &bool {
        &self.credito
    }
    pub async fn get_deuda(&self, db: &DatabaseConnection) -> Res<f64> {
        Ok(entity::deuda::Entity::find()
            .select_only()
            .column(entity::deuda::Column::Monto)
            .filter(Condition::all().add(entity::deuda::Column::Cliente.eq(self.id)))
            .all(db)
            .await?
            .iter()
            .map(|m| m.monto)
            .sum::<f64>())
    }
    pub async fn get_deuda_detalle(
        &self,
        db: &DatabaseConnection,
        user: Option<Arc<User>>,
    ) -> Res<Vec<Venta>> {
        let mut ventas = Vec::new();
        let models = entity::venta::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::venta::Column::Cliente.eq(self.id))
                    .add(entity::venta::Column::Paga.eq(false)),
            )
            .all(db)
            .await?;
        for model in models {
            ventas.push(Mapper::map_model_sale(&model, db, &user).await?);
        }
        Ok(ventas)
    }

    pub async fn pagar_deuda_especifica(
        id: i64,
        db: &DatabaseConnection,
        venta: Venta,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let model = match entity::venta::Entity::find_by_id(*venta.id())
            .one(db)
            .await?
        {
            Some(model) => model,
            None => return Err(AppError::IncorrectError(String::from("Id inexistente"))),
        };
        match model.cliente {
            Some(cli) => {
                if cli == id {
                    let mut model = model.clone().into_active_model();
                    model.paga = Set(true);
                    model.update(db).await?;
                } else {
                    return Err(AppError::IncorrectError("Cliente Incorrecto".to_string()));
                }
            }
            None => return Err(AppError::IncorrectError(String::from("Cliente Incorrecto"))),
        }
        let venta = Mapper::map_model_sale(&model, db, &user).await?;
        Ok(venta)
    }
    pub async fn pagar_deuda_general(id: i64, db: &DatabaseConnection, mut monto: f64) -> Res<f64> {
        let models = entity::venta::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::venta::Column::Cliente.eq(id))
                    .add(entity::venta::Column::Paga.eq(false)),
            )
            .order_by_asc(entity::venta::Column::Time)
            .all(db)
            .await?;
        println!("{:#?} encontrados {}", models, models.len());
        let resto = monto
            - models
                .iter()
                .map(|model| model.monto_total - model.monto_pagado)
                .sum::<f64>();
        for model in models {
            if monto <= 0.0 {
                break;
            }
            let mut model = model.into_active_model();
            let mut pagos = entity::pago::Entity::find()
                .filter(
                    Condition::all()
                        .add(entity::pago::Column::Venta.eq(model.id.clone().unwrap()))
                        .add(entity::pago::Column::MedioPago.eq(0)),
                )
                .all(db)
                .await?
                .iter()
                .cloned()
                .map(|pago| pago.into_active_model())
                .collect::<Vec<entity::pago::ActiveModel>>();
            let mut completados: u8 = 0;
            for i in 0..pagos.len() {
                if monto <= 0.0 {
                    break;
                }
                if pagos[i].pagado.as_ref() < pagos[i].monto.as_ref() {
                    if monto >= pagos[i].monto.as_ref() - pagos[i].pagado.as_ref() {
                        monto -= pagos[i].monto.as_ref() - pagos[i].pagado.as_ref();
                        pagos[i].pagado = Set(*pagos[i].monto.as_ref());
                        completados += 1;
                        pagos[i].clone().update(db).await?;
                    } else {
                        pagos[i].pagado = Set(pagos[i].pagado.as_ref() + monto);
                        monto = 0.0;
                        pagos[i].clone().update(db).await?;
                    }
                }
            }
            if completados == pagos.len() as u8 {
                model.paga = Set(true);
                model.update(db).await?;
            }
        }
        Ok(resto)
    }
}

impl<'a> Cliente {
    pub fn new(cli: Option<Cli>) -> Cliente {
        match cli {
            Some(a) => Cliente::Regular(a),
            None => Cliente::Final,
        }
    }
}
impl Default for Cliente {
    fn default() -> Self {
        Cliente::Final
    }
}
