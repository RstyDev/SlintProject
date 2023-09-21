// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
use tauri::State;

//---------------------------------Structs y Enums-------------------------------------
pub struct Sistema<'a> {
    productos: Vec<Producto>,
    ventas:(Venta<'a>,Venta<'a>),
    proveedores: Vec<String>,
    path_prods:String,
    path_proveedores:String,
}

pub struct Venta<'a>{
    monto_total:f64,
    productos: Vec<&'a Producto>

}

impl<'a> Venta<'a>{
    pub fn new()->Venta<'a>{
        Venta { monto_total: 0.0, productos: Vec::new() }
    }
}



impl <'a>Sistema <'a>{
    pub fn new()->Sistema<'a>{
        let path_prods= String::from("Productos.json");
        let path_proveedores=String::from("Proveedores.json");
        let productos=leer_productos_file(path_prods.clone());
        let proveedores=leer_proveedores_file(path_proveedores.clone());
        Sistema { productos,ventas: (Venta::new(),Venta::new()), proveedores,
        path_prods,path_proveedores }
    }
    pub fn imprimir(&self){
        println!("Printed from rust");
    }
    pub fn new_producto2(
        &mut self,
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
        let prod=Producto::new(codigo_de_barras,precio_de_venta,porcentaje,precio_de_costo,
        tipo_producto,marca,variedad,cantidad,presentacion);
        self.productos.push(prod);
    }
    
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Producto{
    //falta agregar el codigo de proveedor en vec, los proveedores en vec,
    //puede ser un hashmap o un vec de tuplas con referencia a una lista de proveedores
    //algunas cosas mas tambien como familia de productos
    proveedores: Vec<String>,
    codigo_de_barras: usize,
    precio_de_venta: f64,
    porcentaje: f64,
    precio_de_costo: f64,
    tipo_producto: String,
    marca: String,
    variedad: String,
    cantidad: Presentacion,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Presentacion {
    Grs(f64),
    Un(i32),
    Lt(f64),
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
            proveedores:Vec::new(),
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

fn crear_file<'a>(path:String,escritura: &impl Serialize) {
    
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
    match file {
        Ok(mut a) => {
            if let Err(e) = writeln!(
                a,
                "{}",
                match serde_json::to_string_pretty(escritura) {
                    Ok(a) => a,
                    Err(e) => e.to_string(),
                }
            ) {
                println!("Error al escribir porque {}", e);
            }
        }
        Err(e) => println!("No se pudo escribir porque {}", e),
    }
}

pub fn push(pr: Producto,path:String) {
    let mut prods = leer_productos_file(path.clone());
    prods.push(pr);
    crear_file(path.clone(),&prods);
}
fn leer_proveedores_file<'a>(path:String)->Vec<String>{
    let file = OpenOptions::new().read(true).open(path.clone());
    let mut res: Vec<String> = Vec::new();
    match file {
        Ok(mut a) => {
            let mut buf = String::new();
            if let Err(e) = a.read_to_string(&mut buf) {
                panic!("No se pudo leer porque {}", e);
            }
            match serde_json::from_str(&buf.clone()) {
                Ok(a) => res = a,
                Err(_) => (),
            }

            res
        }
        Err(_) => match OpenOptions::new().write(true).create(true).open(path.clone()) {
            Ok(_) => res,
            Err(e) => panic!("No se pudo crear porque {}", e),
        },
    }
}
fn leer_productos_file<'a>(path:String) -> Vec<Producto> {
    let file = OpenOptions::new().read(true).open(path.clone());
    let mut res: Vec<Producto> = Vec::new();
    match file {
        Ok(mut a) => {
            let mut buf = String::new();
            if let Err(e) = a.read_to_string(&mut buf) {
                panic!("No se pudo leer porque {}", e);
            }
            match serde_json::from_str(&buf) {
                Ok(a) => res = a,
                Err(_) => (),
            }

            res
        }
        Err(_) => match OpenOptions::new().write(true).create(true).open(path.clone()) {
            Ok(_) => res,
            Err(e) => panic!("No se pudo crear porque {}", e),
        },
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn buscador(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn imprimir(sistema:State<Sistema>){
    sistema.imprimir();
}

#[tauri::command]
fn agregar(
    codigo: &str,
    precio_de_venta: &str,
    porcentaje: &str,
    precio_de_costo: &str,
    tipo_producto: &str,
    marca: &str,
    variedad: &str,
    cantidad: &str,
    presentacion: &str,sistema:State<Sistema>
) -> Result<String, String> {
    let res: Result<String, String>;
    let mut prods: Vec<Producto>;
    let prod=Producto::new(codigo,precio_de_venta,porcentaje,precio_de_costo,
        tipo_producto,marca,variedad,cantidad,presentacion);
    prods = leer_productos_file(sistema.path_prods.clone());
    if !prods.contains(&prod) {
        prods.push(prod);
        res = Ok(format!("{:#?}", prods));
    } else {
        res = Err("Este producto ya existe".to_string());
    }
    crear_file(sistema.path_prods.clone(),&prods);

    res
}

fn main() {
    tauri::Builder::default().manage(Sistema::new())
        .invoke_handler(tauri::generate_handler![buscador, agregar,imprimir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
