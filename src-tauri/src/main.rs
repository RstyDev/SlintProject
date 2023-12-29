// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::panic;
use mods::Config;
use mods::Formato;
use mods::Presentacion;
use mods::Producto;
use mods::Sistema;
use mods::Valuable;
use mods::Venta;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::sync::Mutex;
use std::thread;
use tauri::async_runtime;
use tauri::State;
use tokio;
mod mods;

use entity::producto;

#[tauri::command]
fn buscador(name: &str) -> String {
    format!("Hello, {}! You've been mensajed from Rust!", name)
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
    window:tauri::Window,
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
) -> Result<String, String> {
    match sistema.lock() {
        Ok(mut sis) => {
            for code in &codigos_de_barras {
                if let Err(e) = code.parse::<u128>() {
                    return Err(e.to_string());
                }
            }
            let producto = Producto::new(
                sis.get_productos().len() as i64,
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
            // let prods:Vec<Producto> = sis.get_productos().iter().map(|x|{
            //     match x{
            //         Valuable::Prod(a)=>Some(a.1.clone()),
            //         _=>None,
            //     }
            // }).flatten().collect();

            
            // for i in 0..prods.len() {
            //     let save = async_runtime::spawn(save_producto(prods[i].clone()));
            //     let _ = async_runtime::block_on(save);
            // }
            if let Err(e)= async_runtime::block_on(producto.clone().save()){
                return Err(e.to_string())
            }
            // let save = async_runtime::spawn(save_producto(producto.clone()));
            // let _ = async_runtime::block_on(save);
            sis.agregar_producto(proveedores, codigos_prov, producto.clone())?;
            if let Err(e)=window.close(){
                return Err(e.to_string())
            }
            Ok(producto.get_nombre_completo())
        }
        Err(a) => Err(a.to_string()),
    }
}

#[tauri::command]
fn get_proveedores(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>, String> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            res = Ok(a.get_proveedores().iter().map(|x| x.to_string()).collect());
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
                .get_productos()
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
) -> Result<Vec<Valuable>, String> {
    let filtros = filtro.split(' ').collect::<Vec<&str>>();
    let res;
    match sistema.lock() {
        Ok(a) => {
            let b = a.get_productos_cloned();
            res = Ok(b
                .into_iter()
                .filter(|x| {
                    let codigo = filtro.parse::<i64>();
                    match x {
                        Valuable::Prod(a) => {
                            if (codigo.is_ok() && a.1.codigos_de_barras.contains(&codigo.unwrap()))
                                || filtros.iter().any(|line| {
                                    if a.1
                                        .get_nombre_completo()
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
                        }
                        _ => false,
                    }
                })
                .take(a.get_configs().get_cantidad_productos())
                .to_owned()
                .collect());
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}

async fn connect() {
    if let Ok(db) = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
        println!("conectado");
    } else {
        println!("no conectado");
    }
}
async fn save_producto(producto: Producto) {
    let model = producto::ActiveModel {
        id: Set(producto.id),
        precio_de_venta: Set(producto.precio_de_venta),
        porcentaje: Set(producto.porcentaje),
        precio_de_costo: Set(producto.precio_de_costo),
        tipo_producto: Set(producto.tipo_producto.clone()),
        marca: Set(producto.marca.clone()),
        variedad: Set(producto.variedad.clone()),
        presentacion: Set(producto.presentacion.to_string()),
    };
    if let Ok(db) = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await {
        println!("conectado");
        let prod = model.insert(&db).await.unwrap();
    } else {
        println!("no conectado");
    }
}
#[tauri::command]
fn agregar_producto_a_venta(sistema: State<Mutex<Sistema>>, id: String, pos: String) {
    // let algo = async_runtime::spawn(connect());
    // let _ = async_runtime::block_on(algo);

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
fn incrementar_producto_a_venta(sistema: State<Mutex<Sistema>>, id: String, pos: String) {
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            match a.incrementar_producto_a_venta(id.parse().unwrap(), pos) {
                Ok(_) => println!("{:?}", a.get_venta(pos)),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    };
}

#[tauri::command]
fn eliminar_producto_de_venta(sistema: State<Mutex<Sistema>>, id: String, pos: String) {
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            match a.eliminar_producto_de_venta(id.parse().unwrap(), pos) {
                Ok(_) => println!("{:?}", a.get_venta(pos)),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    };
    println!("pago eliminado");
}

#[tauri::command]
fn agregar_pago(
    sistema: State<Mutex<Sistema>>,
    medio_pago: String,
    monto: f64,
    pos: usize,
) -> Result<f64, String> {
    let res;
    match sistema.lock() {
        Ok(mut a) => res = a.agregar_pago(medio_pago, monto, pos),
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn eliminar_pago(sistema: State<Mutex<Sistema>>, pos: usize, index: usize) -> Result<(), String> {
    match sistema.lock() {
        Ok(mut a) => match pos {
            0 => a.get_venta_mut(0).eliminar_pago(index),
            1 => a.get_venta_mut(1).eliminar_pago(index),
            _ => return Err("numero de venta incorrecto".to_string()),
        },
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
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
                res = Ok(a.get_venta(1).clone());
            } else {
                res = Ok(a.get_venta(0).clone());
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
        Ok(sis) => res = Ok(sis.get_configs().clone()),
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

#[tauri::command]
fn get_medios_pago(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>, String> {
    let res;
    match sistema.lock() {
        Ok(sis) => res = Ok(sis.get_configs().get_medios_pago()),
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn get_descripcion_valuable(prod: Valuable, conf: Config) -> String {
    let res: String;
    res = prod.get_descripcion(conf);
    res
}

#[tauri::command]
async fn open_add_product(handle: tauri::AppHandle) {
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-product.html".parse().unwrap()),
    ).always_on_top(true).resizable(false).inner_size(800.0, 380.0)
    .build();
}


fn main() {
    let app = tauri::Builder::default()
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
            incrementar_producto_a_venta,
            eliminar_producto_de_venta,
            redondeo,
            agregar_pago,
            eliminar_pago,
            get_venta_actual,
            get_configs,
            set_configs,
            get_medios_pago,
            get_descripcion_valuable,
            open_add_product,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    

    app.run(|_, _| {})
}
