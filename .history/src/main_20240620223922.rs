slint::include_modules!();
mod db;
mod mods;

fn main()->Result<(),slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.
    ui.on_request_increase_value(move || {
        let ui= ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    ui.run()
}

