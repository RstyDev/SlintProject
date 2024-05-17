#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::Level;
use backend::Sistema;
use std::sync::{Arc,Mutex};
use backend::agregar_cliente_2;
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
async fn App() -> Element {
    // Build cool things ✌️
    let mut count: Signal<i32> = use_signal(|| 0);
    let mut sistema = use_signal(|| Sistema::new().unwrap());
    let algo= use_future(cx,(),|_|async move{
        agregar_cliente_2(sistema.into(),"Nombre","3641641",None).await.unwrap();
    })
    
    rsx! {
        div{
            h1 { "style":"color: white", "Aca esta {count}" }
            h2 { "style":"color: white", "Desde sistema: {algo:#?}"}
            //h3 { "{sistema}"}
            button { onclick: move |_| count += 1, "Up high!" }
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
