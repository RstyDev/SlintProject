mod app;
mod components;
use components::pago::Pago;
use app::*;
use leptos::*;

fn main() {
    let (options,_set_options)=create_signal(vec!["Efectivo".to_string(),"Crédito".to_string(),"Débito".to_string()]);
    mount_to_body(move || {
        view! { 
            <App/>
            <Pago options=&options/>
        }
    })
}
