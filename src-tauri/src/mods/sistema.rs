type Res<T> = std::result::Result<T, AppError>;
use super::{
    caja::{Caja, Movimiento},
    cliente::{Cli, Cliente},
    config::Config,
    error::AppError,
    lib::{crear_file, get_hash, leer_file, Db, Mapper},
    pago::Pago,
    pesable::Pesable,
    producto::Producto,
    proveedor::Proveedor,
    relacion_prod_prov::RelacionProdProv,
    rubro::Rubro,
    user::{Rango, User},
    valuable::{Presentacion, Valuable, ValuableTrait},
    venta::Venta,
};
use chrono::Utc;
use entity::prelude::{
    CajaDB, CliDB, CodeDB, ConfDB, MedioDB, PesDB, ProdDB, ProdProvDB, ProvDB, RubDB, UserDB,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use std::{collections::HashSet, sync::Arc};
use tauri::{async_runtime, async_runtime::JoinHandle};
use Valuable as V;
const CUENTA: &str = "Cuenta Corriente";
pub struct Sistema {
    user: Option<Arc<User>>,
    write_db: Arc<DatabaseConnection>,
    read_db: Arc<DatabaseConnection>,
    caja: Caja,
    configs: Config,
    ventas: (Venta, Venta),
    proveedores: Vec<Proveedor>,
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}

async fn get_db(path: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(path).await
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
        credito: bool,
        activo: bool,
        limite: Option<f64>,
    ) -> Res<Cli> {
        async_runtime::block_on(Cli::new_to_db(
            self.write_db(),
            nombre,
            dni,
            credito,
            activo,
            Utc::now().naive_local(),
            limite,
        ))
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f64, pos: bool) -> Res<f64> {
        let res;
        if pos {
            if !medio_pago.eq("Efectivo")
                && self.ventas.0.monto_pagado() + monto > self.ventas.0.monto_total()
            {
                return Err(AppError::AmountError {
                    a_pagar: self.ventas.0.monto_total() - self.ventas.0.monto_pagado(),
                    pagado: monto,
                });
            } else {
                res = self.ventas.0.agregar_pago(medio_pago, monto);
            }
        } else {
            if !medio_pago.eq("Efectivo")
                && self.ventas.1.monto_pagado() + monto > self.ventas.1.monto_total()
            {
                return Err(AppError::AmountError {
                    a_pagar: self.ventas.1.monto_total() - self.ventas.1.monto_pagado(),
                    pagado: monto,
                });
            } else {
                res = self.ventas.1.agregar_pago(medio_pago, monto);
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
        async_runtime::block_on(User::new_to_db(
            Arc::from(id),
            Arc::from(nombre),
            get_hash(pass),
            rango,
            self.write_db(),
        ))
    }
    pub fn new() -> Res<Sistema> {
        let write_db = Arc::from(async_runtime::block_on(get_db(
            "sqlite://db.sqlite?mode=rwc",
        ))?);
        let read_db = Arc::from(async_runtime::block_on(get_db(
            "sqlite://db.sqlite?mode=ro",
        ))?);

        async_runtime::block_on(async {
            if let Err(_) = CajaDB::Entity::find().one(read_db.as_ref()).await {
                Migrator::fresh(write_db.as_ref()).await
            } else {
                Ok(())
            }
        })
        .unwrap();
        let path_proveedores = "Proveedores.json";
        let path_relaciones = "Relaciones.json";
        let mut relaciones = Vec::new();
        leer_file(&mut relaciones, path_relaciones)?;
        let mut proveedores: Vec<Proveedor> = Vec::new();
        leer_file(&mut proveedores, path_proveedores)?;

        let aux = Arc::clone(&write_db);
        let db = Arc::clone(&write_db);
        let configs = async_runtime::block_on(Config::get_or_def(db.as_ref()))?;
        let caja = async_runtime::block_on(Caja::new(aux, Some(0.0), &configs))?;
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
            ventas: (
                async_runtime::block_on(Venta::get_or_new(None, w1.as_ref(), true))?,
                async_runtime::block_on(Venta::get_or_new(None, w1.as_ref(), false))?,
            ),
            proveedores: proveedores.clone(),
            relaciones,
            stash,
            registro,
        };
        Sistema::procesar(
            Arc::clone(&sis.write_db),
            Arc::clone(&sis.read_db),
            sis.proveedores.clone(),
            sis.relaciones.clone(),
        )?;
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
            self.ventas.0.empty();
        } else {
            self.ventas.1.empty();
        }
        Ok(())
    }
    pub fn cerrar_caja(&mut self, monto_actual: f64) -> Res<()> {
        self.caja.set_cajero(self.user().unwrap().nombre());
        let db = Arc::clone(&self.write_db);
        async_runtime::block_on(self.caja.set_n_save(db.as_ref(), monto_actual))?;
        self.generar_reporte_caja();
        self.caja = async_runtime::block_on(Caja::new(
            Arc::clone(&self.write_db),
            Some(monto_actual),
            &self.configs,
        ))?;
        Ok(())
    }
    pub fn eliminar_usuario(&self, user: User) -> Res<()> {
        async_runtime::spawn(Db::eliminar_usuario(user, Arc::clone(&self.read_db)));
        Ok(())
    }

    pub fn caja(&self) -> &Caja {
        &self.caja
    }
    fn procesar(
        write_db: Arc<DatabaseConnection>,
        read_db: Arc<DatabaseConnection>,
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
        // check_codes(&mut productos);

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
        let _: JoinHandle<Result<(), AppError>> = async_runtime::spawn(async move {
            let medios = vec!["Efectivo", "Crédito", "Débito"];
            for medio in medios {
                if MedioDB::Entity::find()
                    .filter(MedioDB::Column::Medio.eq(medio))
                    .one(read_db2.as_ref())
                    .await?
                    .is_none()
                {
                    let model = MedioDB::ActiveModel {
                        medio: Set(medio.to_string()),
                        ..Default::default()
                    };
                    model.insert(write_db2.as_ref()).await?;
                }
            }
            if MedioDB::Entity::find()
                .filter(MedioDB::Column::Medio.eq(CUENTA))
                .one(read_db2.as_ref())
                .await?
                .is_none()
            {
                let model = MedioDB::ActiveModel {
                    medio: Set(CUENTA.to_string()),
                    id: Set(0),
                };
                model.insert(write_db2.as_ref()).await?;
            }
            return Ok(());
        });
        if async_runtime::block_on(UserDB::Entity::find().count(read_db.as_ref()))? == 0 {
            async_runtime::spawn(async move {
                let db = Arc::clone(&write_db);
                let model = UserDB::ActiveModel {
                    user_id: Set("admin".to_owned()),
                    pass: Set(get_hash("1234")),
                    rango: Set(Rango::Admin.to_string()),
                    nombre: Set("Admin".to_owned()),
                    ..Default::default()
                };
                model.insert(db.as_ref()).await.unwrap();
            });
            async_runtime::spawn(Db::cargar_todos_los_valuables(valuables));
            async_runtime::spawn(Db::cargar_todos_los_provs(proveedores));
            async_runtime::spawn(Db::cargar_todas_las_relaciones_prod_prov(relaciones));
        }
        Ok(())
    }

    pub async fn get_clientes(&self) -> Res<Vec<Cli>> {
        Ok(CliDB::Entity::find()
            .all(self.read_db())
            .await?
            .iter()
            .map(|model| {
                Cli::new(
                    model.id,
                    Arc::from(model.nombre.as_str()),
                    model.dni,
                    model.credito,
                    model.activo,
                    model.created,
                    model.limite,
                )
            })
            .collect::<Vec<Cli>>())
    }
    pub async fn try_login(&mut self, id: &str, pass: i64) -> Res<Rango> {
        match UserDB::Entity::find()
            .filter(
                Condition::all()
                    .add(UserDB::Column::UserId.eq(id.to_string()))
                    .add(UserDB::Column::Pass.eq(pass)),
            )
            .one(self.read_db())
            .await?
        {
            Some(user) => {
                self.user = Some(Arc::from(User::new(
                    Arc::from(user.user_id),
                    Arc::from(user.nombre),
                    user.pass,
                    user.rango.as_str(),
                )));
                self.ventas = (
                    Venta::get_or_new(Some(self.arc_user()), &self.write_db, true).await?,
                    Venta::get_or_new(Some(self.arc_user()), &self.write_db, false).await?,
                );
                Ok(self.user().unwrap().rango().clone())
            }
            None => match UserDB::Entity::find()
                .filter(UserDB::Column::UserId.eq(id))
                .one(self.read_db())
                .await?
            {
                Some(_) => Err(AppError::IncorrectError("Contraseña".to_string())),
                None => Err(AppError::IncorrectError("Usuario".to_string())),
            },
        }
    }
    pub async fn val_filtrado(
        &self,
        filtro: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<Valuable>, AppError> {
        let mut res: Vec<Valuable>;
        res = self
            .prods_filtrado(filtro, db)
            .await?
            .iter()
            .cloned()
            .map(|x| V::Prod(x))
            .collect();
        res.append(
            &mut self
                .pes_filtrado(filtro, db)
                .await?
                .iter()
                .cloned()
                .map(|x| V::Pes(x))
                .collect(),
        );
        res.append(
            &mut self
                .rub_filtrado(filtro, db)
                .await?
                .iter()
                .cloned()
                .map(|x| V::Rub(x))
                .collect(),
        );
        Ok(res)
    }
    pub fn cerrar_sesion(&mut self) {
        self.user = None;
    }
    pub async fn pes_filtrado(
        &self,
        filtro: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<(f32, Pesable)>, AppError> {
        let (cant, filtro) = Sistema::splitx(filtro)?;
        let mut prods = Vec::new();
        match filtro.parse::<i64>() {
            Ok(code) => {
                if let Some(model) = PesDB::Entity::find()
                    .filter(PesDB::Column::Codigo.eq(code))
                    .one(db)
                    .await?
                {
                    prods.push((cant, Mapper::map_model_pes(&model)))
                }
            }
            Err(_) => {
                let filtros = filtro.split(' ').collect::<Vec<&str>>();

                let mut res = Vec::new();
                for i in 0..filtros.len() {
                    if i == 0 {
                        res = PesDB::Entity::find()
                            .filter(PesDB::Column::Descripcion.contains(filtros[i]))
                            .order_by_asc(PesDB::Column::Id)
                            .limit(Some((self.configs().cantidad_productos() * 2) as u64))
                            .all(self.read_db())
                            .await?;
                    } else {
                        res = res
                            .iter()
                            .cloned()
                            .filter(|modelo| {
                                modelo
                                    .descripcion
                                    .to_lowercase()
                                    .contains(filtros[i].to_lowercase().as_str())
                            })
                            .take(*self.configs().cantidad_productos() as usize)
                            .collect();
                    }
                }
                for model in &res {
                    prods.push((cant, Mapper::map_model_pes(model)));
                }
            }
        }
        Ok(prods.to_owned())
    }
    pub async fn rub_filtrado(
        &self,
        filtro: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<(u8, Rubro)>, AppError> {
        let mut prods = Vec::new();
        let (cant, filtro) = Sistema::splitx(filtro)?;
        match filtro.parse::<i64>() {
            Ok(code) => {
                if let Some(model) = RubDB::Entity::find()
                    .filter(RubDB::Column::Codigo.eq(code))
                    .one(db)
                    .await?
                {
                    prods.push((cant as u8, Mapper::map_model_rub(&model, 0.0)))
                }
            }
            Err(_) => {
                let filtros = filtro.split(' ').collect::<Vec<&str>>();
                let mut res = Vec::new();
                for i in 0..filtros.len() {
                    if i == 0 {
                        res = RubDB::Entity::find()
                            .filter(RubDB::Column::Descripcion.contains(filtros[i]))
                            .order_by_asc(RubDB::Column::Id)
                            .limit(Some((self.configs().cantidad_productos() * 2) as u64))
                            .all(self.read_db())
                            .await?;
                    } else {
                        res = res
                            .iter()
                            .cloned()
                            .filter(|modelo| {
                                modelo
                                    .descripcion
                                    .to_lowercase()
                                    .contains(filtros[i].to_lowercase().as_str())
                            })
                            .take(*self.configs().cantidad_productos() as usize)
                            .collect();
                    }
                }
                for model in &res {
                    prods.push((cant as u8, Mapper::map_model_rub(model, 0.0)));
                }
            }
        }
        Ok(prods)
    }
    pub async fn prods_filtrado(
        &self,
        filtro: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<(u8, Producto)>, AppError> {
        let (cant, filtro) = Sistema::splitx(filtro)?;
        let mut prods = Vec::new();
        match filtro.parse::<f64>() {
            Ok(code) => {
                if let Some(id) = CodeDB::Entity::find()
                    .filter(CodeDB::Column::Codigo.eq(code))
                    .one(db)
                    .await?
                {
                    prods.push({
                        let model = ProdDB::Entity::find_by_id(id.producto)
                            .one(db)
                            .await?
                            .unwrap();
                        (
                            cant as u8,
                            Mapper::map_model_prod(&model, db)
                                .await?
                                .redondear(&self.configs().politica()),
                        )
                    })
                }
            }
            Err(_) => {
                let mut res = Vec::new();
                let filtros = filtro.split(' ').collect::<Vec<&str>>();
                for i in 0..filtros.len() {
                    if i == 0 {
                        res = ProdDB::Entity::find()
                            .filter(
                                Condition::any()
                                    .add(ProdDB::Column::Marca.contains(filtros[i]))
                                    .add(ProdDB::Column::TipoProducto.contains(filtros[i]))
                                    .add(ProdDB::Column::Variedad.contains(filtros[i])),
                            )
                            .order_by_asc(ProdDB::Column::Id)
                            .limit(Some((self.configs().cantidad_productos() * 2) as u64))
                            .all(self.read_db())
                            .await?;
                    } else {
                        res = res
                            .iter()
                            .cloned()
                            .filter(|modelo| {
                                modelo
                                    .marca
                                    .to_lowercase()
                                    .contains(filtros[i].to_lowercase().as_str())
                                    || modelo
                                        .variedad
                                        .to_lowercase()
                                        .contains(filtros[i].to_lowercase().as_str())
                                    || modelo
                                        .tipo_producto
                                        .to_lowercase()
                                        .contains(filtros[i].to_lowercase().as_str())
                            })
                            .take(*self.configs().cantidad_productos() as usize)
                            .collect();
                    }
                }
                for model in &res {
                    prods.push((
                        cant as u8,
                        Mapper::map_model_prod(model, self.read_db())
                            .await?
                            .redondear(&self.configs().politica()),
                    ));
                }
            }
        }
        Ok(prods)
    }
    fn splitx(filtro: &str) -> Res<(f32, &str)> {
        let partes = filtro.split('*').collect::<Vec<&str>>();
        match partes.len() {
            1 => Ok((1.0, partes[0])),
            2 => Ok((partes[0].parse::<f32>()?, partes[1])),
            _ => Err(AppError::ParseError),
        }
    }
    pub async fn proveedores(&self) -> Vec<Proveedor> {
        match ProvDB::Entity::find().all(self.read_db()).await {
            Ok(a) => {
                let res = a
                    .iter()
                    .map(|x| Mapper::map_model_prov(x))
                    .collect::<Vec<Proveedor>>();
                res
            }
            Err(e) => panic!("Error {}", e),
        }
    }
    pub fn configs(&self) -> &Config {
        &self.configs
    }

    pub fn eliminar_pago(&mut self, pos: bool, id: u32) -> Res<Vec<Pago>> {
        let res;
        if pos {
            self.ventas.0.eliminar_pago(id)?;
            res = self.venta(pos).pagos()
        } else {
            self.ventas.1.eliminar_pago(id)?;
            res = self.venta(pos).pagos()
        }

        Ok(res)
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
        async_runtime::block_on(async {
            let mut res = ConfDB::Entity::find()
                .one(self.read_db())
                .await
                .unwrap()
                .unwrap()
                .into_active_model();
            res.cantidad_productos = Set(*self.configs().cantidad_productos());
            res.formato_producto = Set(self.configs().formato().to_string());
            res.modo_mayus = Set(self.configs().modo_mayus().to_string());
            res.politica_redondeo = Set(self.configs().politica());
            res.update(self.write_db()).await.unwrap();
        });
    }
    pub fn pagar_deuda_especifica(&self, cliente: i64, venta: Venta) -> Res<Venta> {
        async_runtime::block_on(Cli::pagar_deuda_especifica(
            cliente,
            &self.write_db,
            venta,
            &self.user,
        ))
    }
    pub fn pagar_deuda_general(&self, cliente: i64, monto: f64) -> Res<f64> {
        async_runtime::block_on(Cli::pagar_deuda_general(cliente, &self.write_db, monto))
    }
    pub async fn get_cliente(&self, id: i64) -> Res<Cliente> {
        let model = CliDB::Entity::find_by_id(id)
            .one(self.read_db.as_ref())
            .await?
            .unwrap();
        Ok(Mapper::map_model_cli(model).await)
    }
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
        let tipo_producto = tipo_producto.to_lowercase();
        let marca = marca.to_lowercase();
        let variedad = variedad.to_lowercase();

        let precio_de_venta = precio_de_venta.parse::<f64>()?;
        let porcentaje = porcentaje.parse::<f64>()?;
        let precio_de_costo = precio_de_costo.parse::<f64>()?;
        let codigos_de_barras: Vec<i64> = codigos_de_barras
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let presentacion = match presentacion {
            "Gr" => Presentacion::Gr(cantidad.parse().unwrap()),
            "Un" => Presentacion::Un(cantidad.parse().unwrap()),
            "Lt" => Presentacion::Lt(cantidad.parse().unwrap()),
            "Ml" => Presentacion::Ml(cantidad.parse().unwrap()),
            "CC" => Presentacion::CC(cantidad.parse().unwrap()),
            "Kg" => Presentacion::Kg(cantidad.parse().unwrap()),
            _ => panic!("no posible {presentacion}"),
        };
        let prod_model = ProdDB::ActiveModel {
            precio_de_venta: Set(precio_de_venta),
            porcentaje: Set(porcentaje),
            precio_de_costo: Set(precio_de_costo),
            tipo_producto: Set(tipo_producto.to_string()),
            marca: Set(marca.to_owned()),
            variedad: Set(variedad.to_owned()),
            presentacion: Set(presentacion.get_string()),
            updated_at: Set(Utc::now().naive_local()),
            cantidad: Set(presentacion.get_cantidad()),
            ..Default::default()
        };
        let res_prod = ProdDB::Entity::insert(prod_model)
            .exec(self.write_db())
            .await?;
        let codigos_model: Vec<CodeDB::ActiveModel> = codigos_de_barras
            .iter()
            .map(|x| CodeDB::ActiveModel {
                codigo: Set(*x),
                producto: Set(res_prod.last_insert_id),
                ..Default::default()
            })
            .collect();

        CodeDB::Entity::insert_many(codigos_model)
            .exec(self.write_db())
            .await?;
        for i in 0..codigos_prov.len() {
            let codigo = if codigos_prov[i].len() == 0 {
                None
            } else {
                Some(codigos_prov[i].parse::<i64>()?)
            };
            if let Some(prov) = ProvDB::Entity::find()
                .filter(Condition::all().add(ProvDB::Column::Nombre.eq(proveedores[i])))
                .one(self.write_db())
                .await?
            {
                let relacion_model = ProdProvDB::ActiveModel {
                    producto: Set(res_prod.last_insert_id),
                    proveedor: Set(prov.id),
                    codigo: Set(codigo),
                    ..Default::default()
                };
                ProdProvDB::Entity::insert(relacion_model)
                    .exec(self.write_db())
                    .await?;
            }
        }

        let producto = Producto::new(
            res_prod.last_insert_id,
            codigos_de_barras,
            precio_de_venta,
            porcentaje,
            precio_de_costo,
            tipo_producto.as_str(),
            marca.as_str(),
            variedad.as_str(),
            presentacion,
        );

        let result = Ok(producto.clone());

        for i in 0..proveedores.len() {
            match codigos_prov[i].parse::<i64>() {
                Ok(a) => {
                    self.relaciones
                        .push(RelacionProdProv::new(*producto.id(), i as i64, Some(a)))
                }
                Err(_) => {
                    self.relaciones
                        .push(RelacionProdProv::new(*producto.id(), i as i64, None))
                }
            };
        }

        result
    }

    pub fn agregar_proveedor(&mut self, proveedor: &str, contacto: Option<i64>) -> Res<()> {
        async_runtime::block_on(Proveedor::new_to_db(proveedor, contacto, self.write_db()))?;
        Ok(())
    }

    pub async fn agregar_producto_a_venta(&mut self, prod: V, pos: bool) -> Res<()> {
        let existe = match &prod {
            Valuable::Prod(a) => ProdDB::Entity::find_by_id(*a.1.id())
                .one(self.read_db())
                .await?
                .is_some(),
            Valuable::Pes(a) => PesDB::Entity::find_by_id(*a.1.id())
                .one(self.read_db())
                .await?
                .is_some(),
            Valuable::Rub(a) => RubDB::Entity::find_by_id(*a.1.id())
                .one(self.read_db())
                .await?
                .is_some(),
        };
        let result;

        if existe {
            if pos {
                result = Ok(self
                    .ventas
                    .0
                    .agregar_producto(prod, &self.configs().politica()))
            } else {
                result = Ok(self
                    .ventas
                    .1
                    .agregar_producto(prod, &self.configs().politica()))
            }
        } else {
            return Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: format!("{}", prod.descripcion(&self.configs())),
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
                .0
                .restar_producto(index, &self.configs().politica())?
        } else {
            self.ventas
                .1
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
                .0
                .incrementar_producto(index, &self.configs().politica());
        } else {
            result = self
                .ventas
                .1
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
            if self.ventas.0.productos().len() > 1 {
                result = self
                    .ventas
                    .0
                    .eliminar_producto(index, &self.configs().politica());
            } else {
                self.ventas.0.empty();
                result = Ok(self.ventas.0.clone());
            }
        } else {
            if self.ventas.1.productos().len() > 1 {
                result = self
                    .ventas
                    .1
                    .eliminar_producto(index, &self.configs().politica());
            } else {
                self.ventas.1.empty();
                result = Ok(self.ventas.1.clone());
            }
        }

        result
    }
    pub fn venta(&self, pos: bool) -> Venta {
        if pos {
            self.ventas.0.clone()
        } else {
            self.ventas.1.clone()
        }
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Res<Vec<String>> {
        let mut hash = HashSet::new();
        async_runtime::block_on(async {
            ProdDB::Entity::find()
                .filter(ProdDB::Column::Marca.contains(filtro))
                .order_by(ProdDB::Column::Marca, sea_orm::Order::Asc)
                .all(self.read_db())
                .await?
                .iter()
                .for_each(|x| {
                    hash.insert(x.marca.clone());
                });
            Ok(hash.into_iter().collect::<Vec<String>>())
        })
    }
    // pub fn get_deuda_cliente(&self, cliente: Cli)->Res<f64>{

    // }
    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Res<Vec<String>> {
        let mut hash = HashSet::new();
        async_runtime::block_on(async {
            ProdDB::Entity::find()
                .filter(ProdDB::Column::TipoProducto.contains(filtro))
                .order_by(ProdDB::Column::TipoProducto, sea_orm::Order::Asc)
                .all(self.read_db())
                .await?
                .iter()
                .for_each(|x| {
                    hash.insert(x.tipo_producto.clone());
                });
            Ok(hash.into_iter().collect::<Vec<String>>())
        })
    }
    pub fn write_db(&self) -> &DatabaseConnection {
        &self.write_db
    }
    pub fn read_db(&self) -> &DatabaseConnection {
        &self.read_db
    }
    fn set_venta(&mut self, pos: bool, venta: Venta) {
        if pos {
            self.ventas.0 = venta;
        } else {
            self.ventas.1 = venta;
        }
    }
    fn cerrar_venta(&mut self, pos: bool) -> Res<()> {
        async_runtime::block_on(self.venta(pos).guardar(pos, self.write_db()))?;
        self.registro.push(self.venta(pos).clone());
        println!("{:#?}", self.venta(pos));
        async_runtime::block_on(
            self.update_total(self.venta(pos).monto_total(), &self.venta(pos).pagos()),
        )?;
        self.set_venta(
            pos,
            async_runtime::block_on(Venta::get_or_new(
                Some(self.arc_user()),
                self.write_db(),
                pos,
            ))?,
        );
        Ok(())
    }
    pub fn hacer_ingreso(&self, monto: f64, descripcion: Option<Arc<str>>) -> Res<()> {
        let mov = Movimiento::Ingreso { descripcion, monto };
        async_runtime::block_on(self.caja.hacer_movimiento(mov, &self.write_db))
    }
    pub fn hacer_egreso(&self, monto: f64, descripcion: Option<Arc<str>>) -> Res<()> {
        let mov = Movimiento::Egreso { descripcion, monto };
        async_runtime::block_on(self.caja.hacer_movimiento(mov, &self.write_db))
    }
    pub fn get_deuda(&self, cliente: Cli) -> Res<f64> {
        async_runtime::block_on(cliente.get_deuda(&self.read_db))
    }
    pub fn get_deuda_detalle(&self, cliente: Cli) -> Res<Vec<Venta>> {
        async_runtime::block_on(cliente.get_deuda_detalle(&self.read_db, self.user()))
    }
    pub fn eliminar_valuable(&self, val:V){
        async_runtime::spawn(val.eliminar(self.write_db.as_ref().clone()));
    }
    pub fn arc_user(&self) -> Arc<User> {
        Arc::clone(&self.user.as_ref().unwrap())
    }
    pub fn stash_sale(&mut self, pos: bool) -> Res<()> {
        self.stash.push(self.venta(pos));
        self.set_venta(
            pos,
            async_runtime::block_on(Venta::get_or_new(
                Some(self.arc_user()),
                self.write_db(),
                pos,
            ))?,
        );
        Ok(())
    }
    pub fn set_cliente(&mut self, id: i32, pos: bool) -> Res<()> {
        if pos {
            async_runtime::block_on(self.ventas.0.set_cliente(id, &self.read_db))
        } else {
            async_runtime::block_on(self.ventas.1.set_cliente(id, &self.read_db))
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
    pub async fn update_total(&mut self, monto: f64, pagos: &Vec<Pago>) -> Result<(), AppError> {
        self.caja.update_total(&self.write_db, monto, pagos).await
    }
}
