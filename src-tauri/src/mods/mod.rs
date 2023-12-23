use std::{fmt::{self, Display}, borrow::BorrowMut};

use serde::{Serialize, Deserialize};

use crate::redondeo;

use self::lib::{leer_file, crear_file};

mod lib;

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
pub struct Pago {
    medio_pago: String,
    monto: f64,
}
impl Pago {
    pub fn new(medio_pago: String, monto: f64) -> Pago {
        Pago { medio_pago, monto }
    }
}

pub struct Sistema {
    configs: Config,
    productos: Vec<Box<dyn Valuable>>,
    ventas: (Venta, Venta),
    proveedores: Vec<Proveedor>,
    path_prods: String,
    path_proveedores: String,
    path_relaciones: String,
    path_configs: String,
    relaciones: Vec<Relacion>,
    stash: Vec<Venta>,
    registro: Vec<Venta>
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Relacion {
    id_producto: usize,
    id_proveedor: usize,
    codigo: Option<u128>,
}
#[derive(Debug, Clone, Default, Serialize)]
pub struct Venta {
    monto_total: f64,
    productos: Vec<(u32, Producto)>,
    pagos: Vec<Pago>,
    monto_pagado:f64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Producto {
    pub id: usize,
    pub codigos_de_barras: Vec<u128>,
    pub precio_de_venta: f64,
    pub porcentaje: f64,
    pub precio_de_costo: f64,
    pub tipo_producto: String,
    pub marca: String,
    pub variedad: String,
    pub presentacion: Presentacion,
}

pub trait Valuable {
    
}
pub struct Pesable{
    descripcion:String,
}
impl Valuable for Producto{

}


impl Valuable for Pesable{

}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    nombre: String,
    contacto: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Gr(f64),
    Un(i32),
    Lt(f64),
    Ml(i32),
    Cc(i32),
    Kg(f64),
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
impl Config{
    pub fn get_cantidad_productos(&self)->usize{
        self.cantidad_productos
    }
    pub fn get_medios_pago(&self)->Vec<String>{
        self.medios_pago.clone()
    }
}
impl Default for Config{
    fn default() -> Self {
        Config { politica_redondeo: 10.0, formato_producto: Formato::default(), modo_mayus: Mayusculas::default(), cantidad_productos: 10, medios_pago: vec!["Efectivo".to_owned(), "Crédito".to_owned(), "Débito".to_owned()] }
    }
}
impl Relacion {
    pub fn new(id_producto: usize, id_proveedor: usize, codigo: Option<u128>) -> Self {
        Relacion {
            id_producto,
            id_proveedor,
            codigo,
        }
    }
}

impl<'a> Venta {
    pub fn new() -> Venta {
        Venta {
            monto_total: 0.0,
            productos: Vec::new(),
            pagos: Vec::new(),
            monto_pagado:0.0,
        }
    }
    pub fn agregar_pago(&mut self, medio_pago: String, monto: f64) {
        self.pagos.push(Pago::new(medio_pago, monto));
        self.monto_pagado+=monto;
    }
    fn agregar_producto(&mut self, producto: Producto) {
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i].1 {
                self.productos[i].0 += 1;
                esta = true;
            }
        }
        if !esta {
            self.productos.push((1, producto));
        }
        self.monto_total = 0.0;
        for i in &self.productos {
            self.monto_total += i.1.precio_de_venta * i.0 as f64;
        }
    }
    pub fn eliminar_pago(&mut self,index:usize){
        let pago= self.pagos.remove(index);
        self.monto_pagado-=pago.monto;
            
        
        
    }
    fn restar_producto(&mut self, producto: Producto) -> Result<(), String> {
        let mut res = Err("Producto no encontrado".to_string());
        for i in 0..self.productos.len() {
            if producto == self.productos[i].1 {
                self.productos[i].0 -= 1;
                res = Ok(());
            }
        }
        self.monto_total = 0.0;
        for i in &self.productos {
            self.monto_total += i.1.precio_de_venta * i.0 as f64;
        }
        res
    }
    fn eliminar_producto(&mut self, producto: Producto) -> Result<(), String> {
        let mut res = Err("Producto no encontrado".to_string());
        for i in 0..self.productos.len() {
            if producto == self.productos[i].1 {
                self.productos.remove(i);
                res = Ok(());
                break;
            }
        }
        self.monto_total = 0.0;
        for i in &self.productos {
            self.monto_total += i.1.precio_de_venta * i.0 as f64;
        }
        res
    }
}

