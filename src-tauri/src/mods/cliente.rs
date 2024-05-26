use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

use crate::db::Model;

use super::{AppError, Res, User, Venta};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub enum Cliente {
    Final,
    Regular(Cli),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cli {
    id: i32,
    nombre: Arc<str>,
    dni: i32,
    activo: bool,
    created: NaiveDateTime,
    limite: Cuenta,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Cuenta {
    Auth(f32),
    Unauth,
}
impl Cli {
    pub async fn new_to_db(
        db: &Pool<Sqlite>,
        nombre: &str,
        dni: i32,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f32>,
    ) -> Res<Cli> {
        let model: Option<Model> =
            sqlx::query_as!(Model::Cliente, "select * from clientes where dni = ?", dni)
                .fetch_optional(db)
                .await?;
        match model {
            Some(model) => {
                return Err(AppError::ExistingError {
                    objeto: "Cliente".to_string(),
                    instancia: dni.to_string(),
                })
            }
            None => {
                let qres = sqlx::query("insert into clientes values (?, ?, ?, ?, ?)")
                    .bind(nombre)
                    .bind(dni)
                    .bind(limite)
                    .bind(activo)
                    .bind(created)
                    .execute(db)
                    .await?;
                Ok(Cli {
                    id: qres.last_insert_rowid() as i32,
                    nombre: Arc::from(nombre),
                    dni,
                    limite: match limite {
                        Some(limit) => Cuenta::Auth(limit),
                        None => Cuenta::Unauth,
                    },
                    activo,
                    created,
                })
            }
        }
    }
    pub fn new(
        id: i32,
        nombre: Arc<str>,
        dni: i32,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f32>,
    ) -> Cli {
        Cli {
            id,
            nombre,
            dni,
            limite: match limite {
                Some(limit) => Cuenta::Auth(limit),
                None => Cuenta::Unauth,
            },
            activo,
            created,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    #[cfg(test)]
    pub fn dni(&self) -> &i32 {
        &self.dni
    }
    pub fn limite(&self) -> &Cuenta {
        &self.limite
    }
    #[cfg(test)]
    pub fn nombre(&self) -> &str {
        self.nombre.as_ref()
    }
    pub async fn get_deuda(&self, db: &Pool<Sqlite>) -> Res<f32> {
        let model: sqlx::Result<Vec<Model>> = sqlx::query_as!(
            Model::Monto,
            "select monto from deudas where id = ?",
            self.id
        )
        .fetch_all(db)
        .await;
        Ok(model?
            .iter()
            .map(|e| match e {
                Model::Monto { monto } => monto,
                _ => panic!("Se esperaba monto"),
            })
            .sum::<f64>() as f32)
    }
    pub async fn get_deuda_detalle(
        &self,
        db: &Pool<Sqlite>,
        user: Option<Arc<User>>,
    ) -> Res<Vec<Venta>> {
        let mut ventas = Vec::new();

        let models = VentaDB::Entity::find()
            .filter(
                Condition::all()
                    .add(VentaDB::Column::Cliente.eq(self.id))
                    .add(VentaDB::Column::Paga.eq(false)),
            )
            .all(db)
            .await?;
        for model in models {
            ventas.push(Mapper::map_model_sale(&model, db, &user).await?);
        }
        Ok(ventas)
    }

    pub async fn pagar_deuda_especifica(
        id: i32,
        db: &DatabaseConnection,
        venta: Venta,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let model = match VentaDB::Entity::find_by_id(*venta.id()).one(db).await? {
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
    pub async fn pagar_deuda_general(id: i32, db: &DatabaseConnection, mut monto: f32) -> Res<f32> {
        let models = VentaDB::Entity::find()
            .filter(
                Condition::all()
                    .add(VentaDB::Column::Cliente.eq(id))
                    .add(VentaDB::Column::Paga.eq(false)),
            )
            .order_by_asc(VentaDB::Column::Time)
            .all(db)
            .await?;
        println!("{:#?} encontrados {}", models, models.len());
        let resto = monto
            - models
                .iter()
                .map(|model| model.monto_total - model.monto_pagado)
                .sum::<f32>();
        for model in models {
            if monto <= 0.0 {
                break;
            }
            let mut model = model.into_active_model();
            let mut pagos = PagoDB::Entity::find()
                .filter(
                    Condition::all()
                        .add(PagoDB::Column::Venta.eq(model.id.clone().unwrap()))
                        .add(PagoDB::Column::MedioPago.eq(0)),
                )
                .all(db)
                .await?
                .iter()
                .cloned()
                .map(|pago| pago.into_active_model())
                .collect::<Vec<PagoDB::ActiveModel>>();
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
