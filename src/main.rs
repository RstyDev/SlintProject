#![allow(non_snake_case)]
use backend::MedioPago;
//use backend::{Presentacion, Producto, Valuable, Venta};
use dioxus::prelude::*;
use dioxus::hooks::use_callback;
use std::borrow::BorrowMut;
use std::sync::{Arc,Mutex};
use backend::{Sistema,Pago as BPago};
use frontend::Pago;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use tracing::Level;
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    use_context_provider(||(Sistema::new().unwrap()));
    let algo=use_context::<Sistema>();
    algo.agregar_cliente("nombre", 65451, true, 1500.0, write_db)
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}
pub async fn get_db(path: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(path).await
}
#[component]
fn App() -> Element {
    // Build cool things ✌️
    let algo=use_signal(use_context::<Sistema>);
    
    let mut count: Signal<i32> = use_signal(|| 0);
    //let mut venta = use_signal(Venta::default);
    let sistema= Arc::new(use_signal(||Mutex::new(Sistema::new().unwrap())));
    let pago=BPago::new(MedioPago::new("Ef", 0), 0.0, None);
    let sis=Arc::clone(&sistema);
    rsx! {
        div{
            h1 {  "Aca esta {count}" }
            h2 {  "Desde sistema: {sis.read().lock().unwrap().venta(true):#?}"}
            h2 { "Aca el pago: {pago:#?}" }
            
            Pago{ pago:Some(BPago::new(MedioPago::new("Ef", 0), 0.0, None))}
            Pago{pago:None}
            button { onclick: move |_| {count += 1;
                let sist= Arc::clone(&sistema);
            }, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        Router::<Route> {}
    }
}


#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}