// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::{panic, slice};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{self, format, Display};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::{Error, State};

//---------------------------------Structs y Enums-------------------------------------
pub struct Sistema<'a> {
    productos: Vec<Producto>,
    ventas: (Venta<'a>, Venta<'a>),
    proveedores: Vec<Proveedor>,
    path_prods: String,
    path_proveedores: String,
    path_relaciones: String,
    relaciones: Vec<Relacion>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Relacion {
    id_producto: usize,
    id_proveedor: usize,
    codigo: Option<u128>,
}

pub struct Venta<'a> {
    monto_total: f64,
    productos: Vec<&'a Producto>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Producto {
    pub id: usize,
    pub codigo_de_barras: u128,
    pub precio_de_venta: f64,
    pub porcentaje: f64,
    pub precio_de_costo: f64,
    pub tipo_producto: String,
    pub marca: String,
    pub variedad: String,
    pub cantidad: Presentacion,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Proveedor {
    nombre: String,
    contacto: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Grs(f64),
    Un(i32),
    Lt(f64),
}

//-----------------------------------Implementations---------------------------------
impl Display for Presentacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Grs(a) => write!(f, "{} Grs", a),
            Self::Lt(a) => write!(f, "{} Lt", a),
            Self::Un(a) => write!(f, "{} Un", a),
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

impl<'a> Venta<'a> {
    pub fn new() -> Venta<'a> {
        Venta {
            monto_total: 0.0,
            productos: Vec::new(),
        }
    }
}

impl<'a> Sistema<'a> {
    pub fn new() -> Sistema<'a> {
        let path_prods = String::from("Productos.json");
        let path_proveedores = String::from("Proveedores.json");
        let path_relaciones = String::from("Relaciones.json");
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

        Sistema {
            productos,
            ventas: (Venta::new(), Venta::new()),
            proveedores,
            path_prods,
            path_proveedores,
            path_relaciones,
            relaciones,
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
        codigo_de_barras: &str,
        precio_de_venta: &str,
        porcentaje: &str,
        precio_de_costo: &str,
        tipo_producto: &str,
        marca: &str,
        variedad: &str,
        cantidad: &str,
        presentacion: &str,
    ) {
        let producto = Producto::new(
            self.productos.len(),
            codigo_de_barras,
            precio_de_venta,
            porcentaje,
            precio_de_costo,
            tipo_producto,
            marca,
            variedad,
            cantidad,
            presentacion,
        );
        self.productos.push(producto);

        for i in 0..proveedores.len() {
            println!("{:?}", self.productos[self.productos.len() - 1]);
            println!("{:?}", proveedores[i]);
            println!("{:?}", codigos_prov[i]);
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
            Err(e) => panic!("No se pudo porque {}", e),
        }
        match crear_file(&self.path_relaciones, &self.relaciones) {
            Ok(_) => (),
            Err(e) => panic!("No se pudo porque {}", e),
        }
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
}

impl Default for Presentacion {
    fn default() -> Self {
        Presentacion::Un(i32::default())
    }
}

impl Producto {
    fn new(
        id: usize,
        codigo: &str,
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
            "Grs" => Presentacion::Grs(cantidad.parse().unwrap()),
            "Un" => Presentacion::Un(cantidad.parse().unwrap()),
            "Lt" => Presentacion::Lt(cantidad.parse().unwrap()),
            _ => panic!("no posible"),
        };
        Producto {
            id,
            codigo_de_barras: codigo.parse().unwrap(),
            precio_de_venta: precio_de_venta.parse().unwrap(),
            porcentaje: porcentaje.parse().unwrap(),
            precio_de_costo: precio_de_costo.parse().unwrap(),
            tipo_producto: tipo_producto.to_string(),
            marca: marca.to_string(),
            variedad: variedad.to_string(),
            cantidad: cant,
        }
    }
    fn get_nombre_completo(&self) -> String {
        format!(
            "{} {} {} {}",
            self.marca, self.tipo_producto, self.variedad, self.cantidad
        )
    }
    fn get_vec(&self)->Vec<String>{
        let mut res=Vec::new();
        res.push(self.tipo_producto.clone());
        res.push(self.marca.clone());
        res.push(self.variedad.clone());
        res.push(self.cantidad.to_string());
        res.push(self.precio_de_venta.to_string());
        res
    }
}

impl PartialEq for Producto {
    fn eq(&self, other: &Self) -> bool {
        if self.codigo_de_barras == other.codigo_de_barras {
            true
        } else {
            false
        }
    }
}

impl Proveedor {
    pub fn new(nombre: String, contacto: Option<u64>) -> Self {
        Proveedor { nombre, contacto }
    }
}

//--------------------Metodos y Main---------------------------------------------

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
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    codigo_de_barras: &str,
    precio_de_venta: &str,
    porcentaje: &str,
    precio_de_costo: &str,
    tipo_producto: &str,
    marca: &str,
    variedad: &str,
    cantidad: &str,
    presentacion: &str,
) -> String {
    match sistema.lock() {
        Ok(mut sis) => {
            sis.agregar_producto(
                proveedores,
                codigos_prov,
                codigo_de_barras,
                precio_de_venta,
                porcentaje,
                precio_de_costo,
                tipo_producto,
                marca,
                variedad,
                cantidad,
                presentacion,
            );

            format!("{:#?}", sis.productos[sis.productos.len() - 1].clone())
        }
        Err(a) => {
            format!("Error: {}", a)
        }
    }
}

#[tauri::command]
fn get_proveedores(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>, String> {
    match sistema.lock() {
        Ok(a) => {
            let mut res = Vec::new();
            for i in &a.proveedores {
                res.push(match serde_json::to_string_pretty(i) {
                    Ok(a) => a,
                    Err(e) => return Err(e.to_string()),
                })
            }
            Ok(res)
        }
        Err(e) => Err(e.to_string()),
    }
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
fn get_productos_filtrado2(sistema: State<Mutex<Sistema>>,
    filtro: String,
)->Result<Vec<Producto>,String>{
    let res;
    match sistema.lock(){
        Ok(a) => {
            res=Ok(a.productos.clone());
        }
        Err(e) => res=Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn get_productos_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: String,
) -> Result<Vec<String>, String> {
    let res;

    let filtros = filtro.split(' ').collect::<Vec<&str>>();
    match sistema.lock() {
        Ok(a) => {
            res = Ok(a.productos.iter().filter_map(|x| {
                let codigo= filtro.parse::<u128>();
                if codigo.is_ok()&&x.codigo_de_barras.eq(&codigo.unwrap())
                    || filtros.iter().any(|line| {
                        if x.get_nombre_completo().to_lowercase().contains(&line.to_lowercase()) {
                            true
                        } else {
                            false
                        }
                    })
                {
                    Some(serde_json::to_string_pretty(&x).unwrap())
                } else {
                    None
                }
            }).collect())
        }
        Err(e) => res=Err(e.to_string()),
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
            get_productos_filtrado,
            get_productos_filtrado2
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
