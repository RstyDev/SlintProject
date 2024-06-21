slint::include_modules!();
mod db;
mod mods;

fn main()->Result<(),slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}