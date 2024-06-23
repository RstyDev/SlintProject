use crate::db::db;
use crate::mods::cmd::*;
use mods::Sistema;
use slint::{LogicalSize, SharedString, WindowSize};
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
            Windows::Main => String::from("Windows"),
            Windows::Login => String::from("Login"),
        }
    }
}

fn set_logic(log: Logic, sistema: Arc<Mutex<Sistema>>, ui: Arc<App>, window: Windows) {
    log.on_test(|dato| println!("{dato}"));
    match window {
        Windows::Main => {
            log.on_pagar(|a, b| {
                println!("{} {}", a, b); //TODO!
                a + b
            });
            let sis = sistema.clone();
            log.on_get_venta_actual(move |pos| get_venta_actual(sis.clone(), pos).unwrap().to_st());
        }
        Windows::Login => log.on_try_login(move |id, pass| {
            match try_login(sistema.clone(), id.as_str(), pass.as_str()) {
                Ok(_) => {
                    set_window_size_name(ui.clone(), Windows::Main, 800.0, 600.0, sistema.clone());
                    SharedString::from("Ok")
                }
                Err(e) => SharedString::from(format!("{e}")),
            }
        }),
    }
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
