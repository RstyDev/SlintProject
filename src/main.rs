#![allow(non_snake_case)]
use backend::MedioPago;
use backend::{Presentacion, Producto, Valuable, Venta};
use dioxus::prelude::*;
use std::sync::{Arc,Mutex};
use backend::{Sistema,Pago as BPago};
use frontend::Pago;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}
pub async fn get_db(path: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(path).await
}
#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut count: Signal<i32> = use_signal(|| 0);
    //let mut venta = use_signal(Venta::default);
    let sistema= Arc::new(use_signal(||Mutex::new(Sistema::new().unwrap())));
    let pago=BPago::new(MedioPago::new("Ef", 0), 0.0, None);
    let sis=Arc::clone(&sistema);
    let propsis=Arc::clone(&sistema);
    let propsis2=Arc::clone(&sistema);
    rsx! {
        div{
            h1 {  "Aca esta {count}" }
            h2 {  "Desde sistema: {sis.read().lock().unwrap().venta(true):#?}"}
            h2 { "Aca el pago: {pago:#?}" }
            
            Pago{ pago:Some(BPago::new(MedioPago::new("Ef", 0), 0.0, None)),sistema:propsis}
            Pago{pago:None,sistema:propsis2}
            button { onclick: move |_| {count += 1;
                let sist= Arc::clone(&sistema);
                spawn(async move{
                    sist.read().lock().unwrap().agregar_producto_a_venta(Valuable::Prod((0,Producto::new(
                        1, vec![5641], 1400.0, 40.0, 1000.0, "tipo", "marca", "variedad", Presentacion::Un(1)))), true).await.unwrap();
                });
            }, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        
    }
}
