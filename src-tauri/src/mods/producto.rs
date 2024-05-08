use std::sync::Arc;

use super::{redondeo, AppError, Presentacion, Res, Save, ValuableTrait};
use chrono::Utc;
use entity::prelude::{CodeDB, ProdDB};
use sea_orm::{
    ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    id: i32,
    codigos_de_barras: Vec<i64>,
    precio_de_venta: f32,
    porcentaje: f32,
    precio_de_costo: f32,
    tipo_producto: Arc<str>,
    marca: Arc<str>,
    variedad: Arc<str>,
    presentacion: Presentacion,
}

impl Producto {
    pub fn new(
        id: i32,
        codigos_de_barras: Vec<i64>,
        precio_de_venta: f32,
        porcentaje: f32,
        precio_de_costo: f32,
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
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn codigos_de_barras(&self) -> &Vec<i64> {
        &self.codigos_de_barras
    }
    pub fn precio_de_venta(&self) -> &f32 {
        &self.precio_de_venta
    }
    pub fn porcentaje(&self) -> &f32 {
        &self.porcentaje
    }
    pub fn precio_de_costo(&self) -> &f32 {
        &self.precio_de_costo
    }
    pub fn tipo_producto(&self) -> Arc<str> {
        Arc::clone(&self.tipo_producto)
    }
    pub fn marca(&self) -> Arc<str> {
        Arc::clone(&self.marca)
    }
    pub fn variedad(&self) -> Arc<str> {
        Arc::clone(&self.variedad)
    }
    pub fn presentacion(&self) -> &Presentacion {
        &self.presentacion
    }

    // pub fn nombre_completo(&self) -> String {
    //     format!(
    //         "{} {} {} {}",
    //         self.marca, self.tipo_producto, self.variedad, self.presentacion
    //     )
    // }
    // pub fn rm_code(&mut self, i: usize) {
    //     self.codigos_de_barras.remove(i);
    // }

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
    pub async fn eliminar(self, db: &DatabaseConnection) -> Res<()> {
        let model = match ProdDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => model.into_active_model(),
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Producto"),
                    instancia: self.id.to_string(),
                })
            }
        };
        model.delete(db).await?;
        Ok(())
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.tipo_producto,
            self.marca,
            self.variedad,
            self.presentacion.get_cantidad(),
            self.presentacion.get_string()
        )
    }
    pub async fn editar(self, db: &DatabaseConnection) -> Res<()> {
        let mut model = match ProdDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => model.into_active_model(),
            None => {
                return Err(AppError::NotFound {
                    objeto: String::from("Producto"),
                    instancia: self.id.to_string(),
                })
            }
        };
        if self.precio_de_venta == self.precio_de_costo * (1.0 + self.porcentaje / 100.0) {
            model.precio_de_venta = Set(self.precio_de_venta);
        } else {
            return Err(AppError::IncorrectError(String::from(
                "Cálculo de precio incorrecto",
            )));
        }
        model.cantidad = Set(self.presentacion.get_cantidad());
        model.marca = Set(self.marca.to_string());
        model.porcentaje = Set(self.porcentaje);
        model.precio_de_costo = Set(self.precio_de_costo);
        model.presentacion = Set(self.presentacion.get_string());
        model.tipo_producto = Set(self.tipo_producto.to_string());
        model.variedad = Set(self.variedad.to_string());
        model.updated_at = Set(Utc::now().naive_local());
        model.update(db).await?;
        Ok(())
    }
}
impl Save for Producto {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        println!("Guardando producto en DB");
        let model = ProdDB::ActiveModel {
            precio_de_venta: Set(self.precio_de_venta),
            porcentaje: Set(self.porcentaje),
            precio_de_costo: Set(self.precio_de_costo),
            tipo_producto: Set(self.tipo_producto.to_string()),
            marca: Set(self.marca.to_string()),
            variedad: Set(self.variedad.to_string()),
            presentacion: Set(self.presentacion.get_string().to_string()),
            updated_at: Set(Utc::now().naive_local()),
            cantidad: Set(self.presentacion().get_cantidad()),
            ..Default::default()
        };
        let res = ProdDB::Entity::insert(model).exec(&db).await?;
        for codigo in &self.codigos_de_barras {
            let cod_model = CodeDB::ActiveModel {
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
    fn redondear(&self, politica: &f32) -> Producto {
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
