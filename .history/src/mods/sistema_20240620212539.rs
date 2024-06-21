use super::{
    crear_file, get_hash, leer_file, AppError, Caja, Cli, Config, Db, Movimiento, Pago, Pesable,
    Presentacion, Producto, Proveedor, Rango, RelacionProdProv, Res, Rubro, User, Valuable, Venta,
};
use crate::db::fresh;
use crate::db::map::{
    BigIntDB, ClienteDB, CodeDB, CodedPesDB, CodedRubDB, PesableDB, ProductoDB, ProvDB, RubroDB,
    StringDB, UserDB,
};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use tokio::runtime::Runtime;
use std::collections::HashSet;
use std::sync::Arc;
use Valuable as V;

const CUENTA: &str = "Cuenta Corriente";
pub struct Sistema {
    user: Option<Arc<User>>,
    write_db: Arc<Pool<Sqlite>>,
    read_db: Arc<Pool<Sqlite>>,
    caja: Caja,
    configs: Config,
    ventas: Ventas,
    proveedores: Vec<Proveedor>,
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}

pub struct Ventas {
    pub a: Venta,
    pub b: Venta,
}

impl<'a> Sistema {
    pub fn access(&self) {
        if self.user.is_none() {
            panic!("Sesión no iniciada");
        }
    }
    pub fn agregar_cliente(
        &self,
        nombre: &str,
        dni: i64,
        activo: bool,
        limite: Option<f32>,
    ) -> Res<Cli> {
        Runtime::new().unwrap().block_on(async {
            Cli::new_to_db(
            self.write_db(),
            nombre,
            dni,
            activo,
            Utc::now().naive_local(),
            limite,
        )})
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f32, pos: bool) -> Res<f32> {
        let res;
        if pos {
            if !medio_pago.eq("Efectivo")
                && self.ventas.a.monto_pagado() + monto > self.ventas.a.monto_total()
            {
                return Err(AppError::AmountError {
                    a_pagar: self.ventas.a.monto_total() - self.ventas.a.monto_pagado(),
                    pagado: monto,
                });
            } else {
                res = self
                    .ventas
                    .a
                    .agregar_pago(medio_pago, monto, &self.write_db);
            }
        } else {
            if !medio_pago.eq("Efectivo")
                && self.ventas.b.monto_pagado() + monto > self.ventas.b.monto_total()
            {
                return Err(AppError::AmountError {
                    a_pagar: self.ventas.b.monto_total() - self.ventas.b.monto_pagado(),
                    pagado: monto,
                });
            } else {
                res = self
                    .ventas
                    .b
                    .agregar_pago(medio_pago, monto, &self.write_db);
            }
        }
        println!("{:#?}", res);
        if let Ok(a) = res {
            if a <= 0.0 {
                self.cerrar_venta(pos)?
            }
        }
        println!("Aca esta la caja {:#?} -----****", self.caja);
        res
    }
    pub fn agregar_usuario(&self, id: &str, nombre: &str, pass: &str, rango: &str) -> Res<User> {
        Runtime::new().unwrap().block_on(async {User::new_to_db(
            Arc::from(id),
            Arc::from(nombre),
            get_hash(pass),
            rango,
            self.write_db(),
        ).await})
    }
    #[cfg(test)]
    pub fn test(
        user: Option<Arc<User>>,
        read_db: Arc<Pool<Sqlite>>,
        write_db: Arc<Pool<Sqlite>>,
    ) -> Res<Sistema> {
        let w1 = Arc::clone(&write_db);
        Runtime::new().unwrap().block_on(async{fresh(w1.as_ref()).await});
        let configs = Runtime::new().unwrap().block_on(async{Config::get_or_def(&write_db.as_ref()).await}).unwrap();
        let caja = Runtime::new().unwrap().block_on(async {Caja::new(&write_db.as_ref(), Some(0.0), &configs).await})?;
        let w2 = Arc::clone(&write_db);
        let w3 = Arc::clone(&write_db);
        let r2 = Arc::clone(&read_db);
        let sis = Sistema {
            user,
            write_db,
            read_db,
            caja,
            configs,
            ventas: Ventas {
                a: Runtime::new().unwrap().block_on(async {Venta::get_or_new(None, w2.as_ref(), true).await})?,
                b: Runtime::new().unwrap().block_on(async {Venta::get_or_new(None, w3.as_ref(), false).await})?,
            },
            proveedores: Vec::new(),
            relaciones: Vec::new(),
            stash: Vec::new(),
            registro: Vec::new(),
        };
        Runtime::new().unwrap().block_on(async {Sistema::procesar_test(Arc::clone(&w2), r2)})?;
        Ok(sis)
    }
    pub fn new(read_db: Arc<Pool<Sqlite>>, write_db: Arc<Pool<Sqlite>>) -> Res<Sistema> {
        Runtime::new().unwrap().block_on(async {
            let qres: Option<BigIntDB> =
                sqlx::query_as!(BigIntDB, "select id as int from cajas limit 1")
                    .fetch_optional(read_db.as_ref())
                    .await
                    .unwrap();
            if qres.is_none() {
                fresh(write_db.as_ref()).await
            }
        });
        let path_proveedores = "Proveedores.json";
        let path_relaciones = "Relaciones.json";
        let mut relaciones = Vec::new();
        leer_file(&mut relaciones, path_relaciones)?;
        let mut proveedores: Vec<Proveedor> = Vec::new();
        leer_file(&mut proveedores, path_proveedores)?;

        let aux = Arc::clone(&write_db);
        let db = Arc::clone(&write_db);
        let configs = Runtime::new().unwrap().block_on(Config::get_or_def(db.as_ref()))?;
        let caja = Runtime::new().unwrap().block_on(Caja::new(aux.as_ref(), Some(0.0), &configs))?;
        let stash = Vec::new();
        let registro = Vec::new();

        println!("{:#?}", caja);
        let w1 = Arc::clone(&write_db);
        let sis = Sistema {
            user: None,
            write_db,
            read_db,
            caja,
            configs,
            ventas: Ventas {
                a: Runtime::new().unwrap().block_on(Venta::get_or_new(None, w1.as_ref(), true))?,
                b: Runtime::new().unwrap().block_on(Venta::get_or_new(None, w1.as_ref(), false))?,
            },
            proveedores: proveedores.clone(),
            relaciones,
            stash,
            registro,
        };
        Runtime::new().unwrap().block_on(Sistema::procesar(
            Arc::clone(&sis.write_db),
            Arc::clone(&sis.read_db),
            sis.proveedores.clone(),
            sis.relaciones.clone(),
        ))?;
        Ok(sis)
    }
    fn generar_reporte_caja(&self) {
        println!("{:#?}", self.caja);
        println!("Faltante");
    }
    pub fn user(&self) -> Option<Arc<User>> {
        match &self.user {
            Some(a) => Some(Arc::clone(a)),
            None => None,
        }
    }

    pub fn cancelar_venta(&mut self, pos: bool) -> Res<()> {
        if pos {
            self.ventas.a.empty();
        } else {
            self.ventas.b.empty();
        }
        Ok(())
    }
    pub fn cerrar_caja(&mut self, monto_actual: f32) -> Res<()> {
        self.caja.set_cajero(self.user().unwrap().nombre());
        let db = Arc::clone(&self.write_db);
        Runtime::new().unwrap().block_on(self.caja.set_n_save(db.as_ref(), monto_actual))?;
        self.generar_reporte_caja();
        self.caja = Runtime::new().unwrap().block_on(Caja::new(
            self.write_db.as_ref(),
            Some(monto_actual),
            &self.configs,
        ))?;
        Ok(())
    }
    pub fn eliminar_usuario(&self, user: User) -> Res<()> {
        Runtime::new().unwrap().block_on(Db::eliminar_usuario(user, self.read_db.as_ref()))?;
        Ok(())
    }

    pub fn caja(&self) -> &Caja {
        &self.caja
    }
    #[cfg(test)]
    async fn procesar_test(write_db: Arc<Pool<Sqlite>>, read_db: Arc<Pool<Sqlite>>) -> Res<()> {
        use tokio::spawn;

        let read_db2 = Arc::clone(&read_db);
        let _ = spawn(async move {
            let medios = [CUENTA, "Efectivo", "Crédito", "Débito"];
            for i in 0..medios.len() {
                sqlx::query("insert into medios_pago values (?, ?)")
                    .bind(i as i64)
                    .bind(medios[i])
                    .execute(read_db.as_ref())
                    .await?;
            }
            return Ok(());
        });
        let qres: Option<BigIntDB> =
            sqlx::query_as!(BigIntDB, "select id as int from users limit 1")
                .fetch_optional(read_db2.as_ref())
                .await?;
        if qres.is_none() {
            sqlx::query("insert into users values (?, ?, ?, ?)")
                .bind("test")
                .bind(get_hash("9876"))
                .bind(Rango::Admin.to_string())
                .bind("Admin")
                .execute(write_db.as_ref())
                .await?;
        }
        Ok(())
    }
    async fn procesar(
        write_db: Arc<Pool<Sqlite>>,
        read_db: Arc<Pool<Sqlite>>,
        proveedores: Vec<Proveedor>,
        relaciones: Vec<RelacionProdProv>,
    ) -> Res<()> {
        let path_productos = "Productos.json";
        let path_configs = "Configs.json";
        let path_pesables = "Pesables.json";
        let mut configs = Vec::<Config>::new();
        leer_file(&mut configs, path_configs)?;
        if configs.len() == 0 {
            configs.push(Config::default());
            crear_file(path_configs, &mut configs)?;
        }
        let mut productos: Vec<Producto> = Vec::new();
        let mut rubros: Vec<Rubro> = Vec::new();
        let path_rubros = "Rubros.json";
        let mut pesables: Vec<Pesable> = Vec::new();

        leer_file(&mut rubros, path_rubros)?;
        leer_file(&mut pesables, path_pesables)?;
        leer_file(&mut productos, path_productos)?;

        let mut rubros_valuable: Vec<Valuable> =
            rubros.iter().map(|a| V::Rub((0, a.to_owned()))).collect();
        let mut pesables_valuable: Vec<Valuable> = pesables
            .iter()
            .map(|a| V::Pes((0.0, a.to_owned())))
            .collect();
        let mut valuables: Vec<Valuable> = productos
            .clone()
            .iter()
            .map(|a| V::Prod((0, a.to_owned())))
            .collect();
        valuables.append(&mut pesables_valuable);
        valuables.append(&mut rubros_valuable);
        let write_db2 = Arc::clone(&write_db);
        let read_db2 = Arc::clone(&read_db);
        let _: JoinHandle<Result<(), AppError>> = Runtime::new().unwrap().spawn(async move {
            let medios = [CUENTA, "Efectivo", "Crédito", "Débito"];
            for i in 0..medios.len() {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select id as int from medios_pago where medio = ? limit 1",
                    medios[i]
                )
                .fetch_optional(read_db.as_ref())
                .await?;
                if qres.is_none() {
                    sqlx::query("insert into medios_pago values (?, ?)")
                        .bind(i as i64)
                        .bind(medios[i])
                        .execute(write_db.as_ref())
                        .await?;
                }
            }
            return Ok(());
        });
        let qres: Option<BigIntDB> =
            sqlx::query_as!(BigIntDB, "select id as int from users limit 1")
                .fetch_optional(read_db2.as_ref())
                .await?;
        if qres.is_none() {
            sqlx::query("insert into users values (?, ?, ?, ?)")
                .bind("admin")
                .bind(get_hash("1234"))
                .bind(Rango::Admin.to_string())
                .bind("Admin")
                .execute(write_db2.as_ref())
                .await?;
            Db::cargar_todos_los_valuables(valuables, write_db2.as_ref()).await?;
            Db::cargar_todos_los_provs(proveedores, write_db2.as_ref()).await?;
            Db::cargar_todas_las_relaciones_prod_prov(relaciones, write_db2.as_ref()).await?;
        }
        Ok(())
    }

    pub async fn get_clientes(&self) -> Res<Vec<Cli>> {
        let qres: Vec<ClienteDB> = sqlx::query_as!(ClienteDB, r#"select id, nombre, dni as "dni:_", limite as "limite:_", activo, time from clientes "#)
            .fetch_all(self.read_db())
            .await?;
        Ok(qres
            .iter()
            .map(|cli| {
                Cli::build(
                    cli.id,
                    Arc::from(cli.nombre.to_owned()),
                    cli.dni,
                    cli.activo,
                    cli.time,
                    cli.limite,
                )
            })
            .collect::<Vec<Cli>>())
    }
    pub async fn try_login(&mut self, id: &str, pass: i64) -> Res<Rango> {
        let qres: Option<UserDB> = sqlx::query_as!(
            UserDB,
            "select * from users where user_id = ? and pass = ? limit 1",
            id,
            pass
        )
        .fetch_optional(self.read_db())
        .await?;
        match qres {
            None => {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select id as int from users where user_id = ?",
                    id
                )
                .fetch_optional(self.read_db.as_ref())
                .await?;
                match qres {
                    Some(_) => Err(AppError::IncorrectError("Contraseña".to_string())),
                    None => Err(AppError::IncorrectError("Usuario".to_string())),
                }
            }
            Some(user) => {
                self.user = Some(Arc::from(User::build(
                    Arc::from(user.user_id),
                    Arc::from(user.nombre),
                    pass,
                    user.rango.as_ref(),
                )));
                self.ventas = Ventas {
                    a: Venta::get_or_new(Some(self.arc_user()), &self.write_db, true).await?,
                    b: Venta::get_or_new(Some(self.arc_user()), &self.write_db, false).await?,
                };
                Ok(self.user().unwrap().rango().clone())
            }
        }
    }
    pub async fn val_filtrado(
        &self,
        filtro: &str,
        db: &Pool<Sqlite>,
    ) -> Result<Vec<Valuable>, AppError> {
        let mut res: Vec<Valuable> = Vec::new();
        match filtro.parse::<i64>() {
            Ok(code) => {
                let qres: Option<CodeDB> =
                    sqlx::query_as!(CodeDB, "select * from codigos where codigo = ?", code)
                        .fetch_optional(db)
                        .await?;
                match qres {
                    None => return Ok(res),
                    Some(code) => {
                        if let Some(prod) = code.producto {
                            let prod: ProductoDB = sqlx::query_as!(
                                ProductoDB,
                                r#"select id, precio_venta as "precio_venta:_", porcentaje as "porcentaje:_", precio_costo as "precio_costo:_", tipo, marca, variedad, presentacion, size as "size:_", updated_at from productos where id = ?"#,
                                prod
                            )
                            .fetch_one(db)
                            .await?;
                            res.push(V::Prod((
                                0,
                                Producto::build(
                                    prod.id,
                                    vec![code.codigo],
                                    prod.precio_venta,
                                    prod.porcentaje,
                                    prod.precio_costo,
                                    prod.tipo.as_str(),
                                    prod.marca.as_str(),
                                    prod.variedad.as_str(),
                                    Presentacion::build(prod.presentacion.as_str(), prod.size),
                                ),
                            )))
                        } else if let Some(pes) = code.pesable {
                            let pes: PesableDB = sqlx::query_as!(
                                PesableDB,
                                r#"select id, precio_peso as "precio_peso:_", porcentaje as "porcentaje:_", costo_kilo as "costo_kilo:_", descripcion, updated_at from pesables where id = ?"#,
                                pes
                            )
                            .fetch_one(db)
                            .await?;
                            res.push(V::Pes((
                                0.0,
                                Pesable::build(
                                    pes.id,
                                    code.codigo,
                                    pes.precio_peso,
                                    pes.porcentaje,
                                    pes.costo_kilo,
                                    pes.descripcion.as_str(),
                                ),
                            )))
                        } else if let Some(rub) = code.rubro {
                            let rub: RubroDB =
                                sqlx::query_as!(RubroDB, "select * from rubros where id = ? ", rub)
                                    .fetch_one(db)
                                    .await?;
                            res.push(V::Rub((
                                0,
                                Rubro::build(rub.id, code.codigo, None, rub.descripcion.as_str()),
                            )));
                        }
                    }
                }
            }
            Err(_) => {
                let filtros = filtro.split(' ').collect::<Vec<&str>>();
                let mut query=String::from("select * from productos where (tipo like %?% or marca like %?% or presentacion like %?% or size like %?%)");
                let row=" and (tipo like %?% or marca like %?% or presentacion like %?% or size like %?%)";
                for _ in 1..filtros.len() {
                    query.push_str(row);
                }
                let mut qres = sqlx::query_as(query.as_ref());
                for filtro in &filtros {
                    qres = qres.bind(filtro).bind(filtro).bind(filtro).bind(filtro);
                }
                let qres: Vec<ProductoDB> = qres.fetch_all(db).await?;
                res.append(
                    &mut qres
                        .iter()
                        .map(|prod| {
                            V::Prod((
                                0,
                                Producto::build(
                                    prod.id,
                                    vec![],
                                    prod.precio_venta,
                                    prod.porcentaje,
                                    prod.precio_costo,
                                    prod.tipo.as_str(),
                                    prod.marca.as_str(),
                                    prod.variedad.as_str(),
                                    Presentacion::build(prod.presentacion.as_str(), prod.size),
                                ),
                            ))
                        })
                        .collect::<Vec<V>>(),
                );
                let query=String::from("select id, precio_peso, codigo, porcentaje, costo_kilo, descripcion, updated_at, from pesables inner join codigos on pesables.id = codigos.pesable where (descripcion like %?%)");
                let mut qres = sqlx::query_as(query.as_str());
                for filtro in &filtros {
                    qres = qres.bind(filtro);
                }
                let qres: Vec<CodedPesDB> = qres.fetch_all(db).await?;
                res.append(
                    &mut qres
                        .iter()
                        .map(|pes| {
                            V::Pes((
                                0.0,
                                Pesable::build(
                                    pes.id,
                                    pes.codigo,
                                    pes.precio_peso,
                                    pes.porcentaje,
                                    pes.costo_kilo,
                                    pes.descripcion.as_str(),
                                ),
                            ))
                        })
                        .collect::<Vec<V>>(),
                );
                let mut query=String::from("select id, descripcion, updated_at, codigo, precio from rubros inner join codigos on rubros.id = codigos.rubro where descripcion like %?%");
                let row = " and descripcion like %?%";
                for _ in 1..filtros.len() {
                    query.push_str(row);
                }
                let mut qres = sqlx::query_as(query.as_str());
                for filtro in filtros {
                    qres = qres.bind(filtro);
                }
                let qres: Vec<CodedRubDB> = qres.fetch_all(db).await?;
                res.append(
                    &mut qres
                        .iter()
                        .map(|rub| {
                            V::Rub((
                                0,
                                Rubro::build(rub.id, rub.codigo, None, rub.descripcion.as_str()),
                            ))
                        })
                        .collect::<Vec<V>>(),
                );
            }
        }

        Ok(res
            .iter()
            .cloned()
            .take(*self.configs.cantidad_productos() as usize)
            .collect())
    }
    pub fn cerrar_sesion(&mut self) {
        self.user = None;
    }

    fn splitx(filtro: &str) -> Res<(f32, &str)> {
        let partes = filtro.split('*').collect::<Vec<&str>>();
        match partes.len() {
            1 => Ok((1.0, partes[0])),
            2 => Ok((partes[0].parse::<f32>()?, partes[1])),
            _ => Err(AppError::ParseError),
        }
    }
    pub async fn proveedores(&self) -> Res<Vec<Proveedor>> {
        let qres: Vec<ProvDB> = sqlx::query_as!(
            ProvDB,
            r#"select id, nombre, contacto, updated from proveedores"#
        )
        .fetch_all(self.read_db.as_ref())
        .await?;
        Ok(qres
            .iter()
            .map(|prov| Proveedor::build(prov.id, prov.nombre.as_str(), prov.contacto))
            .collect::<Vec<Proveedor>>())
    }
    pub fn configs(&self) -> &Config {
        &self.configs
    }

    pub fn eliminar_pago(&mut self, pos: bool, id: i64) -> Res<Vec<Pago>> {
        let res;
        if pos {
            self.ventas.a.eliminar_pago(id, &self.write_db)?;
            res = self.venta(pos).pagos()
        } else {
            self.ventas.b.eliminar_pago(id, &self.write_db)?;
            res = self.venta(pos).pagos()
        }

        Ok(res)
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
        Runtime::new().unwrap().block_on(async {
            sqlx::query("update config set cantidad = ?, mayus = ?, formato = ?, politica = ?")
                .bind(self.configs.cantidad_productos())
                .bind(self.configs.modo_mayus().to_string())
                .bind(self.configs.formato().to_string())
                .bind(self.configs.politica())
                .execute(self.write_db())
                .await
                .unwrap();
        });
    }
    pub fn pagar_deuda_especifica(&self, cliente: i64, venta: Venta) -> Res<Venta> {
        Runtime::new().unwrap().block_on(async{Cli::pagar_deuda_especifica(
            cliente,
            &self.write_db,
            venta,
            &self.user,
        ).await})
    }
    pub fn pagar_deuda_general(&self, cliente: i64, monto: f32) -> Res<f32> {
        Runtime::new().unwrap().block_on(Cli::pagar_deuda_general(cliente, &self.write_db, monto))
    }
    // pub async fn get_cliente(&self, id: i64) -> Res<Cliente> {
    //     let model = CliDB::Entity::find_by_id(id)
    //         .one(self.read_db.as_ref())
    //         .await?
    //         .unwrap();
    //     Ok(Mapper::map_model_cli(model).await)
    // }
    pub async fn agregar_producto(
        &mut self,
        proveedores: Vec<&str>,
        codigos_prov: Vec<&str>,
        codigos_de_barras: Vec<&str>,
        precio_de_venta: &str,
        porcentaje: &str,
        precio_de_costo: &str,
        tipo_producto: &str,
        marca: &str,
        variedad: &str,
        cantidad: &str,
        presentacion: &str,
    ) -> Res<Producto> {
        let mut codigos = Vec::new();
        for code in &codigos_de_barras {
            codigos.push(code.parse::<i64>()?);
        }
        let mut query = String::from("select id as int from codigos where codigo = ?");
        let row = " or codigo = ?";
        for _ in 1..codigos.len() {
            query.push_str(row);
        }
        let mut qres = sqlx::query_as(query.as_str());
        for code in &codigos {
            qres = qres.bind(*code);
        }
        let qres: Option<BigIntDB> = qres.fetch_optional(self.read_db.as_ref()).await?;
        if let Some(res) = qres {
            return Err(AppError::ExistingError {
                objeto: "Codigo".to_string(),
                instancia: res.int.to_string(),
            });
        }
        let qres:Option<BigIntDB>=sqlx::query_as!(BigIntDB,
            "select id as int from productos where tipo = ? and marca = ? and variedad = ? and presentacion = ? and size = ?",tipo_producto,marca,variedad,presentacion,cantidad)
            .fetch_optional(self.read_db.as_ref()).await?;
        match qres {
            None => {
                let tipo_producto = tipo_producto.to_lowercase();
                let marca = marca.to_lowercase();
                let variedad = variedad.to_lowercase();
                let precio_de_venta = precio_de_venta.parse::<f32>()?;
                let porcentaje = porcentaje.parse::<f32>()?;
                let precio_de_costo = precio_de_costo.parse::<f32>()?;
                let pres = Presentacion::build(presentacion, cantidad.parse::<f32>()?);
                let prod_qres =
                    sqlx::query("insert into productos values (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(precio_de_venta)
                        .bind(porcentaje)
                        .bind(precio_de_costo)
                        .bind(tipo_producto.clone())
                        .bind(marca.clone())
                        .bind(variedad.clone())
                        .bind(presentacion)
                        .bind(cantidad)
                        .bind(Utc::now().naive_local())
                        .execute(self.write_db.as_ref())
                        .await?;
                let mut query = String::from("insert into codigos values (?, ?)");
                let row = ", (?, ?)";
                for _ in 1..codigos_de_barras.len() {
                    query.push_str(row);
                }
                let mut qres = sqlx::query(query.as_ref());
                for code in &codigos {
                    qres = qres.bind(code);
                }
                qres.execute(self.write_db.as_ref()).await?;

                let mut query = String::from("insert into relacion_prod_prov values (?, ?, ?)");
                let row = ", (?, ?, ?)";
                for _ in 1..proveedores.len() {
                    query.push_str(row);
                }
                let mut qres = sqlx::query(query.as_ref());
                for i in 0..proveedores.len() {
                    qres = qres
                        .bind(prod_qres.last_insert_rowid())
                        .bind(proveedores[i].parse::<i64>()?)
                        .bind(codigos_prov[i].parse::<i64>().ok());
                }
                qres.execute(self.write_db()).await?;
                Ok(Producto::build(
                    prod_qres.last_insert_rowid(),
                    codigos,
                    precio_de_venta,
                    porcentaje,
                    precio_de_costo,
                    tipo_producto.as_ref(),
                    marca.as_ref(),
                    variedad.as_ref(),
                    pres,
                ))
            }
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: String::from("Producto"),
                    instancia: format!(
                        "{} {} {} {} {}",
                        tipo_producto, marca, variedad, cantidad, presentacion
                    ),
                })
            }
        }
    }

    pub fn agregar_proveedor(&mut self, proveedor: &str, contacto: Option<i64>) -> Res<()> {
        Runtime::new().unwrap().block_on(Proveedor::new_to_db(proveedor, contacto, self.write_db()))?;
        Ok(())
    }

    pub async fn agregar_producto_a_venta(&mut self, prod: V, pos: bool) -> Res<()> {
        let existe = match &prod {
            Valuable::Prod((_, prod)) => {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select id as int from productos where id = ? ",
                    *prod.id()
                )
                .fetch_optional(self.read_db())
                .await?;
                qres.is_some()
            }
            Valuable::Pes((_, pes)) => {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select id as int from pesables where id = ? ",
                    *pes.id()
                )
                .fetch_optional(self.read_db())
                .await?;
                qres.is_some()
            }
            Valuable::Rub((_, rub)) => {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    "select id as int from rubros where id = ? ",
                    *rub.id()
                )
                .fetch_optional(self.read_db())
                .await?;
                qres.is_some()
            }
        };
        let result;

        if existe {
            if pos {
                result = Ok(self
                    .ventas
                    .a
                    .agregar_producto(prod, &self.configs().politica()))
            } else {
                result = Ok(self
                    .ventas
                    .b
                    .agregar_producto(prod, &self.configs().politica()))
            }
        } else {
            return Err(AppError::NotFound {
                objeto: String::from("producto"),
                instancia: prod.descripcion(&self.configs()),
            });
        }

        result
    }
    pub fn descontar_producto_de_venta(
        &mut self,
        index: usize,
        pos: bool,
    ) -> Result<Venta, AppError> {
        Ok(if pos {
            self.ventas
                .a
                .restar_producto(index, &self.configs().politica())?
        } else {
            self.ventas
                .b
                .restar_producto(index, &self.configs().politica())?
        })
    }
    pub fn incrementar_producto_a_venta(
        &mut self,
        index: usize,
        pos: bool,
    ) -> Result<Venta, AppError> {
        let result;
        if pos {
            result = self
                .ventas
                .a
                .incrementar_producto(index, &self.configs().politica());
        } else {
            result = self
                .ventas
                .b
                .incrementar_producto(index, &self.configs().politica());
        }

        result
    }
    pub fn eliminar_producto_de_venta(
        &mut self,
        index: usize,
        pos: bool,
    ) -> Result<Venta, AppError> {
        let result;
        if pos {
            if self.ventas.a.productos().len() > 1 {
                result = self
                    .ventas
                    .a
                    .eliminar_producto(index, &self.configs().politica());
            } else {
                self.ventas.a.empty();
                result = Ok(self.ventas.a.clone());
            }
        } else {
            if self.ventas.b.productos().len() > 1 {
                result = self
                    .ventas
                    .b
                    .eliminar_producto(index, &self.configs().politica());
            } else {
                self.ventas.b.empty();
                result = Ok(self.ventas.b.clone());
            }
        }

        result
    }
    pub fn venta(&self, pos: bool) -> Venta {
        if pos {
            self.ventas.a.clone()
        } else {
            self.ventas.b.clone()
        }
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Res<Vec<String>> {
        Runtime::new().unwrap().block_on(async {
            let qres: Vec<StringDB> = sqlx::query_as!(
                StringDB,
                "select marca as string from productos where marca like ? order by marca asc",
                filtro
            )
            .fetch_all(self.read_db())
            .await?;
            Ok(qres
                .iter()
                .map(|s| s.string.to_owned())
                .collect::<HashSet<String>>()
                .iter()
                .cloned()
                .collect::<Vec<String>>())
        })
    }
    // pub fn get_deuda_cliente(&self, cliente: Cli)->Res<f64>{

    // }
    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Res<Vec<String>> {
        Runtime::new().unwrap().block_on(async {
            let qres: Vec<StringDB> = sqlx::query_as!(
                StringDB,
                "select marca as string from productos where tipo like ? order by tipo asc",
                filtro
            )
            .fetch_all(self.read_db())
            .await?;

            Ok(qres
                .iter()
                .map(|d| d.string.to_owned())
                .collect::<HashSet<String>>()
                .iter()
                .cloned()
                .collect::<Vec<String>>())
        })
    }
    pub fn write_db(&self) -> &Pool<Sqlite> {
        &self.write_db
    }
    pub fn read_db(&self) -> &Pool<Sqlite> {
        &self.read_db
    }
    fn set_venta(&mut self, pos: bool, venta: Venta) {
        if pos {
            self.ventas.a = venta;
        } else {
            self.ventas.b = venta;
        }
    }
    fn cerrar_venta(&mut self, pos: bool) -> Res<()> {
        Runtime::new().unwrap().block_on(self.venta(pos).guardar(pos, self.write_db()))?;
        self.registro.push(self.venta(pos).clone());
        println!("{:#?}", self.venta(pos));
        Runtime::new().unwrap().block_on(
            self.update_total(self.venta(pos).monto_total(), &self.venta(pos).pagos()),
        )?;
        self.set_venta(
            pos,
            Runtime::new().unwrap().block_on(Venta::get_or_new(
                Some(self.arc_user()),
                self.write_db(),
                pos,
            ))?,
        );
        Ok(())
    }
    pub fn hacer_ingreso(&self, monto: f32, descripcion: Option<Arc<str>>) -> Res<()> {
        let mov = Movimiento::Ingreso { descripcion, monto };
        Runtime::new().unwrap().block_on(self.caja.hacer_movimiento(mov, &self.write_db))
    }
    pub fn hacer_egreso(&self, monto: f32, descripcion: Option<Arc<str>>) -> Res<()> {
        let mov = Movimiento::Egreso { descripcion, monto };
        Runtime::new().unwrap().block_on(self.caja.hacer_movimiento(mov, &self.write_db))
    }
    pub fn get_deuda(&self, cliente: Cli) -> Res<f32> {
        Runtime::new().unwrap().block_on(cliente.get_deuda(&self.read_db))
    }
    pub fn get_deuda_detalle(&self, cliente: Cli) -> Res<Vec<Venta>> {
        Runtime::new().unwrap().block_on(cliente.get_deuda_detalle(&self.read_db, self.user()))
    }
    pub fn eliminar_valuable(&self, val: V) {
        Runtime::new().unwrap().block_on(val.eliminar(self.write_db.as_ref()));
    }
    pub fn editar_valuable(&self, val: V) {
        Runtime::new().unwrap().block_on(val.editar(self.write_db.as_ref()));
    }
    pub fn arc_user(&self) -> Arc<User> {
        Arc::clone(&self.user.as_ref().unwrap())
    }
    pub fn stash_sale(&mut self, pos: bool) -> Res<()> {
        self.stash.push(self.venta(pos));
        self.set_venta(
            pos,
            Runtime::new().unwrap().block_on(Venta::get_or_new(
                Some(self.arc_user()),
                self.write_db(),
                pos,
            ))?,
        );
        Ok(())
    }
    pub fn set_cantidad_producto_venta(
        &mut self,
        index: usize,
        cantidad: f32,
        pos: bool,
    ) -> Res<Venta> {
        if index < self.venta(pos).productos().len() {
            if pos {
                self.ventas
                    .a
                    .set_cantidad_producto(index, cantidad, &self.configs.politica())
            } else {
                self.ventas
                    .b
                    .set_cantidad_producto(index, cantidad, &self.configs.politica())
            }
        } else {
            Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: index.to_string(),
            })
        }
    }
    pub fn set_cliente(&mut self, id: i64, pos: bool) -> Res<()> {
        if pos {
            Runtime::new().unwrap().block_on(self.ventas.a.set_cliente(id, &self.read_db))
        } else {
            Runtime::new().unwrap().block_on(self.ventas.b.set_cliente(id, &self.read_db))
        }
    }
    pub fn unstash_sale(&mut self, pos: bool, index: usize) -> Res<()> {
        if index < self.stash.len() {
            if self.venta(pos).productos().len() > 0 {
                self.stash.push(self.venta(pos).to_owned())
            }
            let venta = self.stash.remove(index);
            self.set_venta(pos, venta);
            Ok(())
        } else {
            Err(AppError::SaleSelection.into())
        }
    }
    pub fn stash(&self) -> &Vec<Venta> {
        &self.stash
    }
    pub async fn update_total(&mut self, monto: f32, pagos: &Vec<Pago>) -> Result<(), AppError> {
        self.caja.update_total(&self.write_db, monto, pagos).await
    }
}
