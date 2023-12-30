pub mod producto;
pub mod sistema;
pub mod valuable;
use self::lib::{crear_file, leer_file};
use self::producto::Producto;
use self::valuable::Valuable;
use crate::redondeo;
use serde::{Deserialize, Serialize};
mod lib;
use entity::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::Database;
use std::{
    borrow::BorrowMut,
    fmt::{self, Display},
};
use tauri::async_runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    politica_redondeo: f64,
    formato_producto: Formato,
    modo_mayus: Mayusculas,
    cantidad_productos: usize,
    medios_pago: Vec<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Formato {
    #[default]
    Tmv,
    Mtv,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mayusculas {
    #[default]
    Upper,
    Lower,
    Camel,
}




#[derive(Debug, Clone, Default, Serialize)]
pub struct Venta {
    monto_total: f64,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f64,
}
#[derive(Debug, Clone, Default, Serialize)]
pub struct Pago {
    medio_pago: String,
    monto: f64,
}
impl Pago {
    pub fn new(medio_pago: String, monto: f64) -> Pago {
        Pago { medio_pago, monto }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RelacionProdProv {
    id_producto: i64,
    id_proveedor: i64,
    codigo_interno: Option<i64>,
}







fn camalize(data: String) -> String {
    let mut es: bool = true;
    for i in data.chars() {
        if es {
            i.to_uppercase();
        } else {
            i.to_lowercase();
        }
        if i == ' ' {
            es = true;
        } else {
            es = false;
        }
    }
    data.to_string()
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i64,
    nombre: String,
    contacto: Option<i64>,
}

//-----------------------------------Implementations---------------------------------

impl Config {
    pub fn get_cantidad_productos(&self) -> usize {
        self.cantidad_productos
    }
    pub fn get_medios_pago(&self) -> Vec<String> {
        self.medios_pago.clone()
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            politica_redondeo: 10.0,
            formato_producto: Formato::default(),
            modo_mayus: Mayusculas::default(),
            cantidad_productos: 10,
            medios_pago: vec![
                "Efectivo".to_owned(),
                "Crédito".to_owned(),
                "Débito".to_owned(),
            ],
        }
    }
}
impl RelacionProdProv {
    pub fn new(id_producto: i64, id_proveedor: i64, codigo_interno: Option<i64>) -> Self {
        RelacionProdProv {
            id_producto,
            id_proveedor,
            codigo_interno,
        }
    }
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
    pub fn agregar_pago(&mut self, medio_pago: String, monto: f64) -> f64 {
        self.pagos.push(Pago::new(medio_pago, monto));
        self.monto_pagado += monto;
        self.monto_total - self.monto_pagado
    }
    fn agregar_producto(&mut self, producto: Valuable, politica: f64) {
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
        self.monto_pagado -= pago.monto;
    }
    fn restar_producto(&mut self, producto: Valuable, politica: f64) -> Result<(), String> {
        let mut res = Err("Producto no encontrado".to_string());
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    Valuable::Pes(a) => prod = Valuable::Pes((a.0 - 1.0, a.1.clone())),
                    Valuable::Prod(a) => prod = Valuable::Prod((a.0 - 1, a.1.clone())),
                    Valuable::Rub(a) => prod = Valuable::Rub((a.0 - 1, a.1.clone())),
                }
                self.productos.insert(i, prod);
                res = Ok(());
            }
        }
        self.update_monto_total(politica);
        res
    }
    fn incrementar_producto(&mut self, producto: Valuable, politica: f64) -> Result<(), String> {
        let mut res = Err("Producto no encontrado".to_string());
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    Valuable::Pes(a) => prod = Valuable::Pes((a.0 + 1.0, a.1.clone())),
                    Valuable::Prod(a) => prod = Valuable::Prod((a.0 + 1, a.1.clone())),
                    Valuable::Rub(a) => prod = Valuable::Rub((a.0 + 1, a.1.clone())),
                }
                self.productos.insert(i, prod);
                res = Ok(());
            }
        }
        self.update_monto_total(politica);
        res
    }
    fn eliminar_producto(&mut self, producto: Valuable, politica: f64) -> Result<(), String> {
        let mut res = Err("Producto no encontrado".to_string());
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                self.productos.remove(i);
                res = Ok(());
                break;
            }
        }
        self.update_monto_total(politica);
        res
    }
}




impl Proveedor {
    pub fn new(id: i64, nombre: String, contacto: Option<i64>) -> Self {
        Proveedor {
            id,
            nombre,
            contacto,
        }
    }
    pub async fn save(&self) -> Result<(), String> {
        let model = proveedor::ActiveModel {
            id: Set(self.id),
            nombre: Set(self.nombre.clone()),
            contacto: Set(self.contacto),
        };
        match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
            Ok(db) => {
                println!("conectado");
                if let Err(e) = model.insert(&db).await {
                    Err(e.to_string())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
impl ToString for Proveedor {
    fn to_string(&self) -> String {
        let res;
        match self.contacto {
            Some(a) => res = format!("{} {}", self.nombre, a),
            None => res = format!("{}", self.nombre),
        }
        res
    }
}
