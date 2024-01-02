use entity::codigo_barras;
use entity::pesable;
use entity::producto;
use entity::rubro;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tauri::async_runtime;

use super::{
    config::Config,
    lib::{crear_file, leer_file},
    pesable::Pesable,
    producto::Producto,
    proveedor::Proveedor,
    relacion_prod_prov::RelacionProdProv,
    rubro::Rubro,
    valuable::{Valuable, ValuableTrait},
    venta::Venta,
};
pub struct Sistema {
    configs: Config,
    productos: Vec<Valuable>,
    ventas: (Venta, Venta),
    proveedores: Vec<Proveedor>,
    path_productos: String,
    path_proveedores: String,
    path_relaciones: String,
    path_configs: String,
    path_pesables: String,
    path_rubros: String,
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}

impl<'a> Sistema {
    pub fn new() -> Sistema {
        let path_productos = String::from("Productos.json");
        let path_proveedores = String::from("Proveedores.json");
        let path_relaciones = String::from("Relaciones.json");
        let path_configs = String::from("Configs.json");
        let path_rubros = String::from("Rubros.json");
        let path_pesables = String::from("Pesables.json");
        let mut productos: Vec<Producto> = Vec::new();
        let mut rubros: Vec<Rubro> = Vec::new();
        let mut pesables: Vec<Pesable> = Vec::new();
        let stash = Vec::new();
        let registro = Vec::new();
        if let Err(e) = leer_file(&mut rubros, &path_rubros) {
            panic!("{}", e);
        }
        if let Err(e) = leer_file(&mut pesables, &path_pesables) {
            panic!("{}", e);
        }
        if let Err(e) = leer_file(&mut productos, &path_productos) {
            panic!("{}", e);
        }

        let mut rubros: Vec<Valuable> = rubros
            .iter()
            .map(|a| Valuable::Rub((0, a.to_owned())))
            .collect();
        let mut pesables: Vec<Valuable> = pesables
            .iter()
            .map(|a| Valuable::Pes((0.0, a.to_owned())))
            .collect();
        let mut productos: Vec<Valuable> = productos
            .iter()
            .map(|a| Valuable::Prod((0, a.to_owned())))
            .collect();
        productos.append(&mut pesables);
        productos.append(&mut rubros);

        let mut proveedores: Vec<Proveedor> = Vec::new();
        if let Err(e) = leer_file(&mut proveedores, &path_proveedores) {
            panic!("{}", e);
        }
        let mut relaciones = Vec::new();
        if let Err(e) = leer_file(&mut relaciones, &path_relaciones) {
            panic!("{}", e);
        }
        let mut configs = Vec::<Config>::new();
        if let Err(e) = leer_file(&mut configs, &path_configs) {
            panic!("{}", e);
        }
        if configs.len() == 0 {
            configs.push(Config::default());
            if let Err(e) = crear_file(&path_configs, &mut configs) {
                panic!("{}", e);
            }
        }
        let mut sis = Sistema {
            configs: configs[0].clone(),
            productos,
            ventas: (Venta::new(), Venta::new()),
            proveedores,
            path_productos,
            path_proveedores,
            path_relaciones,
            path_configs,
            path_pesables,
            path_rubros,
            relaciones,
            stash,
            registro,
        };
        for i in 0..sis.productos.len(){
            sis.productos[i].unifica_codes()
        }
        if let Err(e) = async_runtime::block_on(sis.get_data_valuable()) {
            println!("{e}");
        }
        // if let Err(e) = async_runtime::block_on(sis.set_data_valuable()) {
        //     println!("{e}");
        // }
        sis
    }
    pub fn get_productos(&self) -> &Vec<Valuable> {
        &self.productos
    }
    pub fn get_productos_cloned(&self) -> Vec<Valuable> {
        self.productos.clone()
    }
    pub fn get_proveedores(&self) -> &Vec<Proveedor> {
        &self.proveedores
    }
    pub fn get_configs(&self) -> &Config {
        &self.configs
    }
    // pub fn get_venta_mut(&mut self, pos: usize) -> &mut Venta {
    //     if pos == 1 {
    //         self.ventas.1.borrow_mut()
    //     } else {
    //         self.ventas.0.borrow_mut()
    //     }
    // }
    pub fn agregar_pago(
        &mut self,
        medio_pago: String,
        monto: f64,
        pos: usize,
    ) -> Result<f64, String> {
        let error_msj = "error, hay solo dos posiciones para ventas".to_string();
        let res;
        match pos {
            0 => {
                if !medio_pago.eq("Efectivo")
                    && self.ventas.0.get_monto_pagado() + monto > self.ventas.0.get_monto_total()
                {
                    res = Err(format!(
                        "El monto no puede ser superior al resto con {medio_pago}"
                    ));
                } else {
                    res = Ok(self.ventas.0.agregar_pago(medio_pago, monto));
                }
            }
            1 => {
                if !medio_pago.eq("Efectivo")
                    && self.ventas.1.get_monto_pagado() + monto > self.ventas.1.get_monto_total()
                {
                    res = Err(format!(
                        "El monto no puede ser superior al resto con {medio_pago}"
                    ));
                } else {
                    res = Ok(self.ventas.1.agregar_pago(medio_pago, monto));
                }
            }
            _ => res = Err(error_msj),
        }
        if let Ok(a) = res {
            if a <= 0.0 {
                self.cerrar_venta(pos);
            }
        }
        res
    }
    pub fn eliminar_pago(&mut self, pos: usize, index: usize) -> Result<(), String> {
        match pos {
            0 => self.ventas.0.eliminar_pago(index),
            1 => self.ventas.1.eliminar_pago(index),
            _ => return Err("numero de venta incorrecto".to_string()),
        }
        Ok(())
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
        if let Err(e) = crear_file(&self.path_configs, &vec![&self.configs]) {
            panic!("{e}");
        }
    }
    pub fn imprimir(&self) {
        println!("Printed from rust");
    }
    fn proveedor_esta(&self, proveedor: &str) -> bool {
        let mut res = false;
        for i in &self.proveedores {
            if i.get_nombre().eq_ignore_ascii_case(proveedor) {
                res = true;
            }
        }
        res
    }
    pub fn agregar_producto(
        &mut self,
        proveedores: Vec<String>,
        codigos_prov: Vec<String>,
        producto: Producto,
    ) -> Result<(), String> {
        let mut res = Ok(());

        for i in 0..proveedores.len() {
            match codigos_prov[i].parse::<i64>() {
                Ok(a) => self.relaciones.push(RelacionProdProv::new(
                    self.productos.len() as i64,
                    i as i64,
                    Some(a),
                )),
                Err(_) => self.relaciones.push(RelacionProdProv::new(
                    self.productos.len() as i64,
                    i as i64,
                    None,
                )),
            };
        }
        let mut productos: Vec<Producto> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Prod(a) => Some(a.1.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        productos.push(producto.clone());
        if let Err(e) = crear_file(&self.path_productos, &productos) {
            res = Err(e.to_string());
        }
        if let Err(e) = crear_file(&self.path_relaciones, &self.relaciones) {
            res = Err(e.to_string());
        }
        if res.is_ok() {
            if let Err(e) = async_runtime::block_on(producto.clone().save()) {
                return Err(e.to_string());
            }
            self.productos.push(Valuable::Prod((0, producto)));
        }
        res
    }
    pub fn agregar_pesable(&mut self, pesable: Pesable) -> Result<(), String> {
        let mut res = Ok(());
        let mut pesables: Vec<Pesable> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Pes(a) => Some(a.1.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        pesables.push(pesable.clone());
        if let Err(e) = crear_file(&self.path_pesables, &pesables) {
            res = Err(e.to_string());
        }
        if res.is_ok() {
            if let Err(e) = async_runtime::block_on(pesable.clone().save()) {
                return Err(e.to_string());
            }
            self.productos.push(Valuable::Pes((0.0, pesable)));
        }
        res
    }

    pub fn agregar_rubro(&mut self, rubro: Rubro) -> Result<(), String> {
        let mut res = Ok(());
        let mut rubros: Vec<Rubro> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Rub(a) => Some(a.1.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        rubros.push(rubro.clone());
        if let Err(e) = crear_file(&self.path_rubros, &rubros) {
            res = Err(e.to_string());
        }
        if res.is_ok() {
            if let Err(e) = async_runtime::block_on(rubro.clone().save()) {
                return Err(e.to_string());
            }
            self.productos.push(Valuable::Rub((0, rubro)));
        }
        res
    }
    pub fn agregar_proveedor(&mut self, proveedor: &str, contacto: &str) -> Result<(), String> {
        let mut res = Ok(());
        if self.proveedor_esta(&proveedor) {
            res = Err("Proveedor existente".to_owned());
        } else {
            let prov;
            if contacto.len() > 0 {
                let contacto: String = contacto
                    .chars()
                    .filter(|x| -> bool { x.is_numeric() })
                    .collect();
                let contacto = match contacto.parse() {
                    Ok(a) => Some(a),
                    Err(_) => return Err("Error al convertir el numero".to_owned()),
                };
                prov = Proveedor::new(
                    self.proveedores.len() as i64 + 1,
                    proveedor.to_owned(),
                    contacto,
                );
            } else {
                prov = Proveedor::new(
                    self.proveedores.len() as i64 + 1,
                    proveedor.to_owned(),
                    None,
                );
            }
            if let Err(e) = async_runtime::block_on(prov.save()) {
                return Err(e.to_string());
            }
            self.proveedores.push(prov);
            if let Err(e) = crear_file(&self.path_proveedores, &self.proveedores) {
                res = Err(e.to_string());
            }
        }
        res
    }
    fn get_producto(&mut self, id: i64) -> Result<Valuable, String> {
        let mut res = Err("Producto no encontrado".to_string());
        for p in &self.productos {
            match p {
                Valuable::Pes(a) => {
                    if a.1.id == id {
                        res = Ok(p.clone());
                    }
                }
                Valuable::Prod(a) => {
                    if a.1.id == id {
                        res = Ok(p.clone());
                    }
                }
                Valuable::Rub(a) => {
                    if a.1.id == id {
                        res = Ok(p.clone());
                    }
                }
            }
        }
        res
    }
    pub fn agregar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<Venta, String> {
        let res = self
            .get_producto(id)?
            .redondear(self.get_configs().get_politica());
        let result;
        match pos {
            0 => {
                result = Ok(self
                    .ventas
                    .0
                    .agregar_producto(res, self.get_configs().get_politica()))
            }
            1 => {
                result = Ok(self
                    .ventas
                    .1
                    .agregar_producto(res, self.get_configs().get_politica()))
            }
            _ => result = Err("Numero de venta incorrecto".to_string()),
        }

        result
    }
    pub fn descontar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<Venta, String> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self
                    .ventas
                    .0
                    .restar_producto(res, self.get_configs().get_politica());
            }
            1 => {
                result = self
                    .ventas
                    .1
                    .restar_producto(res, self.get_configs().get_politica());
            }
            _ => result = Err("Numero de venta incorrecto".to_string()),
        }
        result
    }
    pub fn incrementar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<Venta, String> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self
                    .ventas
                    .0
                    .incrementar_producto(res, self.get_configs().get_politica());
            }
            1 => {
                result = self
                    .ventas
                    .1
                    .incrementar_producto(res, self.get_configs().get_politica());
            }
            _ => result = Err("Numero de venta incorrecto".to_string()),
        }
        result
    }
    pub fn eliminar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<Venta, String> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self
                    .ventas
                    .0
                    .eliminar_producto(res, self.get_configs().get_politica());
            }
            1 => {
                result = self
                    .ventas
                    .1
                    .eliminar_producto(res, self.get_configs().get_politica());
            }
            _ => result = Err("Numero de venta incorrecto".to_string()),
        }
        result
    }
    pub fn get_venta(&self, pos: usize) -> Venta {
        let res;
        if pos == 0 {
            res = self.ventas.0.clone();
        } else {
            res = self.ventas.1.clone();
        }
        res
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Vec<String> {
        let iter = self.productos.iter();
        let mut res: Vec<String> = iter
            .filter_map(|x| match x {
                Valuable::Prod(a) => {
                    if a.1.marca.to_lowercase().contains(&filtro.to_lowercase()) {
                        Some(a.1.marca.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        res.sort();
        res.dedup();

        res
    }

    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Vec<String> {
        let iter = self.productos.iter();
        let mut res: Vec<String> = iter
            .filter_map(|x| match x {
                Valuable::Prod(a) => {
                    if a.1
                        .tipo_producto
                        .to_lowercase()
                        .contains(&filtro.to_lowercase())
                    {
                        Some(a.1.tipo_producto.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        res.sort();
        res.dedup();
        res
    }
    fn cerrar_venta(&mut self, pos: usize) {
        match pos {
            0 => {
                self.registro.push(self.ventas.0.clone());
                self.ventas.0 = Venta::new();
            }
            1 => {
                self.registro.push(self.ventas.1.clone());
                self.ventas.1 = Venta::new();
            }
            _ => panic!("error, solo hay 2 posiciones para ventas"),
        };
    }
    pub fn stash_sale(&mut self, pos: usize) {
        match pos {
            0 => {
                self.stash.push(self.ventas.0.clone());
                self.ventas.0 = Venta::new();
            }
            1 => {
                self.stash.push(self.ventas.1.clone());
                self.ventas.1 = Venta::new();
            }
            _ => panic!("error, solo hay 2 posiciones para ventas"),
        };
    }
    pub fn unstash_sale(&mut self, pos: usize, index: usize) -> Result<(), String> {
        match pos {
            0 => {
                self.ventas.0 = self.stash.remove(index);
                Ok(())
            }
            1 => {
                self.stash.push(self.ventas.1.clone());
                self.ventas.1 = self.stash.remove(index);
                Ok(())
            }
            _ => Err("error, solo hay 2 posiciones para ventas".to_string()),
        }
    }
    pub fn get_stash(&self) -> Vec<Venta> {
        self.stash.clone()
    }
    // pub async fn set_data_valuable(&self) -> Result<(), String> {
    //     println!("Guardando productos en DB");
    //     match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
    //         Ok(db) => {
    //             for producto in &self.productos {
    //                 match producto {
    //                     Valuable::Prod(a) => {
    //                         let model = producto::ActiveModel {
    //                             id: Set(a.1.id),
    //                             precio_de_venta: Set(a.1.precio_de_venta),
    //                             porcentaje: Set(a.1.porcentaje),
    //                             precio_de_costo: Set(a.1.precio_de_costo),
    //                             tipo_producto: Set(a.1.tipo_producto.clone()),
    //                             marca: Set(a.1.marca.clone()),
    //                             variedad: Set(a.1.variedad.clone()),
    //                             presentacion: Set(format!("{}", a.1.presentacion)),
    //                         };
    //                         if let Err(e) = model.insert(&db).await {
    //                             return Err(e.to_string());
    //                         }
    //                         for codigo in &a.1.codigos_de_barras {
    //                             let cod_model = codigo_barras::ActiveModel {
    //                                 id: Set(*codigo),
    //                                 producto: Set(a.1.id),
    //                             };
    //                             if let Err(e) = cod_model.insert(&db).await {
    //                                 return Err(e.to_string());
    //                             }
    //                         }
    //                     }
    //                     Valuable::Pes(a)=>{
    //                         let model=pesable::ActiveModel{
    //                             id: Set(a.1.id),
    //                             codigo: Set(a.1.codigo),
    //                             precio_peso: Set(a.1.precio_peso),
    //                             porcentaje: Set(a.1.porcentaje),
    //                             costo_kilo: Set(a.1.costo_kilo),
    //                             descripcion: Set(a.1.descripcion.clone()),
    //                         };
    //                         if let Err(e) = model.insert(&db).await {
    //                             return Err(e.to_string());
    //                         }
    //                     }
    //                     Valuable::Rub(a)=>{
    //                         let model= rubro::ActiveModel{
    //                             id: Set(a.1.id),
    //                             monto: Set(a.1.monto),
    //                             descripcion: Set(a.1.descripcion.clone()),
    //                         };
    //                         if let Err(e) = model.insert(&db).await {
    //                             return Err(e.to_string());
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         Err(e)=>return Err(e.to_string())
    //     }

    //     Ok(())
    // }
    pub async fn get_data_valuable(&mut self) -> Result<(), String> {
        let mut prods: Vec<Valuable>;
        match Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
            Ok(db) => {
                match self.get_productos_from_db(&db).await {
                    Ok(a) => prods = a.iter().map(|x| Valuable::Prod((0, x.clone()))).collect(),

                    Err(e) => return Err(format!("Error gettings products: {}",e.to_string())),
                };
                match self.get_pesables_from_db(&db).await {
                    Ok(a) => prods
                        .append(&mut a.iter().map(|x| Valuable::Pes((0.0, x.clone()))).collect()),
                    Err(e) => return Err(format!("Error getting pesables: {}",e)),
                }
                match self.get_rubro_from_db(&db).await {
                    Ok(a) => {
                        prods.append(&mut a.iter().map(|x| Valuable::Rub((0, x.clone()))).collect())
                    }
                    Err(e) => return Err(format!("Error getting rubros {}",e)),
                }
                if self.productos.len() < prods.len() {
                    self.productos = prods;
                }
                println!("Cantidad de propductos: {}", self.productos.len());
            }
            Err(e) => return Err(e.to_string()),
        };

        Ok(())
    }
    pub async fn get_productos_from_db(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Producto>, String> {
        match entity::producto::Entity::find().all(db).await {
            Ok(a) => {
                let mut res=Vec::new();
                for i in 0..a.len(){
                    match entity::codigo_barras::Entity::find().filter(Condition::all()
                    .add(entity::codigo_barras::Column::Producto.eq(a[i].id))).all(db).await{
                        Ok(b)=>{
                            res.push(map_model_prod(&a[i], b.iter().map(|x|x.id).collect())?)
                        }
                        Err(e)=>return Err(e.to_string()),
                    }
                }
                Ok(res)
        },
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn get_pesables_from_db(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Pesable>, String> {
        match entity::pesable::Entity::find().all(db).await {
            Ok(a) => Ok(a.iter().map(|x| map_model_pes(x.clone())).collect()),
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn get_rubro_from_db(&self, db: &DatabaseConnection) -> Result<Vec<Rubro>, String> {
        match entity::rubro::Entity::find().all(db).await {
            Ok(a) => Ok(a.iter().map(|x| map_model_rub(x.clone())).collect()),
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn get_codigos_db_filtrado(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<Vec<i64>, String> {
        match entity::codigo_barras::Entity::find()
            .filter(Condition::all().add(entity::codigo_barras::Column::Producto.eq(id)))
            .all(db)
            .await
        {
            Ok(a) => Ok(a.iter().map(|x| x.id).collect()),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn map_model_prod(prod: &entity::producto::Model, cods: Vec<i64>) -> Result<Producto, String> {
    let presentacion = match serde_json::from_str(&prod.presentacion){
        Ok(a)=>a,
        Err(e)=>return Err(e.to_string()),
    };

    Ok(Producto {
        id: prod.id,
        codigos_de_barras: cods,
        precio_de_venta: prod.precio_de_venta,
        porcentaje: prod.porcentaje,
        precio_de_costo: prod.precio_de_costo,
        tipo_producto: prod.tipo_producto.clone(),
        marca: prod.marca.clone(),
        variedad: prod.variedad.clone(),
        presentacion: presentacion,
    })
}
fn map_model_rub(rub: entity::rubro::Model) -> Rubro {
    Rubro {
        id: rub.id,
        monto: rub.monto,
        descripcion: rub.descripcion,
    }
}

fn map_model_pes(pes: entity::pesable::Model) -> Pesable {
    Pesable {
        id: pes.id,
        codigo: pes.codigo,
        precio_peso: pes.precio_peso,
        porcentaje: pes.porcentaje,
        costo_kilo: pes.costo_kilo,
        descripcion: pes.descripcion,
    }
}
