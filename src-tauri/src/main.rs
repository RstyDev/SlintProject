// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mods::{
    caja::Caja, cliente::Cli, config::Config, lib::get_hash, pesable::Pesable, rubro::Rubro,
    sistema::Sistema, user::Rango, user::User, valuable::Valuable, venta::Venta,
};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use serde::Serialize;
type Res<T> = std::result::Result<T, String>;
use std::sync::Mutex;
use tauri::{
    async_runtime::{self, block_on},
    window::MenuHandle,
    CustomMenuItem, Manager, Menu, Result, State, Submenu,
};
const DENEGADO: &str = "Permiso denegado";
#[derive(Clone, Serialize)]
struct Payload {
    message: Option<String>,
    pos: Option<bool>,
}
mod mods;
fn try_disable_windows(menu: MenuHandle) -> Result<()> {
    menu.get_item("add product").set_enabled(false)?;
    menu.get_item("add prov").set_enabled(false)?;
    menu.get_item("add user").set_enabled(false)?;
    menu.get_item("add cliente").set_enabled(false)?;
    menu.get_item("edit settings").set_enabled(false)?;
    Ok(())
}
async fn open_add_product(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-product") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
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
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

async fn open_add_pesable(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-pesable") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
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
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
async fn open_add_rubro(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-rubro") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
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
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
fn agregar_cliente(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    nombre: &str,
    dni: i64,
    credito: bool,
) -> Res<Cli> {
    match sistema.lock() {
        Ok(sis) => match sis.agregar_cliente(nombre, dni, credito, true) {
            Ok(a) => {
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
                Ok(a)
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_pago(
    sistema: State<Mutex<Sistema>>,
    medio_pago: &str,
    monto: f64,
    pos: bool,
) -> Res<f64> {
    match sistema.lock() {
        Ok(mut a) => {
            a.arc_user();
            match a.agregar_pago(medio_pago, monto, pos) {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_pesable<'a>(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    precio_peso: f64,
    codigo: i64,
    costo_kilo: f64,
    porcentaje: f64,
    descripcion: &'a str,
) -> Res<String> {
    match sistema.lock() {
        Ok(sis) => match sis.arc_user().rango() {
            Rango::Admin => {
                let pesable = match async_runtime::block_on(Pesable::new_to_db(
                    sis.write_db(),
                    codigo,
                    precio_peso,
                    porcentaje,
                    costo_kilo,
                    descripcion,
                )) {
                    Ok(a) => a,
                    Err(e) => return Err(e.to_string()),
                };
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
                Ok(pesable.descripcion().to_string())
            }
            Rango::Cajero => Err(DENEGADO.to_string()),
        },
        Err(a) => Err(a.to_string()),
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
) -> Res<String> {
    match sistema.lock() {
        Ok(mut sis) => match sis.arc_user().rango() {
            Rango::Admin => {
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
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
                Ok(producto.nombre_completo())
            }
            Rango::Cajero => Err(DENEGADO.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    prod: Valuable,
    pos: bool,
) -> Res<Venta> {
    match sistema.lock() {
        Ok(mut sis) => {
            sis.arc_user();
            match async_runtime::block_on(sis.agregar_producto_a_venta(prod, pos)) {
                Ok(a) => {
                    loop {
                        if let Ok(_) = window
                            .menu_handle()
                            .get_item("confirm stash")
                            .set_enabled(true)
                        {
                            break;
                        }
                    }
                    Ok(a)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_proveedor(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    proveedor: &str,
    contacto: Option<i64>,
) -> Res<()> {
    match sistema.lock() {
        Ok(mut sis) => match sis.arc_user().rango() {
            Rango::Admin => {
                if let Err(e) = sis.agregar_proveedor(proveedor, contacto) {
                    return Err(e.to_string());
                }
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
                Ok(())
            }
            Rango::Cajero => Err(DENEGADO.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn agregar_rubro(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    codigo: i64,
    descripcion: &str,
) -> Res<String> {
    match sistema.lock() {
        Ok(sis) => match sis.arc_user().rango() {
            Rango::Admin => {
                let rubro = match async_runtime::block_on(Rubro::new_to_db(
                    codigo,
                    None,
                    descripcion,
                    sis.read_db(),
                )) {
                    Ok(a) => a,
                    Err(e) => return Err(e.to_string()),
                };
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
                Ok(rubro.descripcion().to_string())
            }
            Rango::Cajero => Err(DENEGADO.to_string()),
        },
        Err(a) => Err(a.to_string()),
    }
}
#[tauri::command]
fn agregar_usuario(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    id: &str,
    nombre: &str,
    pass: &str,
    rango: &str,
) -> Res<User> {
    match sistema.lock() {
        Ok(sis) => match sis.arc_user().rango() {
            Rango::Admin => match sis.agregar_usuario(id, nombre, pass, rango) {
                Ok(user) => {
                    loop{
                        if window.close().is_ok(){
                            break;
                        }
                    }
                    Ok(user)
                }
                Err(e) => Err(e.to_string()),
            },
            Rango::Cajero => Err(DENEGADO.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn buscador(name: &str) -> String {
    format!("Hello, {}! You've been mensajed from Rust!", name)
}
#[tauri::command]
fn cerrar_caja(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    monto_actual: f64,
) -> Res<()> {
    match sistema.lock() {
        Ok(mut sis) => {
            match sis.cerrar_caja(monto_actual) {
                Ok(_) => {
                    loop{
                        if window.close().is_ok(){
                            break;
                        }
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn check_codes(code: i64) -> Res<bool> {
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
fn close_window(window: tauri::Window) -> Res<()> {
    loop{
        if window.close().is_ok(){
            break;
        }
    }
    Ok(())
}
#[tauri::command]
fn descontar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    match sistema.lock() {
        Ok(mut a) => {
            a.arc_user();
            match a.descontar_producto_de_venta(index, pos) {
                Ok(a) => {
                    println!("{:?}", a);
                    Ok(a)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn eliminar_pago(sistema: State<Mutex<Sistema>>, pos: bool, index: usize) -> Res<Venta> {
    match sistema.lock() {
        Ok(mut a) => {
            a.arc_user();
            match a.eliminar_pago(pos, index) {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn eliminar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    match sistema.lock() {
        Ok(mut a) => {
            a.arc_user();
            match a.eliminar_producto_de_venta(index, pos) {
                Ok(a) => {
                    if a.productos().len() == 0 {
                        loop {
                            if let Ok(_) = window
                                .menu_handle()
                                .get_item("confirm stash")
                                .set_enabled(false)
                            {
                                break;
                            }
                        }
                    }
                    Ok(a)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn eliminar_usuario(sistema: State<Mutex<Sistema>>, user: User) -> Res<()> {
    match sistema.lock() {
        Ok(sis) => {
            if let Err(e) = sis.eliminar_usuario(user) {
                return Err(e.to_string());
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}
#[tauri::command]
fn get_caja(sistema: State<Mutex<Sistema>>) -> Res<Caja> {
    match sistema.lock() {
        Ok(sis) => Ok(sis.caja().clone()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_clientes(sistema: State<Mutex<Sistema>>) -> Res<Vec<Cli>> {
    match sistema.lock() {
        Ok(sis) => match async_runtime::block_on(sis.get_clientes()) {
            Ok(a) => {
                println!("{:#?}", a);
                Ok(a)
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_configs(sistema: State<Mutex<Sistema>>) -> Res<Config> {
    match sistema.lock() {
        Ok(sis) => Ok(sis.configs().clone()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_descripcion_valuable(prod: Valuable, conf: Config) -> String {
    let res;
    res = prod.descripcion(&conf);
    res
}
#[tauri::command]
fn get_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: &str,
    tipo_filtro: &str,
) -> Res<Vec<String>> {
    match sistema.lock() {
        Ok(a) => {
            a.arc_user();
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
    }
}
#[tauri::command]
fn get_medios_pago(sistema: State<Mutex<Sistema>>) -> Res<Vec<String>> {
    match sistema.lock() {
        Ok(sis) => {
            sis.arc_user();
            Ok(sis.configs().medios_pago().clone())
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_productos_filtrado(sistema: State<Mutex<Sistema>>, filtro: &str) -> Res<Vec<Valuable>> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            a.arc_user();
            match async_runtime::block_on(a.val_filtrado(filtro, a.read_db())) {
                Ok(a) => res = Ok(a),
                Err(e) => res = Err(e.to_string()),
            }
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}
#[tauri::command]
fn get_proveedores(sistema: State<'_, Mutex<Sistema>>) -> Res<Vec<String>> {
    let res;
    match sistema.lock() {
        Ok(a) => {
            a.arc_user();
            res = Ok(async_runtime::block_on(a.proveedores())
                .iter()
                .map(|x| x.to_string())
                .collect());
        }
        Err(e) => res = Err(e.to_string()),
    }
    res
}
#[tauri::command]
fn get_rango(sistema: State<Mutex<Sistema>>) -> Res<Rango> {
    match sistema.lock() {
        Ok(sis) => Ok(sis.arc_user().rango().clone()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_stash(sistema: State<Mutex<Sistema>>) -> Res<Vec<Venta>> {
    match sistema.lock() {
        Ok(sis) => {
            sis.arc_user();
            Ok(sis.stash().clone())
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_user(sistema: State<Mutex<Sistema>>) -> Res<User> {
    match sistema.lock() {
        Ok(sis) => Ok(sis.arc_user().as_ref().clone()),
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn get_venta_actual(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    pos: bool,
) -> Res<Venta> {
    match sistema.lock() {
        Ok(a) => {
            a.arc_user();
            let venta = a.venta(pos);
            if venta.productos().len() == 0 {
                loop {
                    if let Ok(_) = window
                        .menu_handle()
                        .get_item("confirm stash")
                        .set_enabled(false)
                    {
                        break;
                    }
                }
            } else {
                loop {
                    if let Ok(_) = window
                        .menu_handle()
                        .get_item("confirm stash")
                        .set_enabled(true)
                    {
                        break;
                    }
                }
            }
            Ok(venta)
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn incrementar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    match sistema.lock() {
        Ok(mut a) => {
            a.arc_user();
            match a.incrementar_producto_a_venta(index, pos) {
                Ok(a) => {
                    println!("{:?}", a);
                    Ok(a)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn open_add_prov(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-prov") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "add-prov", /* the unique window label */
                tauri::WindowUrl::App("/pages/add-prov.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(430.0, 110.0)
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
async fn open_add_select(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-select") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
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
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

#[tauri::command]
async fn open_add_user(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-user") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "add-user", /* the unique window label */
                tauri::WindowUrl::App("/pages/add-user.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(430.0, 200.0)
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
async fn open_add_cliente(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-cliente") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "add-cliente",
                tauri::WindowUrl::App("/pages/add-cliente.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(640.0, 400.0)
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
async fn open_cerrar_caja(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("cerrar-caja") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "cerrar-caja",
                tauri::WindowUrl::App("/pages/cerrar-caja.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(640.0, 620.0)
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
async fn open_confirm_stash(handle: tauri::AppHandle, act: bool) -> Res<()> {
    match handle.get_window("confirm-stash") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
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
            .menu(Menu::new())
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
                    for _ in 0..7 {
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
    }
}
#[tauri::command]
async fn open_edit_settings(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("edit-settings") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "edit-settings", /* the unique window label */
                tauri::WindowUrl::App("/pages/edit-settings.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(500.0, 360.0)
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
async fn open_login(handle: tauri::AppHandle) -> Res<()> {
    if let Err(e) = handle.get_window("main").unwrap().minimize() {
        return Err(e.to_string());
    }
    match handle.get_window("login") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "login", /* the unique window label */
                tauri::WindowUrl::App("/pages/login.html".parse().unwrap()),
            )
            .inner_size(400.0, 300.0)
            .resizable(false)
            .minimizable(false)
            .closable(false)
            .always_on_top(true)
            .decorations(false)
            .center()
            .menu(Menu::new())
            // .minimizable(false)
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
async fn open_stash(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("open-stash") {
        Some(window) => {
            if let Err(e) = window.show() {
                return Err(e.to_string());
            }
            Ok(())
        }
        None => {
            match tauri::WindowBuilder::new(
                &handle,
                "open-stash", /* the unique window label */
                tauri::WindowUrl::App("/pages/stash.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(900.0, 600.0)
            .menu(Menu::new())
            .build()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
#[tauri::command]
fn try_login(
    sistema: State<Mutex<Sistema>>,
    handle: tauri::AppHandle,
    window: tauri::Window,
    id: &str,
    pass: &str,
) -> Res<()> {
    match sistema.lock() {
        Ok(mut sis) => match async_runtime::block_on(sis.try_login(id, get_hash(pass))) {
            Ok(r) => {
                match r {
                    Rango::Cajero => {
                        let menu = handle.get_window("main").unwrap().menu_handle();
                        loop {
                            if try_disable_windows(menu.clone()).is_ok() {
                                break;
                            }
                        }
                    }
                    _ => (),
                }
                match window.emit(
                    "inicio-sesion",
                    Payload {
                        message: Some("Correcto".to_string()),
                        pos: None,
                    },
                ) {
                    Err(e) => return Err(e.to_string()),
                    Ok(_) => {
                        if let Some(window) = tauri::Window::get_window(&window, "main") {
                            if let Err(e) = window.maximize() {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}
#[tauri::command]
async fn select_window(handle: tauri::AppHandle, window: tauri::Window, dato: &str) -> Res<()> {
    let res;
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
        loop{
            if window.close().is_ok(){
                break;
            }
        }
    }
    res
}
#[tauri::command]
fn set_cliente(sistema: State<Mutex<Sistema>>,id:&str,pos:bool)->Res<()>{
    match sistema.lock(){
        Ok(mut sis)=>{
            let id=match id.parse::<i32>(){
                Ok(a)=>a,
                Err(e)=>return Err(e.to_string())
            };
            match sis.set_cliente(id,pos){
                Ok(_)=>Ok(()),
                Err(e)=>Err(e.to_string()),
            }
        }
        Err(e)=>Err(e.to_string())
    }
}
#[tauri::command]
fn set_configs(window: tauri::Window, sistema: State<Mutex<Sistema>>, configs: Config) -> Res<()> {
    match sistema.lock() {
        Ok(mut sis) => match sis.arc_user().rango() {
            Rango::Admin => {
                sis.set_configs(configs);
                loop{
                    if window.close().is_ok(){
                        break;
                    }
                }
                Ok(())
            }
            Rango::Cajero => Err(DENEGADO.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn stash_n_close(window: tauri::Window, sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
    match sistema.lock() {
        Ok(mut sis) => {
            sis.arc_user();
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
            loop{
                if window.close().is_ok(){
                    break;
                }
            }
            println!("{:#?}", sis.stash());
            Ok(())
        }

        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
fn unstash_sale(sistema: State<Mutex<Sistema>>, pos: bool, index: usize) -> Res<()> {
    match sistema.lock() {
        Ok(mut sis) => {
            sis.arc_user();
            match sis.unstash_sale(pos, index) {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let cerrar_caja_menu = CustomMenuItem::new(String::from("cerrar caja"), "Cerrar caja");
    let add_product_menu = CustomMenuItem::new(String::from("add product"), "Agregar producto");
    let add_prov_menu = CustomMenuItem::new(String::from("add prov"), "Agregar proveedor");
    let add_user_menu = CustomMenuItem::new(String::from("add user"), "Agregar usuario");
    let add_cliente_menu = CustomMenuItem::new(String::from("add cliente"), "Agregar cliente");
    let edit_settings_menu =
        CustomMenuItem::new(String::from("edit settings"), "Cambiar configuraciones");
    let confirm_stash_menu = CustomMenuItem::new(String::from("confirm stash"), "Guardar venta");
    let open_stash_menu = CustomMenuItem::new(String::from("open stash"), "Ver ventas guardadas");

    let administrar = Submenu::new(
        "Administrar",
        Menu::new()
            .add_item(add_product_menu)
            .add_item(add_prov_menu)
            .add_item(add_user_menu)
            .add_item(add_cliente_menu)
            .add_item(edit_settings_menu),
    );
    let venta = Submenu::new(
        "Venta",
        Menu::new()
            .add_item(confirm_stash_menu)
            .add_item(open_stash_menu),
    );
    let caja = Submenu::new("Caja", Menu::new().add_item(cerrar_caja_menu));
    let menu = Menu::new()
        .add_submenu(administrar)
        .add_submenu(venta)
        .add_submenu(caja);
    let app = tauri::Builder::default()
        .manage(Mutex::new(Sistema::new().unwrap()))
        .invoke_handler(tauri::generate_handler![
            agregar_cliente,
            agregar_pago,
            agregar_pesable,
            agregar_producto,
            agregar_producto_a_venta,
            agregar_proveedor,
            agregar_rubro,
            agregar_usuario,
            buscador,
            cerrar_caja,
            check_codes,
            close_window,
            descontar_producto_de_venta,
            eliminar_pago,
            eliminar_producto_de_venta,
            eliminar_usuario,
            get_caja,
            get_clientes,
            get_configs,
            get_descripcion_valuable,
            get_filtrado,
            get_medios_pago,
            get_productos_filtrado,
            get_proveedores,
            get_rango,
            get_stash,
            get_user,
            get_venta_actual,
            incrementar_producto_a_venta,
            open_add_prov,
            open_add_select,
            open_add_user,
            open_add_cliente,
            open_cerrar_caja,
            open_confirm_stash,
            open_edit_settings,
            open_login,
            open_stash,
            try_login,
            select_window,
            set_cliente,
            set_configs,
            stash_n_close,
            unstash_sale,
        ])
        .menu(menu)
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    let window = app.get_window("main").unwrap();
    let w2=window.clone();
    let handle = app.handle();
    window.on_menu_event(move |event| {
        match event.menu_item_id() {
            "add product" => async_runtime::block_on(open_add_select(handle.clone())),
            "add prov" => async_runtime::block_on(open_add_prov(handle.clone())),
            "add user" => async_runtime::block_on(open_add_user(handle.clone())),
            "add cliente" => async_runtime::block_on(open_add_cliente(handle.clone())),
            "edit settings" => async_runtime::block_on(open_edit_settings(handle.clone())),
            "confirm stash" => {
                loop {
                    if let Ok(_) = w2.emit(
                        "confirm-stash",
                        Payload {
                            message: Some(String::from("now")),
                            pos: None,
                        },
                    ) {
                        break;
                    }
                }

                Ok(())
            }
            "open stash" => async_runtime::block_on(open_stash(handle.clone())),
            "cerrar caja" => async_runtime::block_on(open_cerrar_caja(handle.clone())),

            //add_product, add_prov, add_user, add_cliente, edit_stetings
            //confirm_stash, cerrar_caja, open_stash,
            _ => Ok(()),
        }
        .unwrap();
    });
    app.run(|_, _| {})
}
