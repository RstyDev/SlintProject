#![allow(non_snake_case)]
use backend::{Presentacion, Producto, Valuable, Venta};
use dioxus::prelude::*;
use std::sync::{Arc,Mutex};
use backend::Sistema;
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
    
    let sis=Arc::clone(&sistema);

    rsx! {
        div{
            h1 { "style":"color: white", "Aca esta {count}" }
            h2 { "style":"color: white", "Desde sistema: {sis.read().lock().unwrap().venta(true):#?}"}
            //h3 { "{sistema}"}
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
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension"
            }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}
