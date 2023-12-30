use super::{*, valuable::Presentacion};
use entity::producto;
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
        let model = producto::ActiveModel {
            id: Set(self.id),
            precio_de_venta: Set(self.precio_de_venta),
            porcentaje: Set(self.porcentaje),
            precio_de_costo: Set(self.precio_de_costo),
            tipo_producto: Set(self.tipo_producto.clone()),
            marca: Set(self.marca.clone()),
            variedad: Set(self.variedad.clone()),
            presentacion: Set(self.presentacion.to_string()),
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
