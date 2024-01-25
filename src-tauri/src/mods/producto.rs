use std::sync::Arc;

use super::{lib::Save, valuable::Presentacion, valuable::ValuableTrait};
use crate::redondeo;
use chrono::Utc;
use entity::{codigo_barras, producto};
use sea_orm::{ActiveModelTrait, Database, DbErr, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    id: i64,
    pub codigos_de_barras: Vec<i64>,
    precio_de_venta: f64,
    porcentaje: f64,
    precio_de_costo: f64,
    tipo_producto: Arc<str>,
    marca: Arc<str>,
    variedad: Arc<str>,
    presentacion: Presentacion,
}

impl Producto {
    pub fn new(
        id: i64,
        codigos_de_barras: Vec<i64>,
        precio_de_venta: f64,
        porcentaje: f64,
        precio_de_costo: f64,
        tipo_producto: &str,
        marca: &str,
        variedad: &str,
        presentacion: Presentacion,
    ) -> Producto {
        Producto {
            id,
            codigos_de_barras,
            precio_de_venta,
            porcentaje,
            precio_de_costo,
            tipo_producto: Arc::from(tipo_producto),
            marca: Arc::from(marca),
            variedad: Arc::from(variedad),
            presentacion,
        }
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_codigos_de_barras(&self) -> &Vec<i64> {
        &self.codigos_de_barras
    }
    pub fn get_precio_de_venta(&self) -> &f64 {
        &self.precio_de_venta
    }
    pub fn get_porcentaje(&self) -> &f64 {
        &self.porcentaje
    }
    pub fn get_precio_de_costo(&self) -> &f64 {
        &self.precio_de_costo
    }
    pub fn get_tipo_producto(&self) -> Arc<str> {
        Arc::clone(&self.tipo_producto)
    }
    pub fn get_marca(&self) -> Arc<str> {
        Arc::clone(&self.marca)
    }
    pub fn get_variedad(&self) -> Arc<str> {
        Arc::clone(&self.variedad)
    }
    pub fn get_presentacion(&self) -> &Presentacion {
        &self.presentacion
    }

    pub fn get_nombre_completo(&self) -> String {
        format!(
            "{} {} {} {}",
            self.marca, self.tipo_producto, self.variedad, self.presentacion
        )
    }
    pub fn rm_code(&mut self,i:usize){
        self.codigos_de_barras.remove(i);
    }

    // pub fn unifica_codes(&mut self) {
    //     let mut i=0;
    //     while i<self.codigos_de_barras.len(){
    //         let act=self.codigos_de_barras[i];
    //         let mut j=i+1;
    //         while j<self.codigos_de_barras.len(){
    //             if act==self.codigos_de_barras[j]{
    //                 self.codigos_de_barras.remove(j);
    //             }else{
    //                 j+=1;
    //             }
    //         }
    //         i+=1;
    //     }
    // }
}
impl Save for Producto {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        println!("Guardando producto en DB");
        let model = producto::ActiveModel {
            precio_de_venta: Set(self.precio_de_venta),
            porcentaje: Set(self.porcentaje),
            precio_de_costo: Set(self.precio_de_costo),
            tipo_producto: Set(self.tipo_producto.to_string()),
            marca: Set(self.marca.to_string()),
            variedad: Set(self.variedad.to_string()),
            presentacion: Set(format!("{}", self.presentacion)),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        let res = entity::producto::Entity::insert(model).exec(&db).await?;
        for codigo in &self.codigos_de_barras {
            let cod_model = codigo_barras::ActiveModel {
                codigo: Set(*codigo),
                producto: Set(res.last_insert_id),
                ..Default::default()
            };
            cod_model.insert(&db).await?;
        }
        Ok(())
    }
    
}

impl PartialEq for Producto {
    fn eq(&self, other: &Self) -> bool {
        let mut esta = false;
        for code in &self.codigos_de_barras {
            if other.codigos_de_barras.contains(&code) {
                esta = true;
            }
        }
        esta
    }
    
}

impl ValuableTrait for Producto {
    fn redondear(&self, politica: f64) -> Producto {
        Producto {
            id: self.id,
            codigos_de_barras: self.codigos_de_barras.clone(),
            precio_de_venta: redondeo(politica, self.precio_de_venta),
            porcentaje: self.porcentaje,
            precio_de_costo: self.precio_de_costo,
            tipo_producto: self.tipo_producto.clone(),
            marca: self.marca.clone(),
            variedad: self.variedad.clone(),
            presentacion: self.presentacion.clone(),
        }
    }
}

// impl Default for Producto{
//     fn default() -> Self {
//         Producto{
//             id: todo!(),
//             codigos_de_barras: todo!(),
//             precio_de_venta: todo!(),
//             porcentaje: todo!(),
//             precio_de_costo: todo!(),
//             tipo_producto: todo!(),
//             marca: todo!(),
//             variedad: todo!(),
//             presentacion: todo!(),
//         }
//     }
// }
