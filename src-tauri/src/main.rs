// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use entity::prelude::{CodeDB, PesDB, RubDB};
use mods::{
    Caja,
    Cli,
    Config,
    get_hash,
    Pago,
    Pesable,
    Rubro,
    Sistema,
    Rango, User,
    Valuable as V,
    Venta,Result as Res
};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use serde::Serialize;
use std::sync::Arc;
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
    val: Option<V>,
}
mod mods;
fn set_menus(menu: MenuHandle, state: bool) -> Result<()> {
    menu.get_item("add product").set_enabled(state)?;
    menu.get_item("add prov").set_enabled(state)?;
    menu.get_item("add user").set_enabled(state)?;
    Ok(menu.get_item("edit settings").set_enabled(state)?)
}
async fn open_add_product(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-product") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
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
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

async fn open_add_pesable(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-pesable") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
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
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
async fn open_add_rubro(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-rubro") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
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
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
fn agregar_cliente(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    nombre: &str,
    dni: &str,
    credito: bool,
    limite: Option<&str>,
) -> Res<Cli> {
    let dni = dni.parse::<i32>().map_err(|e| e.to_string())?;
    let limite = match limite {
        Some(l) => Some(l.parse::<f32>().map_err(|e| e.to_string())?),
        None => None,
    };
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let cli = sis.agregar_cliente(nombre, dni, credito, true, limite)?;
    loop {
        if window
            .emit(
                "main",
                Payload {
                    message: Some(String::from("dibujar venta")),
                    pos: None,
                    val: None,
                },
            )
            .is_ok()
        {
            break;
        }
    }
    close_window(window)?;
    Ok(cli)
}
#[tauri::command]
fn agregar_pago(
    sistema: State<Mutex<Sistema>>,
    medio_pago: &str,
    monto: &str,
    pos: bool,
) -> Res<Vec<Pago>> {
    let monto = monto.parse::<f32>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.agregar_pago(medio_pago, monto, pos)?;
    Ok(sis.venta(pos).pagos())
}
#[tauri::command]
fn agregar_pesable<'a>(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    precio_peso: &str,
    codigo: &str,
    costo_kilo: &str,
    porcentaje: &str,
    descripcion: &'a str,
) -> Res<String> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            let precio_peso = precio_peso.parse::<f32>().map_err(|e| e.to_string())?;
            let codigo = codigo.parse::<i64>().map_err(|e| e.to_string())?;
            let costo_kilo = costo_kilo.parse::<f32>().map_err(|e| e.to_string())?;
            let porcentaje = porcentaje.parse::<f32>().map_err(|e| e.to_string())?;
            let pesable = async_runtime::block_on(Pesable::new_to_db(
                sis.write_db(),
                codigo,
                precio_peso,
                porcentaje,
                costo_kilo,
                descripcion,
            ))?;
            close_window(window)?;
            Ok(pesable.descripcion().to_string())
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
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
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            let prod = block_on(sis.agregar_producto(
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
            ))?;
            close_window(window)?;
            Ok(format!("Agregado {:#?}", prod))
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
#[tauri::command]
fn agregar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    prod: V,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    match &prod {
        V::Prod(_) => {
            async_runtime::block_on(sis.agregar_producto_a_venta(prod, pos))?;
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
        V::Pes(a) => {
            async_runtime::spawn(open_select_amount(
                window.app_handle(),
                V::Pes(a.clone()),
                pos,
            ));
        }
        V::Rub(a) => {
            async_runtime::spawn(open_select_amount(
                window.app_handle(),
                V::Rub(a.clone()),
                pos,
            ));
        }
    }
    Ok(sis.venta(pos))
}

#[tauri::command]
fn agregar_proveedor(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    proveedor: &str,
    contacto: Option<i64>,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            sis.agregar_proveedor(proveedor, contacto)?;
            Ok(close_window(window)?)
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
#[tauri::command]
fn agregar_rubro(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    codigo: &str,
    descripcion: &str,
) -> Res<String> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            let codigo = codigo.parse::<i64>().map_err(|e| e.to_string())?;
            let rubro = async_runtime::block_on(Rubro::new_to_db(
                codigo,
                None,
                descripcion,
                sis.write_db(),
            ))?;
            close_window(window)?;
            Ok(rubro.descripcion().to_string())
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
#[tauri::command]
fn agregar_rub_o_pes_a_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    val: V,
    pos: bool,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    async_runtime::block_on(sis.agregar_producto_a_venta(val, pos))?;
    loop {
        if window
            .emit(
                "main",
                Payload {
                    message: Some(String::from("dibujar venta")),
                    pos: None,
                    val: None,
                },
            )
            .is_ok()
        {
            break;
        }
    }
    Ok(close_window(window)?)
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
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            let res = sis.agregar_usuario(id, nombre, pass, rango)?;
            close_window(window)?;
            Ok(res)
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
#[tauri::command]
fn buscador(name: &str) -> String {
    format!("Hello, {}! You've been mensajed from Rust!", name)
}
#[tauri::command]
async fn cerrar_sesion<'ab>(
    sistema: State<'ab, Mutex<Sistema>>,
    handle: tauri::AppHandle,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    match handle.get_window("login") {
        Some(window) => {
            loop {
                if window.show().is_ok() {
                    break;
                }
            }

            Ok(sis.cerrar_sesion())
        }
        None => {
            tauri::WindowBuilder::new(
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
            .build()
            .map_err(|e| e.to_string())?;
            Ok(sis.cerrar_sesion())
        }
    }
}
#[tauri::command]
fn cancelar_venta(sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.cancelar_venta(pos).map_err(|e| e.to_string())
}
#[tauri::command]
fn cerrar_caja(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    monto_actual: f32,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.cerrar_caja(monto_actual)?;
    Ok(close_window(window)?)
}
#[tauri::command]
async fn check_codes(code: i64) -> Res<bool> {
    let db = Database::connect("sqlite://db.sqlite?mode=ro")
        .await
        .map_err(|e| e.to_string())?;
    let disp;
    let mod_opt = CodeDB::Entity::find()
        .filter(CodeDB::Column::Codigo.eq(code))
        .one(&db)
        .await
        .map_err(|e| e.to_string())?;
    disp = match mod_opt {
        Some(_) => false,
        None => {
            match PesDB::Entity::find()
                .filter(PesDB::Column::Codigo.eq(code))
                .one(&db)
                .await
            {
                Ok(a) => {
                    if a.is_none() {
                        match RubDB::Entity::find()
                            .filter(RubDB::Column::Codigo.eq(code))
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
        }
    };
    Ok(disp)
}
#[tauri::command]
fn close_window(window: tauri::Window) -> Res<()> {
    loop {
        if window.close().is_ok() {
            break;
        }
    }
    Ok(())
}
#[tauri::command]
fn descontar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let res = sis.descontar_producto_de_venta(index, pos)?;
    loop {
        if window
            .emit(
                "main",
                Payload {
                    message: Some("dibujar venta".to_string()),
                    pos: None,
                    val: None,
                },
            )
            .is_ok()
        {
            break;
        };
    }
    Ok(res)
}
#[tauri::command]
fn editar_producto(sistema: State<Mutex<Sistema>>, prod: V)->Res<()>{
    let sis=sistema.lock().map_err(|e|e.to_string())?;
    sis.access();
    sis.editar_valuable(prod);
    Ok(())
}
#[tauri::command]
fn eliminar_pago(sistema: State<Mutex<Sistema>>, pos: bool, id: &str) -> Res<Vec<Pago>> {
    let id = id.parse::<u32>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.eliminar_pago(pos, id).map_err(|e| e.to_string())
}
#[tauri::command]
fn eliminar_producto(sistema: State<Mutex<Sistema>>, prod: V)->Res<()>{
    let sis=sistema.lock().map_err(|e|e.to_string())?;
    sis.access();
    sis.eliminar_valuable(prod);
    Ok(())
}
#[tauri::command]
fn eliminar_producto_de_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let res = sis.eliminar_producto_de_venta(index, pos)?;
    loop {
        if window
            .menu_handle()
            .get_item("confirm stash")
            .set_enabled(false)
            .is_ok()
        {
            break;
        }
    }
    Ok(res)
}
#[tauri::command]
fn eliminar_usuario(sistema: State<Mutex<Sistema>>, user: User) -> Res<()> {
    let res = sistema.lock().map_err(|e| e.to_string())?;
    res.access();
    Ok(res.eliminar_usuario(user)?)
}
#[tauri::command]
fn get_caja(sistema: State<Mutex<Sistema>>) -> Res<Caja> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.caja().clone())
}
#[tauri::command]
fn get_clientes(sistema: State<Mutex<Sistema>>) -> Res<Vec<Cli>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(async_runtime::block_on(sis.get_clientes())?)
}
#[tauri::command]
fn get_configs(sistema: State<Mutex<Sistema>>) -> Res<Config> {
    Ok(sistema.lock().map_err(|e| e.to_string())?.configs().clone())
}
#[tauri::command]
fn get_descripciones(prods:Vec<V>,conf:Config)->Vec<(String,Option<f32>)>{
    prods.iter().map(|p|(p.descripcion(&conf),p.price(&conf.politica()))).collect::<Vec<(String,Option<f32>)>>()
}
#[tauri::command]
fn get_descripcion_valuable(prod: V, conf: Config) -> String {
    prod.descripcion(&conf)
}
#[tauri::command]
fn get_deuda(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<f32> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.get_deuda(cliente).map_err(|e| e.to_string())
}
#[tauri::command]
fn get_deuda_detalle(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<Vec<Venta>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.get_deuda_detalle(cliente).map_err(|e| e.to_string())
}
#[tauri::command]
fn get_filtrado(
    sistema: State<Mutex<Sistema>>,
    filtro: &str,
    tipo_filtro: &str,
) -> Res<Vec<String>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    match tipo_filtro {
        "marca" => Ok(sis.filtrar_marca(filtro)?),
        "tipo_producto" => Ok(sis.filtrar_tipo_producto(filtro)?),
        _ => Err(format!("Parámetro incorrecto {tipo_filtro}")),
    }
}
#[tauri::command]
fn get_log_state(sistema: State<Mutex<Sistema>>) -> Res<bool>{
    Ok(sistema.lock().map_err(|e|e.to_string())?.user().is_some())
}
#[tauri::command]
fn get_medios_pago(sistema: State<Mutex<Sistema>>) -> Res<Vec<String>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.configs().medios_pago().iter().map(|m|m.to_string()).collect())
}
#[tauri::command]
fn get_productos_filtrado(sistema: State<Mutex<Sistema>>, filtro: &str) -> Res<Vec<V>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(async_runtime::block_on(
        sis.val_filtrado(filtro, sis.read_db()),
    )?)
}
#[tauri::command]
fn get_proveedores(sistema: State<'_, Mutex<Sistema>>) -> Res<Vec<String>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(async_runtime::block_on(sis.proveedores())
        .iter()
        .map(|x| x.to_string())
        .collect())
}
#[tauri::command]
fn get_rango(sistema: State<Mutex<Sistema>>) -> Res<Rango> {
    Ok(sistema
        .lock()
        .map_err(|e| e.to_string())?
        .arc_user()
        .rango()
        .clone())
}
#[tauri::command]
fn get_stash(sistema: State<Mutex<Sistema>>) -> Res<Vec<Venta>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.stash().clone())
}
#[tauri::command]
fn get_user(sistema: State<Mutex<Sistema>>) -> Res<User> {
    Ok(sistema
        .lock()
        .map_err(|e| e.to_string())?
        .arc_user()
        .as_ref()
        .clone())
}
#[tauri::command]
fn get_venta_actual(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    pos: bool,
) -> Res<Venta> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let venta = sis.venta(pos);
    if venta.productos().len() == 0 {
        loop {
            if window
                .menu_handle()
                .get_item("confirm stash")
                .set_enabled(false)
                .is_ok()
            {
                break;
            }
        }
    } else {
        loop {
            if window
                .menu_handle()
                .get_item("confirm stash")
                .set_enabled(true)
                .is_ok()
            {
                break;
            }
        }
    }
    println!("{:#?}", venta);
    Ok(venta)
}
#[tauri::command]
fn hacer_egreso(sistema: State<Mutex<Sistema>>, monto: f32, descripcion: Option<&str>) -> Res<()> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    Ok(sis.hacer_egreso(monto, descripcion.map(|d| Arc::from(d)))?)
}
#[tauri::command]
fn hacer_ingreso(sistema: State<Mutex<Sistema>>, monto: f32, descripcion: Option<&str>) -> Res<()> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    Ok(sis.hacer_ingreso(monto, descripcion.map(|d| Arc::from(d)))?)
}
#[tauri::command]
fn incrementar_producto_a_venta(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let venta = sis.incrementar_producto_a_venta(index, pos)?;
    loop {
        if window
            .emit(
                "main",
                Payload {
                    message: Some("dibujar venta".to_string()),
                    pos: None,
                    val: None,
                },
            )
            .is_ok()
        {
            break;
        }
    }
    Ok(venta)
}
#[tauri::command]
async fn open_add_prov(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-prov") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
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
            .title("Agregar Proveedor")
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_add_select(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-select") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-select",
                tauri::WindowUrl::App("/pages/add-select.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .title("Seleccione una opción")
            .inner_size(210.0, 80.0)
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

#[tauri::command]
async fn open_add_user(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-user") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-user", /* the unique window label */
                tauri::WindowUrl::App("/pages/add-user.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .title("Agregar Usuario")
            .inner_size(430.0, 200.0)
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_add_cliente(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-cliente") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-cliente",
                tauri::WindowUrl::App("/pages/add-cliente.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .title("Agregar Cliente")
            .inner_size(640.0, 400.0)
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_cancelar_venta(handle: tauri::AppHandle, act: bool) -> Res<()> {
    match handle.get_window("confirm-cancel") {
        Some(window) => {
            window.show().map_err(|e| e.to_string())?;
            window
                .emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("cancelar venta")),
                        pos: Some(act),
                        val: None,
                    },
                )
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        None => {
            let win = tauri::WindowBuilder::new(
                &handle,
                "confirm-cancel",
                tauri::WindowUrl::App("/pages/confirm.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(400.0, 150.0)
            .menu(Menu::new())
            .title("Confirmar")
            .build()
            .map_err(|e| e.to_string())?;
            std::thread::sleep(std::time::Duration::from_millis(500));
            win.emit(
                "get-venta",
                Payload {
                    message: Some(String::from("cancelar venta")),
                    pos: Some(act),
                    val: None,
                },
            )
            .map_err(|e| e.to_string())?;
            for _ in 0..7 {
                std::thread::sleep(std::time::Duration::from_millis(175));
                win.emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("cancelar venta")),
                        pos: Some(act),
                        val: None,
                    },
                )
                .map_err(|e| e.to_string())?;
            }
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_cerrar_caja(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("cerrar-caja") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "cerrar-caja",
                tauri::WindowUrl::App("/pages/cerrar-caja.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .title("Cerrar Caja")
            .inner_size(640.0, 620.0)
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_confirm_stash(handle: tauri::AppHandle, act: bool) -> Res<()> {
    match handle.get_window("confirm") {
        Some(window) => {
            window.show().map_err(|e| e.to_string())?;
            window
                .emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("stash")),
                        pos: Some(act),
                        val: None,
                    },
                )
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        None => {
            let win = tauri::WindowBuilder::new(
                &handle,
                "confirm", /* the unique window label */
                tauri::WindowUrl::App("/pages/confirm.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(400.0, 150.0)
            .menu(Menu::new())
            .title("Confirmar Stash")
            .build()
            .map_err(|e| e.to_string())?;
            std::thread::sleep(std::time::Duration::from_millis(500));
            win.emit(
                "get-venta",
                Payload {
                    message: Some(String::from("stash")),
                    pos: Some(act),
                    val: None,
                },
            )
            .map_err(|e| e.to_string())?;
            for _ in 0..7 {
                std::thread::sleep(std::time::Duration::from_millis(175));
                win.emit(
                    "get-venta",
                    Payload {
                        message: Some(String::from("stash")),
                        pos: Some(act),
                        val: None,
                    },
                )
                .map_err(|e| e.to_string())?;
            }
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_edit_settings(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("edit-settings") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
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
            .title("Configuraciones")
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_login(handle: tauri::AppHandle) -> Res<()> {
    handle
        .get_window("main")
        .unwrap()
        .minimize()
        .map_err(|e| e.to_string())?;
    match handle.get_window("login") {
        Some(window) => {
            window.show().map_err(|e| e.to_string())?;
            Ok(window.set_focus().map_err(|e| e.to_string())?)
        }
        None => {
            let window = tauri::WindowBuilder::new(
                &handle,
                "login", /* the unique window label */
                tauri::WindowUrl::App("src/pages/login/login.html".parse().unwrap()),
            )
            .inner_size(400.0, 300.0)
            .resizable(false)
            .minimizable(false)
            .closable(false)
            .always_on_top(true)
            .decorations(false)
            .center()
            .title("Iniciar Sesión")
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
#[tauri::command]
async fn open_select_amount(handle: tauri::AppHandle, val: V, pos: bool) -> Res<()> {
    match handle.get_window("select-amount") {
        Some(window) => {
            window.show().map_err(|e| e.to_string())?;
            std::thread::sleep(std::time::Duration::from_millis(400));
            let mut res = Err(String::from("Error emitiendo mensaje"));
            for _ in 0..8 {
                std::thread::sleep(std::time::Duration::from_millis(175));
                if window
                    .emit(
                        "select-amount",
                        Payload {
                            message: None,
                            pos: Some(pos),
                            val: Some(val.clone()),
                        },
                    )
                    .is_ok()
                {
                    res = Ok(());
                }
            }
            res
        }
        None => {
            let window = tauri::WindowBuilder::new(
                &handle,
                "select-amount",
                tauri::WindowUrl::App("/pages/select-amount.html".parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(200.0, 100.0)
            .menu(Menu::new())
            .title("Seleccione Monto")
            .build()
            .map_err(|e| e.to_string())?;
            std::thread::sleep(std::time::Duration::from_millis(400));
            let mut res = Err(String::from("Error emitiendo mensaje"));
            for _ in 0..8 {
                std::thread::sleep(std::time::Duration::from_millis(175));
                if window
                    .emit(
                        "select-amount",
                        Payload {
                            message: None,
                            pos: Some(pos),
                            val: Some(val.clone()),
                        },
                    )
                    .is_ok()
                {
                    res = Ok(());
                }
            }
            res
        }
    }
}
#[tauri::command]
async fn open_stash<'a>(
    handle: tauri::AppHandle,
    sistema: State<'a, Mutex<Sistema>>,
    pos: bool,
) -> Res<()> {
    if sistema.lock().map_err(|e| e.to_string())?.stash().len() == 0 {
        Err("Stash vacío".to_string())
    } else {
        match handle.get_window("open-stash") {
            Some(window) => {
                window.show().map_err(|e| e.to_string())?;
                for _ in 0..7 {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    window
                        .emit(
                            "stash",
                            Payload {
                                message: None,
                                pos: Some(pos),
                                val: None,
                            },
                        )
                        .map_err(|e| e.to_string())?;
                }
            }
            None => {
                let win = tauri::WindowBuilder::new(
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
                .title("Ventas en Stash")
                .build()
                .map_err(|e| e.to_string())?;
                for _ in 0..7 {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    win.emit(
                        "stash",
                        Payload {
                            message: None,
                            pos: Some(pos),
                            val: None,
                        },
                    )
                    .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }
}
#[tauri::command]
fn pagar_deuda_especifica(
    sistema: State<Mutex<Sistema>>,
    cliente: i32,
    venta: Venta,
) -> Res<Venta> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.pagar_deuda_especifica(cliente, venta)?)
}
#[tauri::command]
fn pagar_deuda_general(sistema: State<Mutex<Sistema>>, cliente: i32, monto: f32) -> Res<f32> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.pagar_deuda_general(cliente, monto)?)
}
#[tauri::command]
fn try_login(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    id: &str,
    pass: &str,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    let rango = async_runtime::block_on(sis.try_login(id, get_hash(pass)))?;
    let menu = window
        .app_handle()
        .get_window("main")
        .unwrap()
        .menu_handle();
    match rango {
        Rango::Cajero => loop {
            if set_menus(menu.clone(), false).is_ok() {
                break;
            }
        },
        Rango::Admin => loop {
            if set_menus(menu.clone(), true).is_ok() {
                break;
            }
        },
    }
    window
        .emit(
            "main",
            Payload {
                message: Some("inicio sesion".to_string()),
                pos: None,
                val: None,
            },
        )
        .map_err(|e| e.to_string())?;
    if let Some(window) = tauri::Window::get_window(&window, "main") {
        window.maximize().map_err(|e| e.to_string())?;
    }
    Ok(close_window(window)?)
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
        close_window(window)?;
    }
    res
}
#[tauri::command]
fn set_cliente(sistema: State<Mutex<Sistema>>, id: &str, pos: bool) -> Res<Venta> {
    let id = id.parse::<i32>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.set_cliente(id, pos)?;
    Ok(sis.venta(pos))
}
#[tauri::command]
fn set_configs(window: tauri::Window, sistema: State<Mutex<Sistema>>, configs: Config) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            sis.set_configs(configs);
            Ok(close_window(window)?)
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}

#[tauri::command]
fn stash_n_close(window: tauri::Window, sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.stash_sale(pos)?;
    loop {
        if window
            .emit(
                "main",
                Payload {
                    message: Some("dibujar venta".into()),
                    pos: None,
                    val: None,
                },
            )
            .is_ok()
        {
            break;
        }
    }
    println!("{:#?}", sis.stash());
    Ok(close_window(window)?)
}
#[tauri::command]
fn unstash_sale(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    pos: bool,
    index: &str,
) -> Res<()> {
    let index = index.parse::<usize>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    loop {
        if window.close().is_ok() {
            break;
        }
    }
    loop {
        if window
            .emit(
                "main",
                Payload {
                    message: Some(String::from("dibujar venta")),
                    pos: None,
                    val: None,
                },
            )
            .is_ok()
        {
            break;
        }
    }
    Ok(sis.unstash_sale(pos, index)?)
}

fn main() {
    let cerrar_caja_menu = CustomMenuItem::new(String::from("cerrar caja"), "Cerrar caja");
    let add_product_menu = CustomMenuItem::new(String::from("add product"), "Agregar producto");
    let add_prov_menu = CustomMenuItem::new(String::from("add prov"), "Agregar proveedor");
    let add_user_menu = CustomMenuItem::new(String::from("add user"), "Agregar usuario");
    let add_cliente_menu = CustomMenuItem::new(String::from("add cliente"), "Agregar cliente");
    let cerrar_sesion_menu = CustomMenuItem::new(String::from("cerrar sesion"), "Cerrar sesión");
    let edit_settings_menu =
        CustomMenuItem::new(String::from("edit settings"), "Cambiar configuraciones");
    let confirm_stash_menu = CustomMenuItem::new(String::from("confirm stash"), "Guardar venta");
    let open_stash_menu = CustomMenuItem::new(String::from("open stash"), "Ver ventas guardadas");

    let opciones = Submenu::new(
        "Opciones",
        Menu::new()
            .add_item(add_cliente_menu)
            .add_item(add_product_menu)
            .add_item(add_prov_menu)
            .add_item(add_user_menu)
            .add_item(edit_settings_menu)
            .add_item(cerrar_sesion_menu),
    );
    let venta = Submenu::new(
        "Venta",
        Menu::new()
            .add_item(confirm_stash_menu)
            .add_item(open_stash_menu),
    );
    let caja = Submenu::new("Caja", Menu::new().add_item(cerrar_caja_menu));
    let menu = Menu::new()
        .add_submenu(opciones)
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
            agregar_rub_o_pes_a_venta,
            agregar_usuario,
            buscador,
            cancelar_venta,
            cerrar_caja,
            cerrar_sesion,
            check_codes,
            close_window,
            descontar_producto_de_venta,
            editar_producto,
            eliminar_pago,
            eliminar_producto,
            eliminar_producto_de_venta,
            eliminar_usuario,
            get_caja,
            get_clientes,
            get_configs,
            get_descripciones,
            get_descripcion_valuable,
            get_deuda,
            get_deuda_detalle,
            get_filtrado,
            get_log_state,
            get_medios_pago,
            get_productos_filtrado,
            get_proveedores,
            get_rango,
            get_stash,
            get_user,
            get_venta_actual,
            hacer_egreso,
            hacer_ingreso,
            incrementar_producto_a_venta,
            open_add_prov,
            open_add_select,
            open_add_user,
            open_add_cliente,
            open_cancelar_venta,
            open_cerrar_caja,
            open_confirm_stash,
            open_edit_settings,
            open_login,
            open_select_amount,
            open_stash,
            pagar_deuda_especifica,
            pagar_deuda_general,
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
    let w2 = window.clone();
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
                    if w2
                        .emit(
                            "main",
                            Payload {
                                message: Some(String::from("confirm stash")),
                                pos: None,
                                val: None,
                            },
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }
            "cerrar sesion" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload {
                                message: Some(String::from("cerrar sesion")),
                                pos: None,
                                val: None,
                            },
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }

            "open stash" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload {
                                message: Some(String::from("open stash")),
                                pos: None,
                                val: None,
                            },
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }
            "cerrar caja" => async_runtime::block_on(open_cerrar_caja(handle.clone())),

            _ => Ok(()),
        }
        .unwrap();
    });
    app.run(|_, _| {})
}