use self::lib::{crear_file, leer_file};
use crate::redondeo;
use serde::{Deserialize, Serialize};
mod lib;
use entity::{producto,proveedor};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tauri::async_runtime;
use std::{
    borrow::BorrowMut,
    fmt::{self, Display},
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
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Valuable {
    Prod((u16, Producto)),
    Pes((f32, Pesable)),
    Rub((u16, Rubro)),
}
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Gr(f32),
    Un(i16),
    Lt(f32),
    Ml(i16),
    Cc(i16),
    Kg(f32),
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pesable {
    pub id: i64,
    pub codigo: i64,
    pub precio_peso: f64,
    pub porcentaje: f64,
    pub costo_kilo: f64,
    pub descripcion: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rubro {
    pub id: i64,
    pub monto: f64,
    pub descripcion: String,
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

impl Default for Valuable {
    fn default() -> Self {
        Valuable::Prod((1, Producto::default()))
    }
}
impl PartialEq for Valuable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Valuable::Pes(a), Valuable::Pes(b)) => a.1.id == b.1.id,
            (Valuable::Prod(a), Valuable::Prod(b)) => a.1.id == b.1.id,
            (Valuable::Rub(a), Valuable::Rub(b)) => a.1.id == b.1.id,
            (_, _) => false,
        }
    }
}

pub trait ValuableTrait {
    fn redondear(&self, politica: f64) -> Self;
}

