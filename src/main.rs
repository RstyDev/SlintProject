mod mods;
use crate::mods::app;
use crate::mods::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
