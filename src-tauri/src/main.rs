// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::panic;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Read, Write},
};
use tauri::State;

//---------------------------------Structs y Enums-------------------------------------
pub struct Sistema<'a> {
    productos: Vec<Producto>,
    ventas: (Venta<'a>, Venta<'a>),
    proveedores: Vec<String>,
    path_prods: String,
    path_proveedores: String,
}

pub struct Venta<'a> {
    monto_total: f64,
    productos: Vec<&'a Producto>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Producto {
    //falta agregar el codigo de proveedor en vec, los proveedores en vec,
    //puede ser un hashmap o un vec de tuplas con referencia a una lista de proveedores
    //algunas cosas mas tambien como familia de productos
    proveedores_codigos: HashMap<String, Option<u128>>,
    codigo_de_barras: u128,
    precio_de_venta: f64,
    porcentaje: f64,
    precio_de_costo: f64,
    tipo_producto: String,
    marca: String,
    variedad: String,
    cantidad: Presentacion,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Grs(f64),

    Un(i32),
    Lt(f64),
}

//-----------------------------------Implementations---------------------------------

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
        let productos = leer_productos_file(path_prods.clone());
        let proveedores = leer_proveedores_file(path_proveedores.clone());
        println!(
            "A partir de aca estamos escribiendo los productos {:?}",
            productos
        );
        Sistema {
            productos,
            ventas: (Venta::new(), Venta::new()),
            proveedores,
            path_prods,
            path_proveedores,
        }
    }
    pub fn imprimir(&self) {
        println!("Printed from rust");
    }
    pub fn agregar(
        &mut self,
        proveedores_codigos: HashMap<String, Option<u128>>,
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
        let prod = Producto::new(
            proveedores_codigos,
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
        self.productos.push(prod);
        match crear_file(self.path_prods.clone(), &self.productos) {
            Ok(_) => (),
            Err(e) => panic!("No se pudo porque {}", e),
        }
    }
}

impl Default for Presentacion {
    fn default() -> Self {
        Presentacion::Un(i32::default())
    }
}

impl Producto {
    fn new(
        proveedores_codigos: HashMap<String, Option<u128>>,
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
            proveedores_codigos,
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

//--------------------Metodos y Main---------------------------------------------

fn crear_file<'a>(path: String, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", format!("{}", buf))?;
    Ok(())
}

pub fn push(pr: Producto, path: String) {
    let mut prods = leer_productos_file(path.clone());
    prods.push(pr);
    match crear_file(path.clone(), &prods) {
        Ok(_) => (),
        Err(e) => panic!("No se pudo pushear porque {}", e),
    };
}
fn leer_proveedores_file(path: String) -> Vec<String> {
    let file2 = File::open(path.clone());
    let res: Vec<String>;
    let mut file2 = match file2 {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
    };
    let mut buf = String::new();
    if let Err(e) = file2.read_to_string(&mut buf) {
        panic!("No se pudo leer porque {}", e);
    }
    match serde_json::from_str(&buf.clone()) {
        Ok(a) => res = a,
        Err(e) => {
            panic!("No se pudo porque {}", e)
        }
    }
    res
}

fn leer_productos_file<'a>(path: String) -> Vec<Producto> {
    let file2 = File::open(path.clone());
    let mut res: Vec<Producto>;
    let mut file2 = match file2 {
        Ok(file) => file,
        Err(_) => {
            res = Vec::new();
            match crear_file(path.clone(), &res){
                Ok(_)=>File::open(path.clone()).unwrap(),
                Err(e)=>panic!("No se pudo porque {}",e),
            }
        }
    };
    let mut buf = String::new();
    if let Err(e) = file2.read_to_string(&mut buf) {
        panic!("No se pudo leer porque {}", e);
    }
    match serde_json::from_str(&buf.clone()) {
        Ok(a) => res = a,
        Err(_) => {
            let vec: Vec<Producto> = Vec::new();
            if let Err(e) = crear_file(path, &vec) {
                panic!("Otro error {}", e);
            }
            res = vec;
        }
    }
    res
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
fn agregar(
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
    let mut res = HashMap::new();
    match sistema.lock() {
        Ok(mut sis) => {
            if codigos_prov.len() > 0 {
                for i in 0..proveedores.len() - 1 {
                    match codigos_prov[i].parse::<u128>() {
                        Ok(a) => res.insert(proveedores[i].clone(), Some(a)),
                        Err(_) => res.insert(proveedores[i].clone(), None),
                    };
                }
            }
            sis.agregar(
                res,
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
fn get_proveedores(sistema: State<Mutex<Sistema>>) -> Vec<String> {
    sistema.lock().unwrap().proveedores.clone()
}

//----------------------------------------main--------------------------------------------

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Sistema::new()))
        .invoke_handler(tauri::generate_handler![
            buscador,
            agregar,
            imprimir,
            get_proveedores
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
