use chrono::NaiveDateTime;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set, ActiveModelTrait
};
use serde::{Deserialize, Serialize};
use tauri::App;
use std::sync::Arc;

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
         &self,db: &DatabaseConnection,venta:Venta,user:Option<Arc<User>>
     )->Res<Venta>{
         let model=match entity::venta::Entity::find_by_id(*venta.id()).one(db).await?{
             Some(model) => model,
             None => return Err(AppError::IncorrectError(String::from("Id inexistente"))),
         };
         let venta=Mapper::map_model_sale(&model, db, &user).await?;
         match model.cliente{
             Some(cli) => if cli==self.id{
                 let mut model=model.into_active_model();
                 model.paga=Set(true);
                 model.update(db).await?;

             }else{
                 return Err(AppError::IncorrectError("Cliente Incorrecto".to_string()))
             },
            None => return Err(AppError::IncorrectError(String::from("Cliente Incorrecto"))),
         }
         Ok(venta)

     }
     pub async fn pagar_deuda_general(&self,db: &DatabaseConnection,mut monto:f64)->Res<f64>{
         let models=entity::venta::Entity::find().filter(Condition::all()
             .add(entity::venta::Column::Cliente.eq(self.id))
             .add(entity::venta::Column::Paga.eq(false))).all(db).await?;
        for model in models{
            let mut model = model.into_active_model();
            let deuda=entity::pago::Entity::find().filter(Condition::all()
                .add(entity::pago::Column::Venta.eq(model.id.unwrap()))
                .add(entity::pago::Column::MedioPago.eq(1))).all(db).await?.iter();
                
        }
        Ok(0.0)
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
