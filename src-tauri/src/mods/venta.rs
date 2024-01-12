use entity::{
    pago, relacion_venta_prod,
    venta::{self},
};
use sea_orm::{Database, DbErr, EntityTrait, Set};
use serde::Serialize;

use crate::redondeo;

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
        self.pagos.push(Pago::new(medio_pago.to_string(), monto));
        self.monto_pagado += monto;
        self.monto_total - self.monto_pagado
    }
    pub fn agregar_producto(&mut self, producto: Valuable, politica: f64) -> Venta {
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    Valuable::Pes(a) => prod = Valuable::Pes((a.0 + 1.0, a.1.clone())),
                    Valuable::Prod(a) => prod = Valuable::Prod((a.0 + 1, a.1.clone())),
                    Valuable::Rub(a) => self.productos.push(Valuable::Rub(a.clone())),
                }
                self.productos.insert(i, prod);
                esta = true;
            }
        }
        if !esta {
            let prod = match producto {
                Valuable::Pes(a) => Valuable::Pes((a.0 + 1.0, a.1.clone())),
                Valuable::Prod(a) => Valuable::Prod((a.0 + 1, a.1.clone())),
                Valuable::Rub(a) => Valuable::Rub((a.0 + 1, a.1.clone())),
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
                Valuable::Pes(a) => {
                    self.monto_total += redondeo(politica, a.0 as f64 * a.1.precio_peso)
                }
                Valuable::Prod(a) => self.monto_total += a.1.precio_de_venta * a.0 as f64,
                Valuable::Rub(a) => self.monto_total += a.1.monto * a.0 as f64,
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
                    Valuable::Pes(a) => {
                        if a.0 > 1.0 {
                            prod = Valuable::Pes((a.0 - 1.0, a.1.clone()))
                        }
                    }
                    Valuable::Prod(a) => {
                        if a.0 > 1 {
                            prod = Valuable::Prod((a.0 - 1, a.1.clone()))
                        }
                    }
                    Valuable::Rub(a) => {
                        if a.0 > 1 {
                            prod = Valuable::Rub((a.0 - 1, a.1.clone()))
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
                    Valuable::Pes(a) => prod = Valuable::Pes((a.0 + 1.0, a.1.clone())),
                    Valuable::Prod(a) => prod = Valuable::Prod((a.0 + 1, a.1.clone())),
                    Valuable::Rub(a) => prod = Valuable::Rub((a.0 + 1, a.1.clone())),
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
        let model = venta::ActiveModel {
            monto_total: Set(self.monto_total),
            monto_pagado: Set(self.monto_pagado),
            ..Default::default()
        };

        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;

        println!("conectado");
        let result = entity::venta::Entity::insert(model.clone())
            .exec(&db)
            .await?;

        let a = entity::venta::Entity::find_by_id(result.last_insert_id)
            .one(&db)
            .await?;

        if let Some(model) = a {
            let pay_models: Vec<pago::ActiveModel> = self
                .pagos
                .iter()
                .map(|x| pago::ActiveModel {
                    medio_pago: Set(x.get_medio().clone()),
                    monto: Set(x.get_monto()),
                    venta: Set(model.clone().id),
                    ..Default::default()
                })
                .collect();

            entity::pago::Entity::insert_many(pay_models)
                .exec(&db)
                .await?;
            let prod_models:Vec<entity::relacion_venta_prod::ActiveModel> = self
                .productos
                .iter()
                .map(|x| entity::relacion_venta_prod::ActiveModel {
                    producto: Set(match x {
                        Valuable::Prod(a) => a.1.get_id(),
                        Valuable::Pes(a) => a.1.id,
                        Valuable::Rub(a) => a.1.id,
                    }),
                    venta: Set(model.clone().id),
                    ..Default::default()
                })
                .collect();
        }
        Ok(())
    }
}
