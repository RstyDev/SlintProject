// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mods::{
    config::Config, pesable::Pesable, rubro::Rubro, sistema::Sistema, valuable::Valuable,
    venta::Venta,
};
type Result<T> = std::result::Result<T, String>;

use std::sync::Mutex;
use tauri::{
    async_runtime::{self, block_on},
    State,
};

mod mods;

#[tauri::command]
fn buscador(name: &str) -> String {
    format!("Hello, {}! You've been mensajed from Rust!", name)
}

#[tauri::command]
fn agregar_proveedor(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    proveedor: &str,
    contacto: &str,
) -> Result<()> {
    match sistema.lock() {
        Ok(mut sis) => {
            if let Err(e) = sis.agregar_proveedor(proveedor, contacto) {
                return Err(e.to_string());
            }
            if let Err(e) = window.close() {
                return Err(e.to_string());
            }
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_producto(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    proveedores: Vec<&str>,
    codigos_prov: Vec<&str>,
    codigos_de_barras: Vec<&str>,
    precio_de_venta: &str,
    porcentaje: &str,
    precio_de_costo: &str,
    tipo_producto: &str,
    marca: &str,
    variedad: &str,
    cantidad: &str,
    presentacion: &str,
) -> Result<String> {
    match sistema.lock() {
        Ok(mut sis) => {
            let producto = match block_on(sis.agregar_producto(
                proveedores,
                codigos_prov,
                codigos_de_barras,
                precio_de_venta,
                porcentaje,
                precio_de_costo,
                tipo_producto,
                marca,
                variedad,
                cantidad,
                presentacion,
            )) {
                Ok(a) => a,
                Err(e) => return Err(e.to_string()),
            };
            if let Err(e) = window.close() {
                return Err(e.to_string());
            }
            Ok(producto.nombre_completo())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn agregar_pesable<'a>(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    id: i64,
    codigo: i64,
    precio_peso: f64,
    porcentaje: f64,
    costo_kilo: f64,
    descripcion: &'a str,
) -> Result<String> {
    match sistema.lock() {
        Ok(mut sis) => {
            let pesable =
                Pesable::new(id, codigo, precio_peso, porcentaje, costo_kilo, descripcion);

            if let Err(e) = sis.agregar_pesable(pesable.clone()) {
                return Err(e.to_string());
            }
            if let Err(e) = window.close() {
                return Err(e.to_string());
            }
            Ok(pesable.descripcion().to_string())
        }
        Err(a) => Err(a.to_string()),
    }
}
#[tauri::command]
fn agregar_rubro(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    id: i64,
    monto: f64,
    descripcion: &str,
) -> Result<String> {
    match sistema.lock() {
        Ok(mut sis) => {
            let rubro = Rubro::new(id, monto, descripcion);
            if let Err(e) = sis.agregar_rubro(rubro.clone()) {
                return Err(e.to_string());
            }
            if let Err(e) = window.close() {
                return Err(e.to_string());
            }
            Ok(rubro.descripcion().to_string())
        }
        Err(a) => Err(a.to_string()),
    }
}
#[tauri::command]
fn get_proveedores(sistema: State<'_, Mutex<Sistema>>) -> Result<Vec<String>> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            res = Ok(async_runtime::block_on(a.proveedores())
                .iter()
                .map(|x| x.to_string())
                .collect());
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

// #[tauri::command]
// fn get_productos(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>> {
//     let res: Result<Vec<String>>;
//     match sistema.lock() {
//         Ok(a) => {
//             res = Ok(async_runtime::block_on(a.productos())
//                 .iter()
//                 .map(|x| serde_json::to_string_pretty(&x).unwrap())
//                 .collect())
//         }
//         Err(e) => res = Err(e.to_string()),
//     }
//     res
// }

#[tauri::command]
fn get_productos_filtrado(sistema: State<Mutex<Sistema>>, filtro: &str) -> Result<Vec<Valuable>> {
    let res;
    match sistema.lock() {
        Ok(a) => match async_runtime::block_on(a.val_filtrado(filtro)) {
            Ok(a) => res = Ok(a),
            Err(e) => res = Err(e.to_string()),
        },
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn agregar_producto_a_venta(sistema: State<Mutex<Sistema>>, id: &str, pos: &str) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            res =
                match async_runtime::block_on(a.agregar_producto_a_venta(id.parse().unwrap(), pos))
                {
                    Ok(a) => Ok(a),
                    Err(e) => Err(e.to_string()),
                }
        }
        Err(e) => res = Err(e.to_string()),
    };
    res
}
#[tauri::command]
fn descontar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    id: &str,
    pos: &str,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            match a.descontar_producto_de_venta(id.parse().unwrap(), pos) {
                Ok(a) => {
                    println!("{:?}", a);
                    res = Ok(a)
                }
                Err(e) => res = Err(e.to_string()),
            }
        }
        Err(e) => res = Err(e.to_string()),
    };
    res
}
#[tauri::command]
fn incrementar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    id: &str,
    pos: &str,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            match a.incrementar_producto_a_venta(id.parse().unwrap(), pos) {
                Ok(a) => {
                    println!("{:?}", a);
                    res = Ok(a)
                }
                Err(e) => res = Err(e.to_string()),
            }
        }
        Err(e) => res = Err(e.to_string()),
    };
    res
}

