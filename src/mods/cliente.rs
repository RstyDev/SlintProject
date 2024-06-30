use chrono::NaiveDateTime;

use crate::{
    db::{
        map::{ClienteDB, FloatDB, PagoDB, VentaDB},
        Mapper,
    },
    ClienteFND, CuentaFND,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use slint::SharedString;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

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
        let model: Option<ClienteDB> = sqlx::query_as!(
            ClienteDB,
            r#"select id as "id:_", nombre, dni as "dni: _", limite as "limite: _", activo, time from clientes where dni = ? limit 1"#,
            dni
        )
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
    pub fn build(
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
        let model: sqlx::Result<Vec<FloatDB>> = sqlx::query_as!(
            FloatDB,
            r#"select monto as "float:_" from deudas where id = ? "#,
            self.id
        )
        .fetch_all(db)
        .await;
        Ok(model?.iter().map(|e| e.float).sum::<f32>())
    }
    pub async fn get_deuda_detalle(
        &self,
        db: &Pool<Sqlite>,
        user: Option<Arc<User>>,
    ) -> Res<Vec<Venta>> {
        let mut ventas = Vec::new();
        let qres: Vec<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente, cerrada, paga, pos from ventas where cliente = ? and paga = ? "#,
            self.id,
            false
        )
        .fetch_all(db)
        .await?;
        for model in qres {
            ventas.push(Mapper::venta(db, model, &user).await?)
        }
        Ok(ventas)
    }

    pub async fn pagar_deuda_especifica(
        id_cliente: i64,
        db: &Pool<Sqlite>,
        venta: Venta,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let qres: Option<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente, cerrada, paga, pos from ventas where id = ? and cliente = ? and paga = ? "#,
            *venta.id(),
            id_cliente,
            false
        )
        .fetch_optional(db)
        .await?;
        let venta = match qres {
            Some(model) => model,
            None => return Err(AppError::IncorrectError(String::from("Id inexistente"))),
        };

        if venta.cliente.unwrap() == id_cliente {
            let venta = Mapper::venta(db, venta, user).await?;
            sqlx::query!(
                "update ventas set paga = ? where id = ? ",
                *venta.id(),
                true
            )
            .execute(db)
            .await?;
            Ok(venta)
        } else {
            Err(AppError::IncorrectError(String::from("Cliente Incorrecto")))
        }
    }
    pub async fn pagar_deuda_general(
        id: i64,
        db: &Pool<Sqlite>,
        mut monto_a_pagar: f32,
    ) -> Res<f32> {
        let qres: Vec<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente, cerrada, paga, pos from ventas where cliente = ? and paga = ? "#,
            id,
            false
        )
        .fetch_all(db)
        .await?;
        let resto = monto_a_pagar
            - qres
                .iter()
                .map(|model| model.monto_total - model.monto_pagado)
                .sum::<f32>();
        for venta in qres {
            if monto_a_pagar <= 0.0 {
                break;
            }

            let models: Vec<PagoDB> = sqlx::query_as!(
                PagoDB,
                r#"select id as "id:_", medio_pago as "medio_pago:_", monto as "monto:_", pagado as "pagado:_", venta as "venta:_" from pagos where venta = ? and medio_pago = ? "#,
                venta.id,
                0
            )
            .fetch_all(db)
            .await?;
            let mut completados: u8 = 0;
            for i in 0..models.len() {
                if monto_a_pagar <= 0.0 {
                    break;
                }
                if models[i].pagado < models[i].monto {
                    if monto_a_pagar >= (models[i].monto - models[i].pagado) {
                        monto_a_pagar -= models[i].monto - models[i].pagado;
                        completados += 1;
                        sqlx::query("update pagos set pagado = ? where id =?")
                            .bind(models[i].monto)
                            .bind(id)
                            .execute(db)
                            .await?;
                    } else {
                        sqlx::query("update pagos set pagado = ? where id = ?")
                            .bind(models[i].pagado + monto_a_pagar)
                            .bind(id)
                            .execute(db)
                            .await?;
                        monto_a_pagar = 0.0;
                    }
                }
            }
            if completados == models.len() as u8 {
                sqlx::query("update ventas set paga = ? where id = ?")
                    .bind(true)
                    .bind(venta.id)
                    .execute(db)
                    .await?;
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
    pub fn to_fnd(&self) -> ClienteFND {
        let mut reg = ClienteFND::default();
        match self {
            Cliente::Final => reg.regular = false,
            Cliente::Regular(cli) => {
                reg.regular = true;
                reg.activo = cli.activo;
                reg.created = SharedString::from(cli.created.to_string());
                reg.dni = cli.dni; //TODO
            }
        }
        reg
    }
    pub fn from_fnd(cliente: ClienteFND) -> Self {
        match cliente.regular {
            false => Cliente::Final,
            true => Cliente::Regular(Cli::build(
                cliente.id,
                Arc::from(cliente.nombre.as_str()),
                cliente.dni,
                cliente.activo,
                cliente.created.as_str().parse().unwrap(),
                match cliente.limite.auth {
                    true => Some(cliente.limite.cuenta),
                    false => None,
                },
            )),
        }
    }
    pub async fn from_fnd_new(cliente: ClienteFND, db: &Pool<Sqlite>) -> Res<Cliente> {
        Ok(match cliente.regular {
            false => Cliente::Final,
            true => Cliente::Regular(
                Cli::new_to_db(
                    db,
                    cliente.nombre.as_str(),
                    cliente.dni,
                    cliente.activo,
                    Utc::now().naive_local(),
                    match cliente.limite.auth {
                        true => Some(cliente.limite.cuenta),
                        false => None,
                    },
                )
                .await?,
            ),
        })
    }
}

impl Default for Cliente {
    fn default() -> Self {
        Cliente::Final
    }
}
impl Cuenta {
    pub fn to_fnd(&self) -> CuentaFND {
        let mut cuenta = CuentaFND::default();
        cuenta.auth = match self {
            Cuenta::Auth(a) => {
                cuenta.cuenta = *a;
                true
            }
            Cuenta::Unauth => false,
        };
        cuenta
    }
}
