slint::include_modules!();
mod db;
mod mods;

fn main()->Result<(),slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui= ui_handle.unwrap()
    })
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}