#[tauri::command]
fn eliminar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    id: &str,
    pos: &str,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
            let pos = pos.parse().unwrap();
            res = match a.eliminar_producto_de_venta(id.parse().unwrap(), pos) {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => res = Err(e.to_string()),
    };
    res
}

#[tauri::command]
fn agregar_pago(
    sistema: State<Mutex<Sistema>>,
    medio_pago: &str,
    monto: f64,
    pos: usize,
) -> Result<f64> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
            res = match a.agregar_pago(medio_pago, monto, pos) {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn eliminar_pago(sistema: State<Mutex<Sistema>>, pos: usize, index: usize) -> Result<Venta> {
    match sistema.lock() {
        Ok(mut a) => match a.eliminar_pago(pos, index) {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: &str,
    tipo_filtro: &str,
) -> Result<Vec<String>> {
    let res;
    res = match sistema.lock() {
        Ok(a) => {
            if tipo_filtro.eq("marca") {
                match a.filtrar_marca(&filtro) {
                    Ok(a) => Ok(a),
                    Err(e) => Err(e.to_string()),
                }
            } else if tipo_filtro.eq("tipo_producto") {
                match a.filtrar_tipo_producto(&filtro) {
                    Ok(a) => Ok(a),
                    Err(e) => Err(e.to_string()),
                }
            } else {
                Err(format!("Parámetro incorrecto {tipo_filtro}"))
            }
        }
        Err(e) => Err(e.to_string()),
    };

    let res = match res {
        Ok(a) => Ok(a),
        Err(e) => Err(e.to_string()),
    };

    res
}

#[tauri::command]
fn get_venta_actual(sistema: State<Mutex<Sistema>>, pos: i32) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            if pos == 1 {
                res = Ok(a.venta(1).clone());
            } else {
                res = Ok(a.venta(0).clone());
            }
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}
#[tauri::command]
fn get_configs(sistema: State<Mutex<Sistema>>) -> Result<Config> {
    let res;
    match sistema.lock() {
        Ok(sis) => res = Ok(sis.configs().clone()),
        Err(e) => res = Err(e.to_string()),
    }

    res
}
#[tauri::command]
fn set_configs(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    configs: Config,
) -> Result<()> {
    let mut res = Ok(());
    match sistema.lock() {
        Ok(mut sis) => {
            sis.set_configs(configs);
            if let Err(e) = window.close() {
                return Err(e.to_string());
            }
        }
        Err(e) => res = Err(e.to_string()),
    }

    res
}

#[tauri::command]
fn close_window(window: tauri::Window) -> Result<()> {
    if let Err(e) = window.close() {
        return Err(e.to_string());
    }
    Ok(())
}

#[tauri::command]
fn get_medios_pago(sistema: State<Mutex<Sistema>>) -> Result<Vec<String>> {
    let res;
    match sistema.lock() {
        Ok(sis) => res = Ok(sis.configs().medios_pago().clone()),
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn get_descripcion_valuable(prod: Valuable, conf: Config) -> String {
    let res;
    res = prod.descripcion(&conf);
    res
}

#[tauri::command]
fn stash_sale(sistema: State<Mutex<Sistema>>, pos: usize) -> Result<()> {
    match sistema.lock() {
        Ok(mut sis) => match sis.stash_sale(pos) {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn unstash_sale(sistema: State<Mutex<Sistema>>, pos: usize, index: usize) -> Result<()> {
    match sistema.lock() {
        Ok(mut sis) => match sis.unstash_sale(pos, index) {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_stash(sistema: State<Mutex<Sistema>>) -> Result<Vec<Venta>> {
    match sistema.lock() {
        Ok(sis) => Ok(sis.stash().clone()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn open_add_product(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-product.html".parse().unwrap()),
    )
    .always_on_top(true)
    .resizable(false)
    .inner_size(800.0, 380.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn open_add_prov(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-prov.html".parse().unwrap()),
    )
    .always_on_top(true)
    .resizable(false)
    .inner_size(800.0, 110.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn open_edit_settings(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/edit-settings.html".parse().unwrap()),
    )
    .always_on_top(true)
    .resizable(false)
    .inner_size(750.0, 360.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
fn main() {
    let app = tauri::Builder::default()
        .manage(Mutex::new(Sistema::new().unwrap()))
        .invoke_handler(tauri::generate_handler![
            buscador,
            agregar_producto,
            agregar_pesable,
            agregar_rubro,
            agregar_proveedor,
            close_window,
            get_proveedores,
            get_filtrado,
            // get_productos,
            get_productos_filtrado,
            agregar_producto_a_venta,
            descontar_producto_de_venta,
            incrementar_producto_a_venta,
            eliminar_producto_de_venta,
            agregar_pago,
            eliminar_pago,
            get_venta_actual,
            get_configs,
            set_configs,
            get_medios_pago,
            get_descripcion_valuable,
            open_add_product,
            open_add_prov,
            open_edit_settings,
            stash_sale,
            unstash_sale,
            get_stash,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {})
}
