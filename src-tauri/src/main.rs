// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let menu = get_menu();
    let app = tauri::Builder::default()
        .manage(Mutex::new(Sistema::new().unwrap()))
        .invoke_handler(tauri::generate_handler![
            agregar_cliente,
            agregar_pago,
            agregar_pesable,
            agregar_producto,
            agregar_producto_a_venta,
            agregar_proveedor,
            agregar_rubro,
            agregar_rub_o_pes_a_venta,
            agregar_usuario,
            buscador,
            cancelar_venta,
            cerrar_caja,
            cerrar_sesion,
            check_codes,
            close_window,
            descontar_producto_de_venta,
            editar_producto,
            eliminar_pago,
            eliminar_producto,
            eliminar_producto_de_venta,
            eliminar_usuario,
            get_caja,
            get_clientes,
            get_configs,
            get_descripciones,
            get_descripcion_valuable,
            get_deuda,
            get_deuda_detalle,
            get_filtrado,
            get_log_state,
            get_medios_pago,
            get_productos_filtrado,
            get_proveedores,
            get_rango,
            get_stash,
            get_user,
            get_venta_actual,
            hacer_egreso,
            hacer_ingreso,
            incrementar_producto_a_venta,
            open_add_prov,
            open_add_product,
            open_add_user,
            open_add_cliente,
            open_cancelar_venta,
            open_cerrar_caja,
            open_confirm_stash,
            open_edit_settings,
            open_login,
            open_select_amount,
            open_stash,
            pagar_deuda_especifica,
            pagar_deuda_general,
            try_login,
            set_cantidad_producto_venta,
            set_cliente,
            set_configs,
            stash_n_close,
            unstash_sale,
        ])
        .menu(menu)
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    let window = app.get_window("main").unwrap();
    let w2 = window.clone();
    let handle = app.handle();
    window.on_menu_event(move |event| {
        match event.menu_item_id() {
            "add product" => async_runtime::block_on(open_add_product(handle.clone())),
            "add prov" => async_runtime::block_on(open_add_prov(handle.clone())),
            "add user" => async_runtime::block_on(open_add_user(handle.clone())),
            "add cliente" => async_runtime::block_on(open_add_cliente(handle.clone())),
            "edit settings" => async_runtime::block_on(open_edit_settings(handle.clone())),
            "confirm stash" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload {
                                message: Some(String::from("confirm stash")),
                                pos: None,
                                val: None,
                            },
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }
            "cerrar sesion" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload {
                                message: Some(String::from("cerrar sesion")),
                                pos: None,
                                val: None,
                            },
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }

            "open stash" => {
                loop {
                    if w2
                        .emit(
                            "main",
                            Payload {
                                message: Some(String::from("open stash")),
                                pos: None,
                                val: None,
                            },
                        )
                        .is_ok()
                    {
                        break;
                    }
                }
                Ok(())
            }
            "cerrar caja" => async_runtime::block_on(open_cerrar_caja(handle.clone())),

            _ => Ok(()),
        }
        .unwrap();
    });
    app.run(|_, _| {})
}
