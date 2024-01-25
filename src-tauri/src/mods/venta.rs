use crate::redondeo;
use entity::{
    pago,
    venta::{self},
};
use sea_orm::{Database, DbErr, EntityTrait, Set};
use serde::Serialize;
use Valuable as V;

use super::{config::Config, error::AppError, lib::Save, pago::Pago, valuable::Valuable};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Venta {
    monto_total: f64,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f64,
}

impl<'a> Venta {
    pub fn new() -> Venta {
        Venta {
            monto_total: 0.0,
            productos: Vec::new(),
            pagos: Vec::new(),
            monto_pagado: 0.0,
        }
    }
    pub fn get_monto_total(&self) -> f64 {
        self.monto_total
    }
    pub fn get_productos(&self) -> Vec<Valuable> {
        self.productos.clone()
    }
    // pub fn get_pagos(&self)->Vec<Pago>{
    //     self.pagos.clone()
    // }
    pub fn get_monto_pagado(&self) -> f64 {
        self.monto_pagado
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f64) -> f64 {
        self.pagos.push(Pago::new(medio_pago, monto));
        self.monto_pagado += monto;
        self.monto_total - self.monto_pagado
    }
    pub fn 
    agregar_producto(&mut self, producto: Valuable, politica: f64) -> Venta {
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
    fn update_monto_total(&mut self, politica: f64) {
        self.monto_total = 0.0;
        for i in &self.productos {
            match &i {
                V::Pes(a) => {
                    self.monto_total += redondeo(politica, a.0 as f64 * a.1.get_precio_peso())
                }
                V::Prod(a) => self.monto_total += a.1.get_precio_de_venta() * a.0 as f64,
                V::Rub(a) => self.monto_total += a.1.get_monto() * a.0 as f64,
            }
        }
    }
    pub fn eliminar_pago(&mut self, index: usize) {
        let pago = self.pagos.remove(index);
        self.monto_pagado -= pago.get_monto();
    }
    pub fn restar_producto(
        &mut self,
        producto: Valuable,
        politica: f64,
        conf: &Config,
    ) -> Result<Venta, AppError> {
        let mut res = Err(AppError::ProductNotFound(producto.get_descripcion(conf)));
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
        politica: f64,
        conf: &Config,
    ) -> Result<Venta, AppError> {
        let mut res = Err(AppError::ProductNotFound(producto.get_descripcion(conf)));
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
        politica: f64,
        conf: &Config,
    ) -> Result<Venta, AppError> {
        let mut res = Err(AppError::ProductNotFound(producto.get_descripcion(conf)));
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
        let model_venta = venta::ActiveModel {
            monto_total: Set(self.monto_total),
            monto_pagado: Set(self.monto_pagado),
            ..Default::default()
        };

        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;

        println!("conectado");
        let result = entity::venta::Entity::insert(model_venta).exec(&db).await?;

        let saved_sale = entity::venta::Entity::find_by_id(result.last_insert_id)
            .one(&db)
            .await?;

        if let Some(saved_model) = saved_sale {
            let pay_models: Vec<pago::ActiveModel> = self
                .pagos
                .iter()
                .map(|x| pago::ActiveModel {
                    medio_pago: Set(x.get_medio().to_string()),
                    monto: Set(x.get_monto()),
                    venta: Set(saved_model.clone().id),
                    ..Default::default()
                })
                .collect();

            entity::pago::Entity::insert_many(pay_models)
                .exec(&db)
                .await?;
            let prod_models: Vec<entity::relacion_venta_prod::ActiveModel> = self
                .productos
                .iter()
                .filter_map(|x| match x {
                    V::Prod(a) => Some(entity::relacion_venta_prod::ActiveModel {
                        producto: Set(a.1.get_id()),
                        venta: Set(saved_model.id),
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
                        cantidad: Set(a.0 as i16),
                        rubro: Set(*a.1.get_id()),
                        venta: Set(saved_model.id),
                        ..Default::default()
                    }),
                    _ => None,
                })
                .collect();
            entity::relacion_venta_rub::Entity::insert_many(rub_models)
                .exec(&db)
                .await?;
            let pes_models: Vec<entity::relacion_venta_pes::ActiveModel> = self
                .productos
                .iter()
                .filter_map(|x| match x {
                    V::Pes(a) => Some(entity::relacion_venta_pes::ActiveModel {
                        cantidad: Set(a.0),
                        pesable: Set(*a.1.get_id()),
                        venta: Set(saved_model.id),
                        ..Default::default()
                    }),
                    _ => None,
                })
                .collect();
            entity::relacion_venta_pes::Entity::insert_many(pes_models)
                .exec(&db)
                .await?;
        }
        Ok(())
    }
}
