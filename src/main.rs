use mods::Sistema;
use crate::mods::cmd::*;
use crate::db::db;
use std::sync::{Arc,Mutex};
use tokio::runtime::Runtime;
slint::include_modules!();
mod db;
mod mods;

fn set_logic(log: Logic,sistema: Arc<Mutex<Sistema>>){
    log.on_pagar(|a,b|{println!("{} {}",a,b); a+b});
    log.on_test(|dato|{println!("{dato}")});
    log.on_get_venta_actual(move |pos| {
        let venta=get_venta_actual(&sistema,pos).unwrap();
        let mut st=VentaSt::default();
        
        st
    });
}


fn main()->Result<(),slint::PlatformError> {
    let read_db = Runtime::new().unwrap().block_on(async {db(false).await});
    let write_db = Runtime::new().unwrap().block_on(async {db(true).await});
    let sistema=Arc::from(Mutex::from(Sistema::new(Arc::from(read_db), Arc::from(write_db)).unwrap()));
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    set_logic(ui.global::<Logic>(),Arc::clone(&sistema));
    ui.on_request_increase_value(move || {
        let ui= ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    ui.run()
}