impl<'a> Sistema {
    pub fn new() -> Sistema {
        let path_prods = String::from("Productos.json");
        let path_proveedores = String::from("Proveedores.json");
        let path_relaciones = String::from("Relaciones.json");
        let path_configs = String::from("Configs.json");
        let mut productos = Vec::new();
        let stash=Vec::new();
        let registro=Vec::new();
        if let Err(e) = leer_file(&mut productos, &path_prods) {
            panic!("{}", e);
        }
        let mut proveedores = Vec::new();
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
            path_prods,
            path_proveedores,
            path_relaciones,
            path_configs,
            relaciones,
            stash,
            registro,
        }
    }
    pub fn get_productos(&self)->&Vec<Producto>{
        &self.productos
    }
    pub fn get_productos_cloned(&self)->Vec<Producto>{
        self.productos.clone()
    }
    pub fn get_proveedores(&self)->&Vec<Proveedor>{
        &self.proveedores
    }
    pub fn get_configs(&self)->&Config{
        &self.configs
    }
    pub fn get_venta_mut(&mut self,pos: usize)->&mut Venta{
        if pos==1{
            self.ventas.1.borrow_mut()
        }else{ 
            self.ventas.0.borrow_mut()
        }
        
    }
    pub fn agregar_pago(&mut self, medio_pago:String,monto:f64,pos:usize){
        match pos{
            0=>{
                self.ventas.0.agregar_pago(medio_pago, monto);
            }
            1=>{
                self.ventas.1.agregar_pago(medio_pago, monto);
            }
            _=>panic!("error, hay solo dos posiciones para ventas")
        }
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

        self.productos.push(producto);

        for i in 0..proveedores.len() {
            match codigos_prov[i].parse::<u128>() {
                Ok(a) => self
                    .relaciones
                    .push(Relacion::new(self.productos.len() - 1, i, Some(a))),
                Err(_) => self
                    .relaciones
                    .push(Relacion::new(self.productos.len() - 1, i, None)),
            };
        }

        match crear_file(&self.path_prods, &self.productos) {
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
            if contacto.len() > 0 {
                let contacto: String = contacto
                    .chars()
                    .filter(|x| -> bool { x.is_numeric() })
                    .collect();
                let contacto = match contacto.parse() {
                    Ok(a) => Some(a),
                    Err(_) => return Err("Error al convertir el numero".to_owned()),
                };
                self.proveedores
                    .push(Proveedor::new(proveedor.to_owned(), contacto));
            } else {
                self.proveedores
                    .push(Proveedor::new(proveedor.to_owned(), None));
            }
            if let Err(e) = crear_file(&self.path_proveedores, &self.proveedores) {
                res = Err(e.to_string());
            }
        }
        res
    }
    fn get_producto(&mut self, id: usize) -> Result<Producto, String> {
        let mut res = Err("Producto no encontrado".to_string());
        for p in &self.productos {
            if p.id == id {
                res = Ok(p.clone());
            }
        }
        res
    }
    pub fn agregar_producto_a_venta(&mut self, id: usize, pos: usize) -> Result<(), String> {
        let res = self
            .get_producto(id)?
            .redondear(self.configs.politica_redondeo);
        match pos {
            0 => {
                self.ventas.0.agregar_producto(res);
            }
            1 => {
                self.ventas.1.agregar_producto(res);
                self.ventas.0.monto_total = 0.0;
                for i in 0..self.ventas.0.productos.len() {
                    self.ventas.0.monto_total += self.ventas.0.productos[i].1.precio_de_venta;
                }
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }

        Ok(())
    }
    pub fn descontar_producto_de_venta(&mut self, id: usize, pos: usize) -> Result<(), String> {
        let res = self.get_producto(id)?;
        match pos {
            0 => {
                self.ventas.0.restar_producto(res)?;
            }
            1 => {
                self.ventas.1.restar_producto(res)?;
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }
        Ok(())
    }
    pub fn eliminar_producto_de_venta(&mut self, id: usize, pos: usize) -> Result<(), String> {
        let res = self.get_producto(id)?;
        match pos {
            0 => {
                self.ventas.0.eliminar_producto(res)?;
            }
            1 => {
                self.ventas.1.eliminar_producto(res)?;
            }
            _ => return Err("Numero de venta incorrecto".to_string()),
        }
        Ok(())
    }
    pub fn get_venta(&self, pos: usize) -> Venta {
        let res;
        if pos==0{
            res = self.ventas.0.clone();
        }else{
            res = self.ventas.1.clone();
        }
        res
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Vec<String> {
        let iter = self.productos.iter();
        let mut res: Vec<String> = iter
            .filter_map(|x| {
                if x.marca.to_lowercase().contains(&filtro.to_lowercase()) {
                    Some(x.marca.clone())
                } else {
                    None
                }
            })
            .collect();
        res.sort();
        res.dedup();
        println!("de Rust:{:?}", res);

        res
    }

    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Vec<String> {
        let iter = self.productos.iter();
        let mut res: Vec<String> = iter
            .filter_map(|x| {
                if x.tipo_producto
                    .to_lowercase()
                    .contains(&filtro.to_lowercase())
                {
                    Some(x.tipo_producto.clone())
                } else {
                    None
                }
            })
            .collect();
        res.sort();
        res.dedup();
        println!("de Rust:{:?}", res);
        res
    }
    pub fn cerrar_venta(&mut self, pos:usize){
        match pos{
            0=> {
                self.registro.push(self.ventas.0.clone());
                self.ventas.0=Venta::new();
            },
            1=>{
                self.registro.push(self.ventas.1.clone());
                self.ventas.1=Venta::new();
            },
            _=>panic!("error, solo hay 2 posiciones para ventas"),
        };
    }
}

impl Default for Presentacion {
    fn default() -> Self {
        Presentacion::Un(i32::default())
    }
}

impl Producto {
    pub fn new(
        id: usize,
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
            .map(|code| -> u128 { code.parse().unwrap() })
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
    pub fn new(nombre: String, contacto: Option<u64>) -> Self {
        Proveedor { nombre, contacto }
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