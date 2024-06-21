use super::{
    get_hash, Caja, Cli, Config, Pago, Pesable, Rango, Result as Res, Rubro, Sistema, User,
    Valuable as V, Venta,
};
use serde::Serialize;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{
    async_runtime::{self, block_on},
    window::MenuHandle,
    CustomMenuItem, Manager, Menu, Result, State, Submenu,
};
use tokio::time::sleep;
const INDEX: &str = "index.html";
const DENEGADO: &str = "Permiso denegado";
#[derive(Clone, Serialize)]
pub struct Payload {
    message: Option<String>,
    pos: Option<bool>,
    val: Option<V>,
}
impl Payload {
    pub fn new(message: Option<String>, pos: Option<bool>, val: Option<V>) -> Payload {
        Payload { message, pos, val }
    }
}
pub fn get_menu() -> Menu {
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
    Menu::new()
        .add_submenu(opciones)
        .add_submenu(venta)
        .add_submenu(caja)
}
fn set_menus(menu: MenuHandle, state: bool) -> Result<()> {
    menu.get_item("add product").set_enabled(state)?;
    menu.get_item("add prov").set_enabled(state)?;
    menu.get_item("add user").set_enabled(state)?;
    Ok(menu.get_item("edit settings").set_enabled(state)?)
}

pub fn agregar_cliente_2(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    nombre: &str,
    dni: &str,
    limite: Option<&str>,
) -> Res<Cli> {
    let dni = dni.parse::<i64>().map_err(|e| e.to_string())?;
    let limite = match limite {
        Some(l) => Some(l.parse::<f32>().map_err(|e| e.to_string())?),
        None => None,
    };
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let cli = sis.agregar_cliente(nombre, dni, true, limite)?;
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
    close_window_2(window)?;
    Ok(cli)
}

