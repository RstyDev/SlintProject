#[cfg(test)]
mod tests {
    use crate::mods::*;
    use crate::*;
    use tauri::{async_runtime, App, AppHandle, Manager, Window};
    const INCORRECT: &str = "Incorrect";
    use std::sync::Mutex;

    fn build(logged: bool) -> (App, Window, AppHandle) {
        let app = tauri::Builder::default()
            .manage(Mutex::new(Sistema::test(None).unwrap()))
            .any_thread()
            .menu(get_menu())
            .build(tauri::generate_context!())
            .unwrap();
        let win = app.get_window("main").unwrap();

        let handle = app.handle();
        if logged {
            try_login(app.state::<Mutex<Sistema>>(), win.clone(), "test", "9876").unwrap();
        }
        (app, win, handle)
    }

    #[test]
    fn not_open_login_test() {
        let (app, _, _) = build(false);
        assert!(app.get_window("login").is_none());
    }
    #[test]
    fn open_login_test() {
        let (app, _, _) = build(false);
        match Runtime::new().unwrap().block_on(async {open_login(app.handle()).await }) {
            Ok(_) => assert!(app.get_window("login").is_some()),
            Err(e) => panic!("No se abrio la ventana: {}", e),
        }
    }
    #[test]
    fn try_login_test() {
        build(true);
    }
    #[test]
    #[should_panic(expected = "Contraseña")]
    fn not_pass_try_login_test() {
        let (app, window, _) = build(false);
        try_login(app.state::<Mutex<Sistema>>(), window, "test", "6548").unwrap();
    }
    #[test]
    #[should_panic(expected = "Usuario")]
    fn not_user_try_login_test() {
        let (app, window, _) = build(false);
        try_login(app.state::<Mutex<Sistema>>(), window, "other", "9876").unwrap();
    }
    #[test]
    fn agregar_cliente_test() {
        let (app, window, _) = build(true);
        let nombre = "NombreCliente";
        let dni = "37846515";
        match agregar_cliente(app.state::<Mutex<Sistema>>(), window, nombre, dni, None) {
            Ok(a) => assert!(nombre == a.nombre() && dni.parse::<i32>().unwrap() == *a.dni()),
            Err(e) => panic!("{e}"),
        }
    }
    #[test]
    #[should_panic(expected = "existente")]
    fn agregar_cliente_existente_test() {
        let (app, window, _) = build(true);
        let nombre = "NombreCliente";
        let dni = "37846515";
        agregar_cliente(
            app.state::<Mutex<Sistema>>(),
            window.clone(),
            nombre,
            dni,
            None,
        )
        .unwrap();
        agregar_cliente(app.state::<Mutex<Sistema>>(), window, nombre, dni, None).unwrap();
    }
    #[test]
    #[should_panic(expected = "Sesión no iniciada")]
    fn not_logged_agregar_cliente_test() {
        let (app, window, _) = build(false);
        agregar_cliente(
            app.state::<Mutex<Sistema>>(),
            window,
            "nombre",
            "464511",
            None,
        )
        .unwrap();
    }
    #[test]
    fn get_clientes_test() {
        let (app, window, _) = build(true);
        let nombre = "NombreCliente";
        let dni = "37846515";
        let nombre2 = "Nombre2";
        let dni2 = "73222512";
        agregar_cliente(
            app.state::<Mutex<Sistema>>(),
            window.clone(),
            nombre,
            dni,
            None,
        )
        .unwrap();
        agregar_cliente(
            app.state::<Mutex<Sistema>>(),
            window,
            nombre2,
            dni2,
            Some("1000.0"),
        )
        .unwrap();
        let clientes = get_clientes(app.state::<Mutex<Sistema>>()).unwrap();
        assert!(clientes[0].nombre() == nombre && clientes[1].nombre() == nombre2);
    }
    #[test]
    #[should_panic(expected = "Sesión no iniciada")]
    fn not_logged_get_clientes() {
        let (app, _, _) = build(false);
        get_clientes(app.state::<Mutex<Sistema>>()).unwrap();
    }
    #[test]
    fn agregar_pesable_test() {
        let (app, window, _) = build(true);
        let desc = "PesablePrueba";
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            "1000",
            "1541546",
            "1400",
            "40",
            desc,
        )
        .unwrap();
        match get_productos_filtrado(app.state::<Mutex<Sistema>>(), desc) {
            Ok(res) => assert!(res.len() == 1 && res[0].desc() == desc),
            Err(e) => panic!("{e}"),
        }
    }
    #[test]
    #[should_panic(expected = "existente")]
    fn agregar_pesable_existente_test() {
        let (app, window, _) = build(true);
        let desc = "PesablePrueba";
        agregar_pesable(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            "1400",
            "1541546",
            "1000",
            "40",
            desc,
        )
        .unwrap();
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            "1000",
            "1541546",
            "1400",
            "40",
            desc,
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "invalid float")]
    fn not_precio_peso_agregar_pesable_test() {
        let (app, window, _) = build(true);
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            INCORRECT,
            "1651351",
            "1800",
            "30",
            "descripcion",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "invalid digit")]
    fn not_codigo_agregar_pesable_test() {
        let (app, window, _) = build(true);
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            "1450",
            INCORRECT,
            "1800",
            "30",
            "descripcion",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "invalid float")]
    fn not_costo_kilo_agregar_pesable_test() {
        let (app, window, _) = build(true);
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            "1458",
            "1651351",
            INCORRECT,
            "30",
            "descripcion",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "invalid float")]
    fn not_porcentaje_agregar_pesable_test() {
        let (app, window, _) = build(true);
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            "4810",
            "1651351",
            "1800",
            INCORRECT,
            "descripcion",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "Sesión no iniciada")]
    fn not_logged_agregar_pesable_test() {
        let (app, window, _) = build(false);
        agregar_pesable(
            window,
            app.state::<Mutex<Sistema>>(),
            "1240",
            "1651351",
            "1800",
            "30",
            "descripcion",
        )
        .unwrap();
    }
    #[test]
    fn agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window,
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
        match get_productos_filtrado(app.state::<Mutex<Sistema>>(), "tip mar var") {
            Ok(res) => assert!(
                res.len() == 1
                    && res[0].desc().contains("tipo")
                    && res[0].desc().contains("marca")
                    && res[0].desc().contains("variedad")
            ),
            Err(e) => panic!("{e}"),
        }
    }
    #[test]
    #[should_panic(expected = "existente")]
    fn agregar_producto_existente_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
        agregar_producto(
            window,
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "None")]
    fn not_logged_agregar_producto_test() {
        let (app, window, _) = build(false);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "ParseInt")]
    fn not_code_agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec![INCORRECT],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "flotante")]
    fn not_precio_de_venta_agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            INCORRECT,
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "flotante")]
    fn not_porcentaje_agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            INCORRECT,
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "flotante")]
    fn not_precio_costo_agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            INCORRECT,
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "ParseInt")]
    fn not_cantidad_agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            INCORRECT,
            "Un",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "No posible")]
    fn not_presentacion_agregar_producto_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "5",
            INCORRECT,
        )
        .unwrap();
    }
    #[test]
    fn get_productos_filtrado_test() {
        let (app, window, _) = build(true);
        let marca = "marca";
        let tipo = "tipo_prod";
        let variedad = "variedad";
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["6841651"],
            "1400",
            "40",
            "1000",
            tipo,
            marca,
            variedad,
            "5",
            "Un",
        )
        .unwrap();
        agregar_producto(
            window,
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "1400",
            "40",
            "1000",
            "type",
            "brand",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
        let res = get_productos_filtrado(app.state::<Mutex<Sistema>>(), "ti mar va ").unwrap();
        let prod = match &res[0] {
            V::Prod(p) => p.1.clone(),
            V::Pes(p) => panic!("Dio Pes {p:#?}"),
            V::Rub(r) => panic!("Dios Rub {r:#?}"),
        };
        assert!(
            res.len() == 1
                && prod.marca().as_ref() == marca
                && prod.tipo_producto().as_ref() == tipo
        );
    }
    #[test]
    fn agregar_producto_a_venta_test() {
        let (app, window, _) = build(true);
        let marca = "marca";
        let tipo = "tipo_prod";
        let variedad = "variedad";
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["6841651"],
            "1400",
            "40",
            "1000",
            tipo,
            marca,
            variedad,
            "5",
            "Un",
        )
        .unwrap();
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["51435613"],
            "2800",
            "40",
            "2000",
            "type",
            "brand",
            "variedad",
            "5",
            "Un",
        )
        .unwrap();
        let res = get_productos_filtrado(app.state::<Mutex<Sistema>>(), " ").unwrap();
        agregar_producto_a_venta(
            app.state::<Mutex<Sistema>>(),
            window.clone(),
            res[0].to_owned(),
            true,
        )
        .unwrap();
        agregar_producto_a_venta(
            app.state::<Mutex<Sistema>>(),
            window,
            res[1].to_owned(),
            true,
        )
        .unwrap();
        assert!(
            app.state::<Mutex<Sistema>>()
                .lock()
                .unwrap()
                .venta(true)
                .monto_total()
                == (4200.0)
        );
    }
    #[test]
    #[should_panic(expected = "No encontrado")]
    fn not_exist_agregar_producto_a_venta_test() {
        let (app, window, _) = build(true);
        agregar_producto_a_venta(
            app.state::<Mutex<Sistema>>(),
            window,
            V::Prod((
                0,
                Producto::new(
                    541651,
                    Vec::new(),
                    1400.0,
                    40.0,
                    1000.0,
                    "tipo",
                    "marca",
                    "variedad",
                    mods::Presentacion::Un(1),
                ),
            )),
            true,
        )
        .unwrap();
    }
    #[test]
    fn agregar_producto_a_venta_repetido_test() {
        let (app, window, _) = build(true);
        let marca = "marca";
        let tipo = "tipo_prod";
        let variedad = "variedad";
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["6841651"],
            "1400",
            "40",
            "1000",
            tipo,
            marca,
            variedad,
            "5",
            "Un",
        )
        .unwrap();
        let res = get_productos_filtrado(app.state::<Mutex<Sistema>>(), " ").unwrap();
        agregar_producto_a_venta(
            app.state::<Mutex<Sistema>>(),
            window.clone(),
            res[0].to_owned(),
            true,
        )
        .unwrap();
        agregar_producto_a_venta(
            app.state::<Mutex<Sistema>>(),
            window,
            res[0].to_owned(),
            true,
        )
        .unwrap();
        assert!(
            app.state::<Mutex<Sistema>>()
                .lock()
                .unwrap()
                .venta(true)
                .monto_total()
                == (2800.0)
        );
    }
    #[test]
    fn agregar_rubro_test() {
        let (app, window, _) = build(true);
        let desc = "Rubro";
        agregar_rubro(window, app.state::<Mutex<Sistema>>(), "6441", desc).unwrap();
        let res = get_productos_filtrado(app.state::<Mutex<Sistema>>(), "rub").unwrap();
        let rub = match &res[0] {
            V::Prod(_) => panic!("Dio Prod"),
            V::Pes(_) => panic!("Dio Pes"),
            V::Rub(rub) => rub.1.clone(),
        };
        assert!(res.len() == 1 && rub.desc() == desc)
    }
    #[test]
    #[should_panic(expected = "existente")]
    fn agregar_rubro_repetido_test() {
        let (app, window, _) = build(true);
        let cod = "1465";
        agregar_rubro(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            cod,
            "UnaDesc",
        )
        .unwrap();
        agregar_rubro(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            cod,
            "OtraDesc",
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "invalid digit")]
    fn not_code_agregar_rubro_test() {
        let (app, window, _) = build(true);
        agregar_rubro(window, app.state::<Mutex<Sistema>>(), INCORRECT, "Desc").unwrap();
    }
    #[test]
    #[should_panic(expected = "None")]
    fn not_logged_agregar_rubro_test() {
        let (app, window, _) = build(false);
        agregar_rubro(window, app.state::<Mutex<Sistema>>(), "5641", "Desc").unwrap();
    }
    #[test]
    fn agregar_proveedor_sin_contacto_test() {
        let (app, window, _) = build(true);
        let prov = "EjemploProv";
        agregar_proveedor(window, app.state::<Mutex<Sistema>>(), prov, None).unwrap();
        let provs =
            Runtime::new().unwrap().block_on(async {app.state::<Mutex<Sistema>>().lock().unwrap().proveedores().await })
                .clone();
        assert!(
            provs.len() == 1 && provs[0].nombre().as_ref() == prov && provs[0].contacto().is_none()
        );
    }
    #[test]
    fn agregar_proveedor_con_contacto_test() {
        let (app, window, _) = build(true);
        let prov = "EjemploProv2";
        let cont = "54161";
        agregar_proveedor(window, app.state::<Mutex<Sistema>>(), prov, Some(cont)).unwrap();
        let provs =
            Runtime::new().unwrap().block_on(async {app.state::<Mutex<Sistema>>().lock().unwrap().proveedores().await })
                .clone();
        assert!(
            provs.len() == 1
                && provs[0].nombre().as_ref() == prov
                && provs[0].contacto().unwrap() == cont.parse::<i64>().unwrap()
        )
    }
    #[test]
    #[should_panic(expected = "invalid digit")]
    fn not_contacto_agregar_proveedor_test() {
        let (app, window, _) = build(true);
        agregar_proveedor(
            window,
            app.state::<Mutex<Sistema>>(),
            "EjemploProv",
            Some(INCORRECT),
        )
        .unwrap();
    }
    #[test]
    #[should_panic(expected = "None")]
    fn not_logged_agregar_proveedor_test() {
        let (app, window, _) = build(false);
        agregar_proveedor(window, app.state::<Mutex<Sistema>>(), "Prov", Some("64")).unwrap();
    }
    #[test]
    fn agregar_pago_test() {
        let (app, window, _) = build(true);
        agregar_producto(
            window.clone(),
            app.state::<Mutex<Sistema>>(),
            Vec::new(),
            Vec::new(),
            vec!["658651"],
            "1400",
            "40",
            "1000",
            "tipo_producto",
            "marca",
            "variedad",
            "1",
            "Un",
        )
        .unwrap();
        let prod = get_productos_filtrado(app.state::<Mutex<Sistema>>(), "").unwrap()[0].clone();
        agregar_producto_a_venta(app.state::<Mutex<Sistema>>(), window.clone(), prod, true)
            .unwrap();
        let pagos = agregar_pago(
            window,
            app.state::<Mutex<Sistema>>(),
            "Efectivo",
            "1000",
            true,
        )
        .unwrap();
        assert!(pagos.len() == 1 && pagos[0].monto() == 1000.0);
    }
}
