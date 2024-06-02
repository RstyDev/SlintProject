use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

use crate::db::{Mapper, Model};

use super::{AppError, Res, User, Venta};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub enum Cliente {
    Final,
    Regular(Cli),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cli {
    id: i64,
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
        dni: i64,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f32>,
    ) -> Res<Cli> {
        let model: Option<Model> =
            sqlx::query_as!(Model::Cliente, "select * from clientes where dni = ?", dni)
                .fetch_optional(db)
                .await?;
        match model {
            Some(_) => {
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
                    id: qres.last_insert_rowid(),
                    nombre: Arc::from(nombre),
                    dni: dni as i32,
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
    pub fn build(
        id: i64,
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
    pub fn id(&self) -> &i64 {
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
        let qres: Vec<Model> = sqlx::query_as!(
            Model::Venta,
            "select * from ventas where cliente = ? and paga = ?",
            self.id,
            false
        )
        .fetch_all(db)
        .await?;
        for model in qres {
            match model {
                Model::Venta {
                    id: _,
                    time: _,
                    monto_total: _,
                    monto_pagado: _,
                    cliente: _,
                    cerrada: _,
                    paga: _,
                    pos: _,
                } => ventas.push(Mapper::venta(db, model, &user).await?),
                _ => return Err(AppError::IncorrectError(String::from("se esperaba venta"))),
            }
        }
        Ok(ventas)
    }

    pub async fn pagar_deuda_especifica(
        id_cliente: i64,
        db: &Pool<Sqlite>,
        venta: Venta,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let qres: Option<Model> = sqlx::query_as!(
            Model::Venta,
            "select * from ventas where id = ? and cliente = ? and paga = ?",
            *venta.id(),
            id_cliente,
            false
        )
        .fetch_optional(db)
        .await?;
        let model = match qres {
            Some(model) => model,
            None => return Err(AppError::IncorrectError(String::from("Id inexistente"))),
        };
        match model {
            Model::Venta {
                id,
                time: _,
                monto_total: _,
                monto_pagado: _,
                cliente,
                cerrada: _,
                paga: _,
                pos: _,
            } => {
                let cli_id = cliente.unwrap();
                if cli_id == id_cliente {
                    let venta = Mapper::venta(db, model, user).await?;
                    sqlx::query!("update ventas set paga = ? where id = ?", id, true)
                        .execute(db)
                        .await;
                    Ok(venta)
                } else {
                    Err(AppError::IncorrectError(String::from("Cliente Incorrecto")))
                }
            }
            _ => Err(AppError::IncorrectError(String::from("se esperaba venta"))),
        }
    }
    pub async fn pagar_deuda_general(id: i32, db: &Pool<Sqlite>, mut monto: f32) -> Res<f32> {
        let qres: Vec<Model> = sqlx::query_as!(
            Model::Venta,
            "select * from ventas where cliente = ? and paga = ?",
            id,
            false
        )
        .fetch_all(db)
        .await?;
        let resto = monto
            - qres
                .iter()
                .map(|model| match model {
                    Model::Venta {
                        id,
                        time,
                        monto_total,
                        monto_pagado,
                        cliente,
                        cerrada,
                        paga,
                        pos,
                    } => monto_total - monto_pagado,
                    _ => panic!("se esperaba venta"),
                })
                .sum::<f64>() as f32;
        for model in qres {
            if monto <= 0.0 {
                break;
            }
            match model{
                Model::Venta { id, time, monto_total, monto_pagado, cliente, cerrada, paga, pos }=>{
                    monto=monto+monto_pagado as f32 - monto_total as f32;
                    let id_venta=id;
                    let qres:Vec<Model>=sqlx::query_as!(Model::Pago, "select * from pagos where venta = ? and medio_pago = ?",id_venta, 0).fetch_all(db).await?;
                    for model in qres{
                        match model{
                            Model::Pago { id, medio_pago, monto, pagado, venta }=>{
                                
                            },
                            _=>return Err(AppError::IncorrectError(String::from("Se esperaba Pago")))
                        }
                    }
                },
                _=>return Err(AppError::IncorrectError(String::from("Se esperaba venta"))),
            }
        }

        // TODO!---------------------------------

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
