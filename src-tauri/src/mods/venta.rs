use chrono::Utc;
use entity::pago;
type Res<T> = std::result::Result<T, AppError>;
use sea_orm::{
    ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryOrder, Set,
};
use serde::Serialize;
use std::sync::Arc;
use tauri::async_runtime;

use Valuable as V;

use crate::mods::pago::medio_from_db;

use super::{
    config::Config,
    error::AppError,
    lib::{redondeo, Save},
    pago::{MedioPago, Pago},
    valuable::Valuable,
    vendedor::Vendedor,
};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Venta {
    id: i64,
    monto_total: f64,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f64,
    vendedor: Arc<Vendedor>,
}

impl<'a> Venta {
    pub async fn new(vendedor: Arc<Vendedor>, db: &DatabaseConnection) -> Res<Venta> {
        let venta = entity::venta::Entity::find()
            .order_by_desc(entity::venta::Column::Id)
            .one(db)
            .await?;
        let id = match venta {
            Some(a) => a.id + 1,
            None => 0,
        };
        let venta = Venta {
            monto_total: 0.0,
            productos: Vec::new(),
            pagos: Vec::new(),
            monto_pagado: 0.0,
            vendedor,
            id,
        };
        entity::venta::ActiveModel {
            id: Set(venta.id),
            monto_total: Set(venta.monto_total),
            monto_pagado: Set(venta.monto_pagado),
            time: Set(Utc::now().naive_local()),
        }
        .insert(db)
        .await?;
        Ok(venta)
    }
    pub fn monto_total(&self) -> f64 {
        self.monto_total
    }
    pub fn productos(&self) -> Vec<Valuable> {
        self.productos.clone()
    }
    // pub fn get_pagos(&self)->Vec<Pago>{
    //     self.pagos.clone()
    // }
    pub fn monto_pagado(&self) -> f64 {
        self.monto_pagado
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f64) -> f64 {
        let model = async_runtime::block_on(medio_from_db(medio_pago));
        let medio_pago = MedioPago::new(&model.medio, model.id);
        self.pagos.push(Pago::new(medio_pago, monto));
        self.monto_pagado += monto;
        self.monto_total - self.monto_pagado
    }
    pub fn agregar_producto(&mut self, producto: Valuable, politica: &f64) -> Venta {
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    V::Pes(a) => prod = V::Pes((a.0 + 1.0, a.1.clone())),
                    V::Prod(a) => prod = V::Prod((a.0 + 1, a.1.clone())),
                    V::Rub(a) => self.productos.push(V::Rub(a.clone())),
                }
                self.productos.insert(i, prod);
                esta = true;
            }
        }
        if !esta {
            let prod = match producto {
                V::Pes(a) => V::Pes((a.0 + 1.0, a.1.clone())),
                V::Prod(a) => V::Prod((a.0 + 1, a.1.clone())),
                V::Rub(a) => V::Rub((a.0 + 1, a.1.clone())),
            };
            self.productos.push(prod);
        }
        self.update_monto_total(politica);
        self.clone()
    }
    fn update_monto_total(&mut self, politica: &f64) {
        self.monto_total = 0.0;
        for i in &self.productos {
            match &i {
                V::Pes(a) => self.monto_total += redondeo(politica, a.0 as f64 * a.1.precio_peso()),
                V::Prod(a) => self.monto_total += a.1.precio_de_venta() * a.0 as f64,
                V::Rub(a) => self.monto_total += a.1.monto().unwrap() * a.0 as f64,
            }
        }
    }
    pub fn eliminar_pago(&mut self, index: usize) {
        let pago = self.pagos.remove(index);
        self.monto_pagado -= pago.monto();
    }
    pub fn restar_producto(
        &mut self,
        producto: Valuable,
        politica: &f64,
        conf: &Config,
    ) -> Result<Venta, AppError> {
        let mut res = Err(AppError::ProductNotFound(producto.descripcion(conf)));
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    V::Pes(a) => {
                        if a.0 > 1.0 {
                            prod = V::Pes((a.0 - 1.0, a.1.clone()))
                        }
                    }
                    V::Prod(a) => {
                        if a.0 > 1 {
                            prod = V::Prod((a.0 - 1, a.1.clone()))
                        }
                    }
                    V::Rub(a) => {
                        if a.0 > 1 {
                            prod = V::Rub((a.0 - 1, a.1.clone()))
                        }
                    }
                }
                self.productos.insert(i, prod);
                esta = true;
            }
        }
        self.update_monto_total(politica);
        if esta {
            res = Ok(self.clone());
        }
        res
    }
    pub fn incrementar_producto(
        &mut self,
        producto: Valuable,
        politica: &f64,
        conf: &Config,
    ) -> Result<Venta, AppError> {
        let mut res = Err(AppError::ProductNotFound(producto.descripcion(conf)));
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                esta = true;
                let mut prod = self.productos.remove(i);
                match &prod {
                    V::Pes(a) => prod = V::Pes((a.0 + 1.0, a.1.clone())),
                    V::Prod(a) => prod = V::Prod((a.0 + 1, a.1.clone())),
                    V::Rub(a) => prod = V::Rub((a.0 + 1, a.1.clone())),
                }
                self.productos.insert(i, prod);
            }
        }
        self.update_monto_total(politica);
        if esta {
            res = Ok(self.clone());
        }
        res
    }
    pub fn eliminar_producto(
        &mut self,
        producto: Valuable,
        politica: &f64,
        conf: &Config,
    ) -> Result<Venta, AppError> {
        let mut res = Err(AppError::ProductNotFound(producto.descripcion(conf)));
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                self.productos.remove(i);
                esta = true;
                break;
            }
        }
        self.update_monto_total(politica);
        if esta {
            res = Ok(self.clone());
        }
        res
    }
}
impl Save for Venta {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        let mut venta = entity::venta::Entity::find_by_id(self.id)
            .one(&db)
            .await?
            .unwrap()
            .into_active_model();
        venta.monto_total = Set(self.monto_total);
        venta.monto_pagado = Set(self.monto_pagado);
        venta.time = Set(Utc::now().naive_local());

