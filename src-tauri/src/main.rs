// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mods::lib::get_hash;
use mods::{
    config::Config, pesable::Pesable, rubro::Rubro, sistema::Sistema, valuable::Valuable,
    venta::Venta,
};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
type Result<T> = std::result::Result<T, String>;
use std::sync::Mutex;
use tauri::{
    async_runtime::{self, block_on},
    State,
};
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: Option<String>,
    pos: Option<bool>,
}
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
    codigo: i64,
    descripcion: &str,
) -> Result<String> {
    match sistema.lock() {
        Ok(mut sis) => {
            let rubro = match async_runtime::block_on(Rubro::new_to_db(
                codigo,
                None,
                descripcion,
                sis.read_db(),
            )) {
                Ok(a) => a,
                Err(e) => return Err(e.to_string()),
            };
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
        Ok(a) => match async_runtime::block_on(a.val_filtrado(filtro, a.read_db())) {
            Ok(a) => res = Ok(a),
            Err(e) => res = Err(e.to_string()),
        },
        Err(e) => res = Err(e.to_string()),
    }
    res
}

#[tauri::command]
fn agregar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    prod: Valuable,
    pos: bool,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
            res = match async_runtime::block_on(a.agregar_producto_a_venta(prod, pos)) {
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
    pos: bool,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => match a.descontar_producto_de_venta(id.parse().unwrap(), pos) {
            Ok(a) => {
                println!("{:?}", a);
                res = Ok(a)
            }
            Err(e) => res = Err(e.to_string()),
        },
        Err(e) => res = Err(e.to_string()),
    };
    res
}
#[tauri::command]
fn incrementar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    id: &str,
    pos: bool,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => match a.incrementar_producto_a_venta(id.parse().unwrap(), pos) {
            Ok(a) => {
                println!("{:?}", a);
                res = Ok(a)
            }
            Err(e) => res = Err(e.to_string()),
        },
        Err(e) => res = Err(e.to_string()),
    };
    res
}

#[tauri::command]
fn eliminar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    id: &str,
    pos: bool,
) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(mut a) => {
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
async fn check_codes(code: i64) -> Result<bool> {
    match Database::connect("sqlite://db.sqlite?mode=ro").await {
        Ok(db) => {
            let mut disp = true;
            if disp {
                disp = match entity::codigo_barras::Entity::find()
                    .filter(entity::codigo_barras::Column::Codigo.eq(code))
                    .one(&db)
                    .await
                {
                    Ok(a) => {
                        if a.is_none() {
                            match entity::pesable::Entity::find()
                                .filter(entity::pesable::Column::Codigo.eq(code))
                                .one(&db)
                                .await
                            {
                                Ok(a) => {
                                    if a.is_none() {
                                        match entity::rubro::Entity::find()
                                            .filter(entity::rubro::Column::Codigo.eq(code))
                                            .one(&db)
                                            .await
                                        {
                                            Ok(a) => a.is_none(),
                                            Err(e) => return Err(e.to_string()),
                                        }
                                    } else {
                                        false
                                    }
                                }
                                Err(e) => return Err(e.to_string()),
                            }
                        } else {
                            false
                        }
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            Ok(disp)
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_pago(
    sistema: State<Mutex<Sistema>>,
    medio_pago: &str,
    monto: f64,
    pos: bool,
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
fn eliminar_pago(sistema: State<Mutex<Sistema>>, pos: bool, index: usize) -> Result<Venta> {
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
fn get_venta_actual(sistema: State<Mutex<Sistema>>, pos: bool) -> Result<Venta> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            res = Ok(a.venta(pos).clone());
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
    let db = match sistema.lock() {
        Ok(a) => a.write_db().clone(),
        Err(e) => return Err(e.to_string()),
    };
    match sistema.lock() {
        Ok(mut sis) => {
            sis.set_configs(configs, &db);
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
fn stash_n_close(window: tauri::Window, sistema: State<Mutex<Sistema>>, pos: bool) -> Result<()> {
    match sistema.lock() {
        Ok(mut sis) => {
            if let Err(e) = sis.stash_sale(pos) {
                return Err(e.to_string());
            }
            if let Err(e) = window.emit(
                "main",
                Payload {
                    message: Some("dibujar venta".into()),
                    pos: None,
                },
            ) {
                return Err(e.to_string());
            }
            if let Err(e) = window.close() {
                return Err(e.to_string());
            }
            println!("{:#?}", sis.stash());
            Ok(())
        }

        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn unstash_sale(sistema: State<Mutex<Sistema>>, pos: bool, index: usize) -> Result<()> {
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

async fn open_add_product(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-product.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(800.0, 400.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
async fn open_add_pesable(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-pesable", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-pesable.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(350.0, 260.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
async fn open_add_rubro(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-rubro", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-rubro.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(350.0, 180.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn open_add_select(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-select",
        tauri::WindowUrl::App("/pages/add-select.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(210.0, 80.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn select_window(handle: tauri::AppHandle, window: tauri::Window, dato: &str) -> Result<()> {
    let mut res;
    match dato {
        "Producto" => {
            res = open_add_product(handle).await;
        }
        "Pesable" => {
            res = open_add_pesable(handle).await;
        }
        "Rubro" => {
            res = open_add_rubro(handle).await;
        }
        _ => return Err("Solo existen Producto, Pesable y Rubro".to_string()),
    }
    if res.is_ok() {
        res = match window.close() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    res
}
#[tauri::command]
async fn open_add_prov(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/add-prov.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(430.0, 110.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn open_confirm_stash(handle: tauri::AppHandle, act: bool) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "confirm-stash", /* the unique window label */
        tauri::WindowUrl::App("/pages/want-to-stash.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(400.0, 150.0)
    .build()
    {
        Ok(a) => {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Err(e) = a.emit(
                "get-venta",
                Payload {
                    message: None,
                    pos: Some(act),
                },
            ) {
                return Err(e.to_string());
            }
            for _ in 0..5 {
                std::thread::sleep(std::time::Duration::from_millis(175));
                if let Err(e) = a.emit(
                    "get-venta",
                    Payload {
                        message: None,
                        pos: Some(act),
                    },
                ) {
                    return Err(e.to_string());
                }
            }
            Ok(())
        }

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
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(500.0, 360.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn open_stash(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "add-product", /* the unique window label */
        tauri::WindowUrl::App("/pages/stash.html".parse().unwrap()),
    )
    .always_on_top(true)
    .center()
    .resizable(false)
    .minimizable(false)
    .inner_size(900.0, 600.0)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn open_login(handle: tauri::AppHandle) -> Result<()> {
    match tauri::WindowBuilder::new(
        &handle,
        "login", /* the unique window label */
        tauri::WindowUrl::App("/pages/login.html".parse().unwrap()),
    )
    .inner_size(600.0, 400.0)
    .resizable(false)
    .closable(false)
    .always_on_top(true)
    .center()
    // .minimizable(false)
    .build()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn try_login(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    id: &str,
    pass: &str,
) -> Result<()> {
    match sistema.lock() {
        Ok(mut sis) => match async_runtime::block_on(sis.try_login(id, get_hash(pass))) {
            Ok(_) => {
                println!("{:#?}", sis.user());
                if let Err(e) = window.close() {
                    return Err(e.to_string());
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
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
            open_add_select,
            select_window,
            open_add_prov,
            open_confirm_stash,
            open_edit_settings,
            stash_n_close,
            unstash_sale,
            get_stash,
            open_stash,
            check_codes,
            open_login,
            try_login,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {})
}
