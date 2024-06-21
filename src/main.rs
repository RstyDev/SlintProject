slint::include_modules!();
mod db;
mod mods;

fn set_logic(log: Logic){
    log.on_pagar(|a,b|{println!("{} {}",a,b); a+b});
    log.on_test(|dato|{println!("{dato}")});
}


fn main()->Result<(),slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    set_logic(ui.global::<Logic>());
    ui.on_request_increase_value(move || {
        let ui= ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    ui.run()
}

