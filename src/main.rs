use crate::db::db;
use crate::mods::cmd::*;
use crate::mods::Res;
use mods::Sistema;
use crate::mods::Valuable;
use crate::mods::Rubro;
use mods::Venta;
use crate::mods::Cliente;
use chrono::Utc;
use slint::{LogicalSize, ModelRc, SharedString, VecModel, WindowSize};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
slint::include_modules!();
mod db;
mod mods;
#[derive(Copy, Clone)]
enum Windows {
    Main,
    Login,
}
impl ToString for Windows {
    fn to_string(&self) -> String {
        match self {
            Windows::Main => String::from("Main"),
            Windows::Login => String::from("Login"),
        }
    }
}

fn set_logic(log: Logic, sistema: Arc<Mutex<Sistema>>, ui: Arc<App>, window: Windows) -> Res<()> {
    log.on_test(|dato| println!("{dato}"));
    match window {
        Windows::Main => {
            let mut ventas = match sistema.lock() {
                Ok(sis) => sis.ventas(),
                Err(_) => match sistema.lock() {
                    Ok(sis) => sis.ventas(),
                    Err(e) => panic!("{e}"),
                }
            };
            ventas.a = Venta::build(2, 154815.0, vec![Valuable::Rub((3,Rubro::build(1,154,Some(135.2),"Rubro")))], vec![], 35015.0, None, Cliente::Final, false, false, Utc::now().naive_local());
            println!("{:#?}",ventas.a);
            ui.set_ventas(ModelRc::new(VecModel::from(vec![
                ventas.a.to_fnd(),
                ventas.b.to_fnd(),
            ])));
            log.on_pagar(|a, b| {
                println!("{} {}", a, b); //TODO!
                a + b
            });
            let sis = sistema.clone();
            log.on_get_venta_actual(move |pos| {
                get_venta_actual(sis.clone(), pos).unwrap().to_fnd()
            });
        }
        Windows::Login => log.on_try_login(move |id, pass| {
            match try_login(sistema.clone(), id.as_str(), pass.as_str()) {
                Ok(_) => {
                    println!("{}", Windows::Main.to_string());
                    set_window_size_name(ui.clone(), Windows::Main, 800.0, 600.0, sistema.clone());
                    SharedString::from("Ok")
                }
                Err(e) => SharedString::from(format!("{e}")),
            }
        }),
    }
    Ok(())
}

fn set_window_size_name(
    ui: Arc<App>,
    window: Windows,
    width: f32,
    height: f32,
    sistema: Arc<Mutex<Sistema>>,
) {
    ui.window()
        .set_size(WindowSize::Logical(LogicalSize::new(width, height)));
    ui.set_window(SharedString::from(window.to_string()));
    match window {
        Windows::Login => {
            ui.window();
        }
        Windows::Main => (),
    }
    set_logic(ui.global::<Logic>(), sistema, ui.clone(), window);
}

fn main() -> Result<(), slint::PlatformError> {
    let read_db = Runtime::new().unwrap().block_on(async { db(false).await });
    let write_db = Runtime::new().unwrap().block_on(async { db(true).await });
    let sistema = Arc::from(Mutex::from(
        Sistema::new(Arc::from(read_db), Arc::from(write_db)).unwrap(),
    ));
    let ui = Arc::from(App::new()?);

    set_window_size_name(ui.clone(), Windows::Login, 300.0, 200.0, sistema.clone());
    ui.set_logged(sistema.lock().unwrap().get_logged_state());

    ui.run()
}
