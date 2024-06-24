use super::{redondeo, AppError, Presentacion, Res, ValuableTrait};
use crate::db::map::BigIntDB;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    id: i64,
    codigos_de_barras: Vec<i64>,
    precio_de_venta: f32,
    porcentaje: f32,
    precio_de_costo: f32,
    tipo_producto: Arc<str>,
    marca: Arc<str>,
    variedad: Arc<str>,
    presentacion: Presentacion,
    proveedores: Vec<RelacionProdProv>
}

impl Producto {
    pub fn build(
        id: i64,
        codigos_de_barras: Vec<i64>,
        precio_de_venta: f32,
        porcentaje: f32,
        precio_de_costo: f32,
        tipo_producto: &str,
        marca: &str,
        variedad: &str,
        presentacion: Presentacion,
        proveedores: Vec<RelacionProdProv>
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
            proveedores
        }
    }
    pub fn id(&self) -> &i64 {
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
    pub fn proveedores(&self)->&Vec<RelacionProdProv>{
        &self.proveedores
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
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from productos where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(model) => {
                sqlx::query("delete from productos where id = ?")
                    .bind(model.int)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: self.id.to_string(),
            }),
        }
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
    pub async fn editar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from productos where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(model) => {
                if self.precio_de_venta != self.precio_de_costo * (1.0 + self.porcentaje / 100.0) {
                    return Err(AppError::IncorrectError(String::from(
                        "Cálculo de precio incorrecto",
                    )));
                }
                sqlx::query(
                    "update productos set precio_venta = ?, porcentaje = ?, precio_costo = ?, tipo = ?, marca = ?, variedad = ?, presentacion = ?, size = ?, updated_at = ? where id = ?")
                    .bind(self.precio_de_venta).bind(self.porcentaje).bind(self.precio_de_costo).bind(self.tipo_producto.as_ref()).bind(self.marca.as_ref()).bind(self.variedad.as_ref()).bind(self.presentacion.get_string()).bind(self.presentacion.get_cantidad()).bind(Utc::now().naive_local()).bind(model.int).execute(db).await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: self.id.to_string(),
            }),
        }
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
            proveedores: self.proveedores.clone()
        }
    }
}


#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct RelacionProdProv {
    proveedor: i64,
    codigo_interno: Option<i64>,
}

impl RelacionProdProv {
    pub fn new(proveedor: i64, codigo_interno: Option<i64>) -> Self {
        RelacionProdProv {
            proveedor,
            codigo_interno,
        }
    }
    
    pub fn proveedor(&self) -> &i64 {
        &self.proveedor
    }
    pub fn codigo_interno(&self) -> Option<i64> {
        self.codigo_interno
    }
}