pub fn agregar_pago_2(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    medio_pago: &str,
    monto: &str,
    pos: bool,
) -> Res<Vec<Pago>> {
    let monto = monto.parse::<f32>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.agregar_pago(medio_pago, monto, pos)?;
    if sis.venta(pos).pagos().len() == 0 {
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
    }
    Ok(sis.venta(pos).pagos())
}
pub fn agregar_pesable_2<'a>(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    precio_peso: &str,
    codigo: &str,
    costo_kilo: &str,
    porcentaje: &str,
    descripcion: &'a str,
) -> Res<String> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    match sis.arc_user().rango() {
        Rango::Admin => {
            let precio_peso = precio_peso.parse::<f32>().map_err(|e| e.to_string())?;
            let codigo = codigo.parse::<i64>().map_err(|e| e.to_string())?;
            let costo_kilo = costo_kilo.parse::<f32>().map_err(|e| e.to_string())?;
            let porcentaje = match porcentaje.parse::<f32>().map_err(|e| e.to_string()) {
                Ok(res) => {
                    println!("salio bien {}", res);
                    res
                }
                Err(e) => {
                    println!("salio mal {e}");
                    return Err(e.into());
                }
            };
            let pesable = block_on(Pesable::new_to_db(
                sis.write_db(),
                codigo,
                precio_peso,
                porcentaje,
                costo_kilo,
                descripcion,
            ))?;
            close_window_2(window)?;
            Ok(pesable.descripcion().to_string())
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
pub fn agregar_producto_2(
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
            close_window_2(window)?;
            Ok(format!("Agregado {prod:#?}"))
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
pub fn agregar_producto_a_venta_2(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    prod: V,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    match &prod {
        V::Prod(_) => {
            block_on(sis.agregar_producto_a_venta(prod, pos))?;
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
            async_runtime::spawn(open_select_amount_2(
                window.app_handle(),
                V::Pes(a.clone()),
                pos,
            ));
        }
        V::Rub(a) => {
            async_runtime::spawn(open_select_amount_2(
                window.app_handle(),
                V::Rub(a.clone()),
                pos,
            ));
        }
    }
    Ok(sis.venta(pos))
}

pub fn agregar_proveedor_2(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    proveedor: &str,
    contacto: Option<&str>,
) -> Res<()> {
    let contacto = match contacto {
        Some(c) => {
            if c.len() > 0 {
                Some(c.parse::<i64>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        }
        None => None,
    };
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            sis.agregar_proveedor(proveedor, contacto)?;
            Ok(close_window_2(window)?)
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
pub fn agregar_rubro_2(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    codigo: &str,
    descripcion: &str,
) -> Res<String> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            let codigo = codigo.parse::<i64>().map_err(|e| e.to_string())?;
            let rubro = block_on(Rubro::new_to_db(codigo, None, descripcion, sis.write_db()))?;
            close_window_2(window)?;
            Ok(rubro.descripcion().to_string())
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
pub fn agregar_rub_o_pes_a_venta_2(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    val: V,
    pos: bool,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    block_on(sis.agregar_producto_a_venta(val, pos))?;
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
    Ok(close_window_2(window)?)
}
pub fn agregar_usuario_2(
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
            close_window_2(window)?;
            Ok(res)
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}
pub async fn cerrar_sesion_2<'a>(
    sistema: State<'a, Mutex<Sistema>>,
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
pub fn cancelar_venta_2(sistema: State<Mutex<Sistema>>, pos: bool) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.cancelar_venta(pos).map_err(|e| e.to_string())
}
pub fn cerrar_caja_2(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    monto_actual: f32,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.cerrar_caja(monto_actual)?;
    Ok(close_window_2(window)?)
}
pub fn close_window_2(window: tauri::Window) -> Res<()> {
    loop {
        if window.close().is_ok() {
            break;
        }
    }
    Ok(())
}
pub fn descontar_producto_de_venta_2(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let res = sis.descontar_producto_de_venta(index, pos)?;
    Ok(res)
}
pub fn editar_producto_2(sistema: State<Mutex<Sistema>>, prod: V) -> Res<()> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.editar_valuable(prod);
    Ok(())
}
pub fn eliminar_pago_2(sistema: State<Mutex<Sistema>>, pos: bool, id: &str) -> Res<Vec<Pago>> {
    let id = id.parse::<i64>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.eliminar_pago(pos, id).map_err(|e| e.to_string())
}
pub fn eliminar_producto_2(sistema: State<Mutex<Sistema>>, prod: V) -> Res<()> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.eliminar_valuable(prod);
    Ok(())
}
pub fn eliminar_producto_de_venta_2(
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
pub fn eliminar_usuario_2(sistema: State<Mutex<Sistema>>, user: User) -> Res<()> {
    let res = sistema.lock().map_err(|e| e.to_string())?;
    res.access();
    Ok(res.eliminar_usuario(user)?)
}
pub fn get_caja_2(sistema: State<Mutex<Sistema>>) -> Res<Caja> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.caja().clone())
}
pub fn get_clientes_2(sistema: State<Mutex<Sistema>>) -> Res<Vec<Cli>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(block_on(sis.get_clientes())?)
}
pub fn get_configs_2(sistema: State<Mutex<Sistema>>) -> Res<Config> {
    Ok(sistema.lock().map_err(|e| e.to_string())?.configs().clone())
}
pub fn get_descripciones_2(prods: Vec<V>, conf: Config) -> Vec<(String, Option<f32>)> {
    prods
        .iter()
        .map(|p| (p.descripcion(&conf), p.price(&conf.politica())))
        .collect::<Vec<(String, Option<f32>)>>()
}
pub fn get_descripcion_valuable_2(prod: V, conf: Config) -> String {
    prod.descripcion(&conf)
}
pub fn get_deuda_2(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<f32> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.get_deuda(cliente).map_err(|e| e.to_string())
}
pub fn get_deuda_detalle_2(sistema: State<Mutex<Sistema>>, cliente: Cli) -> Res<Vec<Venta>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    sis.get_deuda_detalle(cliente).map_err(|e| e.to_string())
}
pub fn get_filtrado_2(
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
pub fn get_log_state_2(sistema: State<Mutex<Sistema>>) -> Res<bool> {
    Ok(sistema.lock().map_err(|e| e.to_string())?.user().is_some())
}
pub fn get_medios_pago_2(sistema: State<Mutex<Sistema>>) -> Res<Vec<String>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis
        .configs()
        .medios_pago()
        .iter()
        .map(|m| m.to_string())
        .collect())
}
pub fn get_productos_filtrado_2(sistema: State<Mutex<Sistema>>, filtro: &str) -> Res<Vec<V>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(block_on(sis.val_filtrado(filtro, sis.read_db()))?)
}
pub fn get_proveedores_2(sistema: State<'_, Mutex<Sistema>>) -> Res<Vec<String>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(block_on(sis.proveedores())?
        .iter()
        .map(|x| x.to_string())
        .collect())
}
pub fn get_rango_2(sistema: State<Mutex<Sistema>>) -> Res<Rango> {
    Ok(sistema
        .lock()
        .map_err(|e| e.to_string())?
        .arc_user()
        .rango()
        .clone())
}
pub fn get_stash_2(sistema: State<Mutex<Sistema>>) -> Res<Vec<Venta>> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.stash().clone())
}
pub fn get_user_2(sistema: State<Mutex<Sistema>>) -> Res<User> {
    Ok(sistema
        .lock()
        .map_err(|e| e.to_string())?
        .arc_user()
        .as_ref()
        .clone())
}
pub fn get_venta_actual_2(
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
pub fn hacer_egreso_2(
    sistema: State<Mutex<Sistema>>,
    monto: f32,
    descripcion: Option<&str>,
) -> Res<()> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    Ok(sis.hacer_egreso(monto, descripcion.map(|d| Arc::from(d)))?)
}
pub fn hacer_ingreso_2(
    sistema: State<Mutex<Sistema>>,
    monto: f32,
    descripcion: Option<&str>,
) -> Res<()> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    Ok(sis.hacer_ingreso(monto, descripcion.map(|d| Arc::from(d)))?)
}
pub fn incrementar_producto_a_venta_2(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    pos: bool,
) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    let venta = sis.incrementar_producto_a_venta(index, pos)?;
    Ok(venta)
}
pub async fn open_add_prov_2(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-prov") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-prov", /* the unique window label */
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .inner_size(330.0, 210.0)
            .menu(Menu::new())
            .title("Agregar Proveedor")
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
pub async fn open_add_product_2(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-prod") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-prod",
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .title("Seleccione una opción")
            .inner_size(600.0, 380.0)
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

pub async fn open_add_user_2(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-user") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-user", /* the unique window label */
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
pub async fn open_add_cliente_2(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("add-cliente") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "add-cliente",
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
            )
            .always_on_top(true)
            .center()
            .resizable(false)
            .minimizable(false)
            .title("Agregar Cliente")
            .inner_size(400.0, 230.0)
            .menu(Menu::new())
            .build()
            .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}
pub async fn open_cancelar_venta_2(handle: tauri::AppHandle, act: bool) -> Res<()> {
    //TODO!(Hay que ver si es necesario usar un mismo html o no asi evi el window.emit)
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
            tauri::WindowBuilder::new(
                &handle,
                "confirm-cancel",
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
            Ok(())
        }
    }
}
pub async fn open_cerrar_caja_2(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("cerrar-caja") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "cerrar-caja",
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
pub async fn open_confirm_stash_2(handle: tauri::AppHandle, act: bool) -> Res<()> {
    //TODO!(Aca la otra parte que usa el confirm)
    match handle.get_window("confirm-stash") {
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
                "confirm-stash", /* the unique window label */
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
            sleep(std::time::Duration::from_millis(500)).await;
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
                sleep(std::time::Duration::from_millis(175)).await;
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
pub async fn open_edit_settings_2(handle: tauri::AppHandle) -> Res<()> {
    match handle.get_window("edit-settings") {
        Some(window) => Ok(window.show().map_err(|e| e.to_string())?),
        None => {
            tauri::WindowBuilder::new(
                &handle,
                "edit-settings", /* the unique window label */
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
pub async fn open_login_2(handle: tauri::AppHandle) -> Res<()> {
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
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
pub async fn open_select_amount_2(handle: tauri::AppHandle, val: V, pos: bool) -> Res<()> {
    match handle.get_window("select-amount") {
        Some(window) => {
            window.show().map_err(|e| e.to_string())?;
            sleep(std::time::Duration::from_millis(400)).await;
            let mut res = Err(String::from("Error emitiendo mensaje"));
            for _ in 0..8 {
                sleep(std::time::Duration::from_millis(175)).await;
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
                tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
            sleep(std::time::Duration::from_millis(400)).await;
            let mut res = Err(String::from("Error emitiendo mensaje"));
            for _ in 0..8 {
                sleep(std::time::Duration::from_millis(175)).await;
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
pub async fn open_stash_2<'a>(
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
                    sleep(std::time::Duration::from_millis(250)).await;
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
                    tauri::WindowUrl::App(INDEX.parse().unwrap()),
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
                    sleep(std::time::Duration::from_millis(250)).await;
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
pub fn pagar_deuda_especifica_2(
    sistema: State<Mutex<Sistema>>,
    cliente: i64,
    venta: Venta,
) -> Res<Venta> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.pagar_deuda_especifica(cliente, venta)?)
}
pub fn pagar_deuda_general_2(sistema: State<Mutex<Sistema>>, cliente: i64, monto: f32) -> Res<f32> {
    let sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.pagar_deuda_general(cliente, monto)?)
}
pub fn set_cantidad_producto_venta_2(
    sistema: State<Mutex<Sistema>>,
    index: usize,
    cantidad: &str,
    pos: bool,
) -> Res<Venta> {
    let cantidad = cantidad.parse::<f32>().map_err(|e| e.to_string())?;
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.access();
    Ok(sis.set_cantidad_producto_venta(index, cantidad, pos)?)
}
pub fn set_cliente_2(sistema: State<Mutex<Sistema>>, id: i64, pos: bool) -> Res<Venta> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    sis.set_cliente(id, pos)?;
    Ok(sis.venta(pos))
}
pub fn set_configs_2(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    configs: Config,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    match sis.arc_user().rango() {
        Rango::Admin => {
            sis.set_configs(configs);
            Ok(close_window_2(window)?)
        }
        Rango::Cajero => Err(DENEGADO.to_string()),
    }
}

pub fn stash_n_close_2(
    window: tauri::Window,
    sistema: State<Mutex<Sistema>>,
    pos: bool,
) -> Res<()> {
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
    Ok(close_window_2(window)?)
}
pub fn try_login_2(
    sistema: State<Mutex<Sistema>>,
    window: tauri::Window,
    id: &str,
    pass: &str,
) -> Res<()> {
    let mut sis = sistema.lock().map_err(|e| e.to_string())?;
    let rango = block_on(sis.try_login(id, get_hash(pass)))?;
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
    Ok(close_window_2(window)?)
}
pub fn unstash_sale_2(
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
