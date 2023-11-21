// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::panic;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Mutex, MutexGuard, PoisonError};
use tauri::State;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    politica_redondeo: f64,
    formato_producto: Formato,
    modo_luz: Luz,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Formato {
    #[default]
    Tmv,
    Mtv,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Luz {
    #[default]
    Claro,
    Oscuro,
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

//---------------------------------Structs y Enums-------------------------------------
pub struct Sistema {
    configs: Config,
    productos: Vec<Producto>,
    ventas: (Venta, Venta),
    proveedores: Vec<Proveedor>,
    path_prods: String,
    path_proveedores: String,
    path_relaciones: String,
    path_configs: String,
    relaciones: Vec<Relacion>,
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
        }
    }
    fn agregar_pago(&mut self, medio_pago: String, monto: f64) -> f64 {
        self.pagos.push(Pago::new(medio_pago, monto));
        let mut pagado = 0.0;
        for i in &self.pagos {
            pagado += i.monto;
        }
        self.monto_total - pagado
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
}

impl<'a> Sistema {
    pub fn new() -> Sistema {
        let path_prods = String::from("Productos.json");
        let path_proveedores = String::from("Proveedores.json");
        let path_relaciones = String::from("Relaciones.json");
        let path_configs = String::from("Configs.json");
        let mut productos = Vec::new();
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
        }
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
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
    fn agregar_producto_a_venta(&mut self, id: usize, pos: usize) -> Result<(), String> {
        let res = self.get_producto(id)?;
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
    fn descontar_producto_de_venta(&mut self, id: usize, pos: usize) -> Result<(), String> {
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
    fn get_venta(&self, pos: usize) -> Result<Venta, String> {
        let res;
        match pos {
            0 => res = Ok(self.ventas.0.clone()),
            1 => res = Ok(self.ventas.1.clone()),
            _ => res = Err("Numero de venta erroneo".to_string()),
        }
        res
    }
    fn filtrar_marca(&self, filtro: &str) -> Vec<String> {
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

    fn filtrar_tipo_producto(&self, filtro: &str) -> Vec<String> {
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
}

impl Default for Presentacion {
    fn default() -> Self {
        Presentacion::Un(i32::default())
    }
}

impl Producto {
    fn new(
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
            _ => panic!("no posible"),
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
    fn get_nombre_completo(&self) -> String {
        format!(
            "{} {} {} {}",
            self.marca, self.tipo_producto, self.variedad, self.presentacion
        )
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

//--------------------Funciones y Main---------------------------------------------

fn crear_file<'a>(path: &String, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", format!("{}", buf))?;
    Ok(())
}

pub fn push(pr: Producto, path: &String) {
    let mut prods = Vec::new();
    if let Err(e) = leer_file(&mut prods, path) {
        panic!("{}", e);
    }
    prods.push(pr);
    match crear_file(&path, &prods) {
        Ok(_) => (),
        Err(e) => panic!("No se pudo pushear porque {}", e),
    };
}

fn leer_file<T: DeserializeOwned + Clone + Serialize>(
    buf: &mut T,
    path: &String,
) -> std::io::Result<()> {
    let file2 = File::open(path.clone());
    let mut file2 = match file2 {
        Ok(file) => file,
        Err(_) => {
            let esc: Vec<String> = Vec::new();
            crear_file(path, &esc)?;
            File::open(path.clone())?
        }
    };

    let mut buf2 = String::new();
    file2.read_to_string(&mut buf2)?;
    match serde_json::from_str::<T>(&buf2.clone()) {
        Ok(a) => *buf = a.clone(),
        Err(e) => {
            panic!("No se pudo porque {}", e)
        }
    }
    Ok(())
}

// -------------------------------Commands----------------------------------------------

#[tauri::command]
fn buscador(name: &str) -> String {
    format!("Hello, {}! You've been mensaje1ed from Rust!", name)
}

#[tauri::command]
fn imprimir(sistema: State<Mutex<Sistema>>) {
    let sis = sistema.lock().unwrap();
    sis.imprimir();
}
#[tauri::command]
fn agregar_proveedor(
    sistema: State<Mutex<Sistema>>,
    proveedor: &str,
    contacto: &str,
) -> Result<(), String> {
    match sistema.lock() {
        Ok(mut sis) => {
            sis.agregar_proveedor(proveedor, contacto)?;
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_producto(
    sistema: State<Mutex<Sistema>>,
    proveedores: Vec<String>,
    codigos_prov: Vec<String>,
    codigos_de_barras: Vec<&str>,
    precio_de_venta: &str,
    porcentaje: &str,
    precio_de_costo: &str,
    tipo_producto: &str,
    marca: &str,
    variedad: &str,
    cantidad: &str,
    presentacion: &str,
) -> Result<(), String> {
    match sistema.lock() {
        Ok(mut sis) => {
            for code in &codigos_de_barras {
                if let Err(e) = code.parse::<u128>() {
                    return Err(e.to_string());
                }
            }
            let producto = Producto::new(
                sis.productos.len(),
                codigos_de_barras,
                precio_de_venta,
                porcentaje,
                precio_de_costo,
                tipo_producto,
                marca,
                variedad,
                cantidad,
                presentacion,
            );
            sis.agregar_producto(proveedores, codigos_prov, producto)?;

            Ok(())
        }
        Err(a) => Err(a.to_string()),
    }
}

#[tauri::command]
fn get_proveedores(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>, String> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            res = Ok(a.proveedores.iter().map(|x| x.to_string()).collect());
            // let mut res = Vec::new();
            // for i in &a.proveedores {
            //     res.push(match serde_json::to_string_pretty(i) {
            //         Ok(a) => a,
            //         Err(e) => return Err(e.to_string()),
            //     })
            // }
            // Ok(res)
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn get_productos(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>, String> {
    let res: Result<Vec<String>, String>;
    match sistema.lock() {
        Ok(a) => {
            res = Ok(a
                .productos
                .iter()
                .map(|x| serde_json::to_string_pretty(&x).unwrap())
                .collect())
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn get_productos_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: String,
) -> Result<Vec<Producto>, String> {
    let filtros = filtro.split(' ').collect::<Vec<&str>>();
    let res;
    match sistema.lock() {
        Ok(a) => {
            let b = a.productos.clone();
            res = Ok(b
                .into_iter()
                .filter(|x| {
                    let codigo = filtro.parse::<u128>();
                    if (codigo.is_ok() && x.codigos_de_barras.contains(&codigo.unwrap()))
                        || filtros.iter().any(|line| {
                            if x.get_nombre_completo()
                                .to_lowercase()
                                .contains(&line.to_lowercase())
                            {
                                true
                            } else {
                                false
                            }
                        })
                    {
                        true
                    } else {
                        false
                    }
                })
                .to_owned()
                .collect());
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}
#[tauri::command]
fn agregar_producto_a_venta(sistema: State<Mutex<Sistema>>, id: String, pos: String) {
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            match a.agregar_producto_a_venta(id.parse().unwrap(), pos) {
                Ok(_) => println!("{:?}", a.get_venta(pos)),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    };
}
#[tauri::command]
fn descontar_producto_de_venta(sistema: State<Mutex<Sistema>>, id: String, pos: String) {
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            match a.descontar_producto_de_venta(id.parse().unwrap(), pos) {
                Ok(_) => println!("{:?}", a.get_venta(pos)),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    };
}

#[tauri::command]
fn agregar_pago(
    sistema: State<Mutex<Sistema>>,
    medio_pago: String,
    monto: f64,
    pos: String,
) -> Result<f64, String> {
    let res;
    let pos: usize = pos.parse().unwrap();
    match sistema.lock() {
        Ok(mut a) => match pos {
            0 => res = Ok(a.ventas.0.agregar_pago(medio_pago, monto)),
            1 => res = Ok(a.ventas.1.agregar_pago(medio_pago, monto)),
            _ => res = Err("numero de venta incorrecto".to_string()),
        },
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn get_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: String,
    tipo_filtro: String,
) -> Result<Vec<String>, String> {
    let mut res = Err("No inicializado".to_string());
    match sistema.lock() {
        Ok(a) => {
            if tipo_filtro.eq("marca") {
                res = Ok(a.filtrar_marca(&filtro));
            } else if tipo_filtro.eq("tipo_producto") {
                res = Ok(a.filtrar_tipo_producto(&filtro));
            }
        }
        Err(e) => res = Err(e.to_string()),
    }

    res
}
#[tauri::command]
fn redondeo(politica: f64, numero: f64) -> f64 {
    let mut res = numero;
    let dif = numero % politica;
    if dif != 0.0 {
        if dif < politica / 2.0 {
            res = numero - dif;
        } else {
            res = numero + politica - dif;
        }
    }
    res
}
#[tauri::command]
fn get_venta_actual(sistema: State<Mutex<Sistema>>, pos: i32) -> Result<Venta, String> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            if pos == 1 {
                res = Ok(a.ventas.1.clone());
            } else {
                res = Ok(a.ventas.0.clone());
            }
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}
#[tauri::command]
fn get_configs(sistema: State<Mutex<Sistema>>) -> Result<Config, String> {
    let res;
    match sistema.lock() {
        Ok(sis) => res = Ok(sis.configs.clone()),
        Err(e) => res = Err(e.to_string()),
    }
    res
}
#[tauri::command]
fn set_configs(sistema: State<Mutex<Sistema>>, configs: Config) -> Result<(), String> {
    let mut res = Ok(());
    match sistema.lock() {
        Ok(mut sis) => sis.set_configs(configs),
        Err(e) => res = Err(e.to_string()),
    }
    res
}

//----------------------------------------main--------------------------------------------

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Sistema::new()))
        .invoke_handler(tauri::generate_handler![
            buscador,
            agregar_producto,
            agregar_proveedor,
            imprimir,
            get_proveedores,
            get_productos,
            get_filtrado,
            get_productos_filtrado,
            agregar_producto_a_venta,
            descontar_producto_de_venta,
            redondeo,
            agregar_pago,
            get_venta_actual,
            get_configs,
            set_configs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
