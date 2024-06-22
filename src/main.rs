use crate::db::db;
use crate::mods::cmd::*;
use mods::Sistema;
use slint::{SharedString, WindowSize,LogicalSize};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
slint::include_modules!();
mod db;
mod mods;

fn set_logic(log: Logic, sistema: Arc<Mutex<Sistema>>) {
    log.on_pagar(|a, b| {
        println!("{} {}", a, b);
        a + b
    });
    log.on_test(|dato| println!("{dato}"));
    log.on_get_venta_actual(move |pos| get_venta_actual(&sistema, pos).unwrap().to_st());
    log.on_try_login(move|id,pass|);
}

fn set_window_size_name(ui:&AppWindow,window: &str,width:f32,height:f32){
    ui.set_window(SharedString::from(window));
    ui.window().set_size(WindowSize::Logical(LogicalSize::new(width,height)));
}

fn main() -> Result<(), slint::PlatformError> {
    let read_db = Runtime::new().unwrap().block_on(async { db(false).await });
    let write_db = Runtime::new().unwrap().block_on(async { db(true).await });
    let sistema = Arc::from(Mutex::from(
        Sistema::new(Arc::from(read_db), Arc::from(write_db)).unwrap(),
    ));
    let ui = AppWindow::new()?;

    set_window_size_name(&ui,"login",300.0,200.0);
    let ui_handle = ui.as_weak();
    ui.set_logged(sistema.lock().unwrap().get_logged_state());
    set_logic(ui.global::<Logic>(), Arc::clone(&sistema));
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    ui.run()
}