        venta.update(&db).await?;

        let mut pay_models = vec![];
        for pago in &self.pagos {
            let model = medio_from_db(&pago.medio().to_string().as_str()).await;
            pay_models.push(pago::ActiveModel {
                medio_pago: Set(model.id),
                monto: Set(pago.monto()),
                venta: Set(self.id),
                ..Default::default()
            });
        }
        println!("pay models: {pay_models:#?}");
        let aux;
        if pay_models.len() > 1 {
            aux = entity::pago::Entity::insert_many(pay_models)
                .exec(&db)
                .await;
            println!("{aux:#?}");
            aux?;
        } else {
            aux = entity::pago::Entity::insert(pay_models[0].clone())
                .exec(&db)
                .await;
            println!("{aux:#?}");
            aux?;
        }
        println!("control");

        let prod_models: Vec<entity::relacion_venta_prod::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                V::Prod(a) => Some(entity::relacion_venta_prod::ActiveModel {
                    producto: Set(*a.1.id()),
                    venta: Set(self.id),
                    cantidad: Set(a.0),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        entity::relacion_venta_prod::Entity::insert_many(prod_models)
            .exec(&db)
            .await?;

        let rub_models: Vec<entity::relacion_venta_rub::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                V::Rub(a) => Some(entity::relacion_venta_rub::ActiveModel {
                    cantidad: Set(a.0),
                    rubro: Set(*a.1.id()),
                    venta: Set(self.id),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        let aux;
        if rub_models.len() > 1 {
            aux = entity::relacion_venta_rub::Entity::insert_many(rub_models)
                .exec(&db)
                .await;
            println!("{aux:#?}");
            aux?;
        } else if rub_models.len() == 1 {
            aux = entity::relacion_venta_rub::Entity::insert(rub_models[0].clone())
                .exec(&db)
                .await;
            println!("{aux:#?}");
            aux?;
        }
        let pes_models: Vec<entity::relacion_venta_pes::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                V::Pes(a) => Some(entity::relacion_venta_pes::ActiveModel {
                    cantidad: Set(a.0),
                    pesable: Set(*a.1.id()),
                    venta: Set(self.id),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        if pes_models.len() > 1 {
            entity::relacion_venta_pes::Entity::insert_many(pes_models)
                .exec(&db)
                .await?;
        } else if pes_models.len() == 1 {
            entity::relacion_venta_pes::Entity::insert(pes_models[0].clone())
                .exec(&db)
                .await?;
        }

        Ok(())
    }
}
