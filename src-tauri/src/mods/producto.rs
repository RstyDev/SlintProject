use crate::redondeo;
use super::{valuable::Presentacion, valuable::ValuableTrait};
use entity::{codigo_barras, producto};
use sea_orm::{Database, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Producto {
    pub id: i64,
    pub codigos_de_barras: Vec<i64>,
    pub precio_de_venta: f64,
    pub porcentaje: f64,
    pub precio_de_costo: f64,
    pub tipo_producto: String,
    pub marca: String,
    pub variedad: String,
    pub presentacion: Presentacion,
}

impl Producto {
    pub fn new(
        id: i64,
        codigos: Vec<&str>,
        precio_de_venta: &str,
        porcentaje: &str,
        precio_de_costo: &str,
        tipo_producto: &str,
        marca: &str,
        variedad: &str,
        cantidad: &str,
        presentacion: &str,
    ) -> Producto {
        let cant = match presentacion {
            "Gr" => Presentacion::Gr(cantidad.parse().unwrap()),
            "Un" => Presentacion::Un(cantidad.parse().unwrap()),
            "Lt" => Presentacion::Lt(cantidad.parse().unwrap()),
            _ => panic!("no posible {presentacion}"),
        };
        let codigos = codigos
            .iter()
            .map(|code| -> i64 { code.parse().unwrap() })
            .collect();
        Producto {
            id,
            codigos_de_barras: codigos,
            precio_de_venta: precio_de_venta.parse().unwrap(),
            porcentaje: porcentaje.parse().unwrap(),
            precio_de_costo: precio_de_costo.parse().unwrap(),
            tipo_producto: tipo_producto.to_string(),
            marca: marca.to_string(),
            variedad: variedad.to_string(),
            presentacion: cant,
        }
    }
    pub fn get_nombre_completo(&self) -> String {
        format!(
            "{} {} {} {}",
            self.marca, self.tipo_producto, self.variedad, self.presentacion
        )
    }
    pub async fn save(&self) -> Result<(), String> {
        match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
            Ok(db) => {
                println!("conectado");
                let model = producto::ActiveModel {
                    id: Set(self.id),
                    precio_de_venta: Set(self.precio_de_venta),
                    porcentaje: Set(self.porcentaje),
                    precio_de_costo: Set(self.precio_de_costo),
                    tipo_producto: Set(self.tipo_producto.clone()),
                    marca: Set(self.marca.clone()),
                    variedad: Set(self.variedad.clone()),
                    presentacion: Set(format!("{}",self.presentacion)),
                };
                if let Err(e) = model.insert(&db).await {
                    return Err(e.to_string());
                }
                for codigo in &self.codigos_de_barras {
                    let cod_model = codigo_barras::ActiveModel {
                        id: Set(*codigo),
                        producto: Set(self.id),
                    };
                    if let Err(e)=cod_model.insert(&db).await{
                        return Err(e.to_string());
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }
    pub fn unifica_codes(&mut self){
        let mut e=0;
        for i in 0..self.codigos_de_barras.len(){
            let act=self.codigos_de_barras[i];
            for j in i..self.codigos_de_barras.len(){
                if act==self.codigos_de_barras[j-e]{
                    self.codigos_de_barras.remove(j-e);
                    e+=1;
                }
            }
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