mod db;
mod mods;
use crate::{
    db::db,
    mods::{cmd::*, Cliente, Pesable, Res, Rubro, Sistema, Valuable, Venta},
};
use chrono::Utc;
use slint::{LogicalSize, ModelRc, SharedString, VecModel, WindowSize};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
slint::include_modules!();
#[derive(Copy, Clone)]
enum Windows {
    Main,
    Login,
}
impl Display for Windows {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Windows::Login => "Login",
                Windows::Main => "Main",
            }
        )
    }
}

fn set_logic(log: Logic, sistema: Arc<Mutex<Sistema>>, ui: Arc<App>, window: Windows) -> Res<()> {
    match window {
        Windows::Main => {
            let mut ventas = match sistema.lock() {
                Ok(sis) => sis.ventas(),
                Err(_) => match sistema.lock() {
                    Ok(sis) => sis.ventas(),
                    Err(e) => panic!("{}", e),
                },
            };
            ventas.a = Venta::build(
                2,
                154815.0,
                vec![
                    Valuable::Rub((3, Rubro::build(1, 154, Some(135.2), "Rubro"))),
                    Valuable::Pes((
                        1.1,
                        Pesable::build(1, 1, 100.0, 40.0, 140.0, "Pan Flautita"),
                    )),
                ],
                vec![],
                35015.0,
                None,
                Cliente::Final,
                false,
                false,
                Utc::now().naive_local(),
            );
            println!("{:#?}", ventas.a);
            ui.set_ventas(ModelRc::new(VecModel::from(vec![
                ventas.a.to_fnd(),
                ventas.b.to_fnd(),
            ])));
        }
        Windows::Login => (),
    }
    let sis = sistema.clone();
    log.on_get_venta_actual(move |pos| get_venta_actual(sis.clone(), pos).unwrap().to_fnd());
    log.on_pagar(|a, b| {
        println!("{} {}", a, b); //TODO!
        a + b
    });
    log.on_test(|dato| println!("{dato}"));
    log.on_try_login(move |id, pass| {
        match try_login(sistema.clone(), id.as_str(), pass.as_str()) {
            Ok(_) => {
                println!("{}", Windows::Main.to_string());
                set_window_size_name(
                    ui.clone(),
                    Windows::Main,
                    800.0,
                    600.0, /*, sistema.clone()*/
                );
                SharedString::from("Ok")
            }
            Err(e) => SharedString::from(format!("{e}")),
        }
    });
    Ok(())
}

fn set_window_size_name(
    ui: Arc<App>,
    window: Windows,
    width: f32,
    height: f32,
    //sistema: Arc<Mutex<Sistema>>,
) {
    ui.window()
        .set_size(WindowSize::Logical(LogicalSize::new(width, height)));
    ui.set_window(SharedString::from(window.to_string()));
    match window {
        Windows::Login => {
            ui.window();
        }
        Windows::Main => (),
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let read_db = Runtime::new().unwrap().block_on(async { db(false).await });
    let write_db = Runtime::new().unwrap().block_on(async { db(true).await });
    let sistema = Arc::from(Mutex::from(
        Sistema::new(Arc::from(read_db), Arc::from(write_db)).unwrap(),
    ));
    let ui = Arc::from(App::new()?);
    if set_logic(
        ui.global::<Logic>(),
        sistema.clone(),
        ui.clone(),
        Windows::Login,
    )
    .is_err()
    {
        set_logic(
            ui.global::<Logic>(),
            sistema.clone(),
            ui.clone(),
            Windows::Login,
        )
        .unwrap();
    }
    set_window_size_name(
        ui.clone(),
        Windows::Login,
        300.0,
        200.0,
        //   sistema.clone()
    );
    ui.set_logged(sistema.lock().unwrap().get_logged_state());

    ui.run()
}
