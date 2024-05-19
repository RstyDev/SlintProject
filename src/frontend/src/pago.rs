#[allow(non_snake_case)]
use backend::{Pago,Sistema};
use std::{borrow::{Borrow, BorrowMut}, ops::Deref, sync::{Arc,Mutex}};
use dioxus::{core_macro::Props, prelude::{dioxus_core, dioxus_elements, rsx, Element}, signals::Signal};
#[derive(Props,PartialEq,Clone)]
pub struct Prop{
    pago:Option<Pago>,
    //sistema: Arc<Mutex<Sistema>>,
}

pub fn Pago(props: Prop)->Element{
    let pagado=props.pago.is_some();
   
    rsx!{
        form{
            "class":"pago",
            input{
                "type":"number"
            }
            select{
                "disabled": pagado,
                "Seleccione",
            }
            button{
                "Pagar"
            }
        }
    }
}