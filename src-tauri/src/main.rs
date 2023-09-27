// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::panic;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::State;

//---------------------------------Structs y Enums-------------------------------------
pub struct Sistema<'a> {
    productos: Vec<Producto>,
    ventas: (Venta<'a>, Venta<'a>),
    proveedores: Vec<String>,
    path_prods: String,
    path_proveedores: String,
    relaciones: Vec<Relacion>,
}

pub struct Relacion {
    producto: Producto,
    proveedor: String,
    codigo: Option<u128>,
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
    codigo_de_barras: u128,
    precio_de_venta: f64,
    porcentaje: f64,
    precio_de_costo: f64,
    tipo_producto: String,
    marca: String,
    variedad: String,
    cantidad: Presentacion,
}

pub struct Proveedor {
    nombre: String,
    contacto: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Presentacion {
    Grs(f64),

    Un(i32),
    Lt(f64),
}

//-----------------------------------Implementations---------------------------------
impl Relacion {
    pub fn new(producto: Producto, proveedor: String, codigo: Option<u128>) -> Self {
        Relacion {
            producto,
            proveedor,
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
        let mut productos = Vec::new();
        leer_file(&mut productos, path_prods.clone());
        let mut proveedores = Vec::new();
        leer_file(&mut proveedores, path_proveedores.clone());
        let relaciones = Vec::new();
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
            relaciones,
        }
    }
    pub fn imprimir(&self) {
        println!("Printed from rust");
    }
    pub fn agregar(
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
        let mut relacion = Vec::new();
        let producto = Producto::new(
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
        if codigos_prov.len() > 0 {
            for i in 0..proveedores.len() - 1 {
                match codigos_prov[i].parse::<u128>() {
                    Ok(a) => relacion.push(Relacion::new(
                        producto.clone(),
                        proveedores[i].clone(),
                        Some(a),
                    )),
                    Err(_) => relacion.push(Relacion::new(
                        producto.clone(),
                        proveedores[i].clone(),
                        None,
                    )),
                };
            }
        }
        self.relaciones.extend(relacion);
        self.productos.push(producto);
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
    let mut prods = Vec::new();
    leer_file(&mut prods, path.clone());
    prods.push(pr);
    match crear_file(path.clone(), &prods) {
        Ok(_) => (),
        Err(e) => panic!("No se pudo pushear porque {}", e),
    };
}

fn leer_file<T: DeserializeOwned + Clone>(buf: &mut T, path: String) {
    let file2 = File::open(path.clone());
    let mut file2 = match file2 {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
    };
    let mut buf2 = String::new();
    if let Err(e) = file2.read_to_string(&mut buf2) {
        panic!("No se pudo leer porque {}", e);
    }
    match serde_json::from_str::<T>(&buf2.clone()) {
        Ok(a) => *buf = a.clone(),
        Err(e) => {
            panic!("No se pudo porque {}", e)
        }
    }
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
    match sistema.lock() {
        Ok(mut sis) => {
            sis.agregar(
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