impl Valuable {
    pub fn get_price(&self, politica: f64) -> f64 {
        match self {
            Valuable::Pes(a) => redondeo(politica, a.0 as f64 * a.1.precio_peso),
            Valuable::Prod(a) => a.1.redondear(politica).precio_de_venta,
            Valuable::Rub(a) => a.1.redondear(politica).monto,
        }
    }
    pub fn get_descripcion(&self, conf: Config) -> String {
        let mut res = match self {
            Valuable::Pes(a) => a.1.descripcion.clone(),
            Valuable::Rub(a) => a.1.descripcion.clone(),
            Valuable::Prod(a) => match conf.formato_producto {
                Formato::Mtv => match a.1.presentacion {
                    Presentacion::Gr(cant) => format!(
                        "{} {} {} {} Gr",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Cc(cant) => format!(
                        "{} {} {} {} Cc",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Kg(cant) => format!(
                        "{} {} {} {} Kg",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Lt(cant) => format!(
                        "{} {} {} {} Lt",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Ml(cant) => format!(
                        "{} {} {} {} Ml",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                    Presentacion::Un(cant) => format!(
                        "{} {} {} {} Un",
                        a.1.marca, a.1.tipo_producto, a.1.variedad, cant
                    ),
                },
                Formato::Tmv => format!("{} {} {}", a.1.tipo_producto, a.1.marca, a.1.variedad),
            },
        };
        match conf.modo_mayus {
            Mayusculas::Lower => res = res.to_lowercase(),
            Mayusculas::Upper => res = res.to_uppercase(),
            Mayusculas::Camel => res = camalize(res),
        }
        res
    }
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

impl ValuableTrait for Valuable {
    fn redondear(&self, politica: f64) -> Valuable {
        match self {
            Valuable::Pes(a) => Valuable::Pes(a.clone()),
            Valuable::Prod(a) => Valuable::Prod((a.0, a.1.redondear(politica))),
            Valuable::Rub(a) => Valuable::Rub((a.0, a.1.redondear(politica))),
        }
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
impl ValuableTrait for Rubro {
    fn redondear(&self, politica: f64) -> Rubro {
        Rubro {
            id: self.id,
            monto: redondeo(politica, self.monto),
            descripcion: self.descripcion.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    id: i64,
    nombre: String,
    contacto: Option<i64>,
}

//-----------------------------------Implementations---------------------------------
impl Display for Presentacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gr(a) => write!(f, "{} Gr", a),
            Self::Lt(a) => write!(f, "{} Lt", a),
            Self::Un(a) => write!(f, "{} Un", a),
            Self::Ml(a) => write!(f, "{} Ml", a),
            Self::Cc(a) => write!(f, "{} CC", a),
            Self::Kg(a) => write!(f, "{} Kg", a),
        }
    }
}
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

        let mut proveedores:Vec<Proveedor> = Vec::new();
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
        Sistema {
            configs: configs[0].clone(),
            productos,
            ventas: (Venta::new(), Venta::new()),
            proveedores,
            path_productos,
            path_proveedores,
            path_relaciones,
            path_configs,
            relaciones,
            stash,
            registro,
        }
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
    pub fn get_venta_mut(&mut self, pos: usize) -> &mut Venta {
        if pos == 1 {
            self.ventas.1.borrow_mut()
        } else {
            self.ventas.0.borrow_mut()
        }
    }
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
                    && self.ventas.0.monto_pagado + monto > self.ventas.0.monto_total
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
                    && self.ventas.1.monto_pagado + monto > self.ventas.1.monto_total
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
            if i.nombre.eq_ignore_ascii_case(proveedor) {
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

        self.productos.push(Valuable::Prod((0, producto)));

        for i in 0..proveedores.len() {
            match codigos_prov[i].parse::<i64>() {
                Ok(a) => self.relaciones.push(RelacionProdProv::new(
                    self.productos.len() as i64 - 1,
                    i as i64,
                    Some(a),
                )),
                Err(_) => self.relaciones.push(RelacionProdProv::new(
                    self.productos.len() as i64 - 1,
                    i as i64,
                    None,
                )),
            };
        }
        let productos: Vec<Producto> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Prod(a) => Some(a.1.clone()),
                Valuable::Pes(_) => None,
                Valuable::Rub(_) => None,
            })
            .flatten()
            .collect();
        match crear_file(&self.path_productos, &productos) {
            Ok(_) => (),
            Err(e) => res = Err(e.to_string()),
        }
        match crear_file(&self.path_relaciones, &self.relaciones) {
            Ok(_) => (),
            Err(e) => res = Err(e.to_string()),
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
                prov=Proveedor::new(
                    self.proveedores.len() as i64,
                    proveedor.to_owned(),
                    contacto,
                );
            } else {
                prov=Proveedor::new(
                    self.proveedores.len() as i64,
                    proveedor.to_owned(),
                    None,
                );
            }
            if let Err(e)=async_runtime::block_on(prov.save()){
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
    pub fn agregar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<(), String> {
        let res = self
            .get_producto(id)?
            .redondear(self.configs.politica_redondeo);
        match pos {
            0 => {
                self.ventas
                    .0
                    .agregar_producto(res, self.configs.politica_redondeo);
            }
            1 => {
                self.ventas
                    .1
                    .agregar_producto(res, self.configs.politica_redondeo);
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }

        Ok(())
    }
    pub fn descontar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<(), String> {
        let res = self.get_producto(id)?;
        match pos {
            0 => {
                self.ventas
                    .0
                    .restar_producto(res, self.configs.politica_redondeo)?;
            }
            1 => {
                self.ventas
                    .1
                    .restar_producto(res, self.configs.politica_redondeo)?;
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }
        Ok(())
    }
    pub fn incrementar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<(), String> {
        let res = self.get_producto(id)?;
        match pos {
            0 => {
                self.ventas
                    .0
                    .incrementar_producto(res, self.configs.politica_redondeo)?;
            }
            1 => {
                self.ventas
                    .1
                    .incrementar_producto(res, self.configs.politica_redondeo)?;
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }
        Ok(())
    }
    pub fn eliminar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<(), String> {
        let res = self.get_producto(id)?;
        match pos {
            0 => {
                self.ventas
                    .0
                    .eliminar_producto(res, self.configs.politica_redondeo)?;
            }
            1 => {
                self.ventas
                    .1
                    .eliminar_producto(res, self.configs.politica_redondeo)?;
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }
        Ok(())
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
}

impl Default for Presentacion {
    fn default() -> Self {
        Presentacion::Un(i16::default())
    }
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
                if let Err(e) =model.insert(&db).await{
                    Err(e.to_string())
                }else{
                    Ok(())
                }
            }
            Err(e) => Err(e.to_string()),
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
                if let Err(e) =model.insert(&db).await{
                    Err(e.to_string())
                }else{
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
