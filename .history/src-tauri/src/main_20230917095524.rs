// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
const PATH: &str = "Productos.json";

//---------------------------------Structs y Enums-------------------------------------

#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub struct Producto {
    codigo_de_barras:usize,
    precio:u64,
    tipo_producto: String,
    marca: String,
    variedad: String,
    cantidad: Presentacion,
}



#[derive(Debug, Serialize, Deserialize,PartialEq)]
pub enum Presentacion {
    Grs(f64),
    Un(i32),
    Lt(f64),
}

//--------------------Metodos y Main---------------------------------------------

fn crear_file<'a>(escritura: &impl Serialize) {
    // File::create(path).expect("Error");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(PATH);
    match file {
        Ok(mut a) => {
            if let Err(e) = writeln!(a, "{}", match serde_json::to_string_pretty(escritura){
                Ok(a)=>a,
                Err(e)=>e.to_string(),
            }) {
                println!("Error al escribir porque {}", e);
            }
        }
        Err(e) => println!("No se pudo escribir porque {}", e),
    }
}

pub fn push(pr: Producto) {
    let mut prods = leer_productos_file();
    prods.push(pr);
    crear_file(&prods);
}

fn leer_productos_file<'a>() -> Vec<Producto> {
    let file = OpenOptions::new().read(true).open(PATH);
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
        Err(_) => match OpenOptions::new().write(true).create(true).open(PATH) {
            Ok(_) => res,
            Err(e) => panic!("No se pudo crear porque {}", e),
        },
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn buscador(name: &str) -> String {

    format!(
        "Hello, {}! You've been greeted from Rust!",
        name
    )
}

#[tauri::command]
fn agregar(
    codigo_de_barras:&str,
    precio:&str,
    tipo_producto: &str,
    marca: &str,
    variedad: &str,
    cantidad: &str,
    presentacion: &str,
) -> Result<String,String> {
    let res:Result<String,String>;
    let mut prods: Vec<Producto>;
    let cant = match presentacion {
        "Grs" => Presentacion::Grs(cantidad.parse().unwrap()),
        "Un" => Presentacion::Un(cantidad.parse().unwrap()),
        "Lt" => Presentacion::Lt(cantidad.parse().unwrap()),
        _ => panic!("no posible"),
    };
    let prod = Producto {
        codigo_de_barras:
        tipo_producto: tipo_producto.to_string(),
        marca: marca.to_string(),
        variedad: variedad.to_string(),
        cantidad: cant,
    };
    prods = leer_productos_file();
    if !prods.contains(&prod) {
        prods.push(prod);
        res=Ok(format!("{:#?}", prods));
    }else{
        res=Err("Este producto ya existe".to_string());
    }
    crear_file(&prods);
    
    res
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![buscador, agregar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
