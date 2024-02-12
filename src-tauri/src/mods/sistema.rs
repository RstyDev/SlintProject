type Res<T> = std::result::Result<T, AppError>;
use chrono::Utc;
use entity::codigo_barras;
use entity::*;
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;

use sea_orm::IntoActiveModel;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::Set;
use sea_orm::{Database, EntityTrait};
use std::collections::HashSet;
use std::sync::Arc;
use tauri::async_runtime;
use tauri::async_runtime::JoinHandle;
use Valuable as V;

use crate::mods::caja::Caja;
use crate::mods::lib::cargar_todas_las_relaciones_prod_prov;
use crate::mods::lib::cargar_todos_los_provs;
use crate::mods::lib::cargar_todos_los_valuables;
use crate::mods::lib::get_hash;
use crate::mods::user::Rango;
use crate::mods::vendedor::Vendedor;

use super::error::AppError;

use super::lib::save;
use super::lib::Mapper;
use super::proveedor::Proveedor;
use super::user::User;
use super::valuable::Presentacion;
use super::{
    config::Config,
    lib::{crear_file, leer_file},
    pesable::Pesable,
    producto::Producto,
    relacion_prod_prov::RelacionProdProv,
    rubro::Rubro,
    valuable::{Valuable, ValuableTrait},
    venta::Venta,
};

pub struct Sistema {
    user: Option<User>,
    write_db: Arc<DatabaseConnection>,
    read_db: Arc<DatabaseConnection>,
    caja: Caja,
    vendedor: Arc<Vendedor>,
    configs: Config,
    ventas: (Venta, Venta),
    proveedores: Vec<Proveedor>,
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}

async fn get_cantidad_productos() -> Result<u64, DbErr> {
    let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
    Ok(entity::producto::Entity::find().count(&db).await?)
}
// fn check_codes(prods: &mut Vec<Producto>) {
//     for i in 0..prods.len() {
//         println!("Producto {}", i);
//         for j in 0..prods[i].codigos_de_barras.len() {
//             for k in i + 1..prods.len() {
//                 let mut l = 0;
//                 while l < prods[k].codigos_de_barras.len() {
//                     if prods[i].codigos_de_barras[j] == prods[k].codigos_de_barras[l] {
//                         prods[k].rm_code(l);
//                     } else {
//                         l += 1;
//                     }
//                 }
//             }
//         }
//     }
// }

async fn get_db(path: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(path).await
}

impl<'a> Sistema {
    pub fn new() -> Res<Sistema> {
        let write_db = Arc::from(async_runtime::block_on(get_db(
            "sqlite://db.sqlite?mode=rwc",
        ))?);
        let read_db = Arc::from(async_runtime::block_on(get_db(
            "sqlite://db.sqlite?mode=ro",
        ))?);

        async_runtime::block_on(async {
            if let Err(_) = entity::caja::Entity::find().one(read_db.as_ref()).await {
                Migrator::fresh(write_db.as_ref()).await
            } else {
                println!("Dio ok");
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
        let aux2 = Arc::clone(&write_db);
        let vendedor = async_runtime::spawn(Vendedor::get_or_def(aux2));
        let caja = async_runtime::spawn(Caja::new(aux, Some(0.0)));

        let stash = Vec::new();
        let registro = Vec::new();

        println!(
            "Acá la cantidad de producto actual {}",
            async_runtime::block_on(get_cantidad_productos()).unwrap()
        );

        let vendedor = Arc::from(async_runtime::block_on(vendedor)??);
        let caja = async_runtime::block_on(caja)??;
        let w1 = Arc::clone(&write_db);
        let db = Arc::clone(&read_db);
        let sis = Sistema {
            user: None,
            write_db,
            read_db,
            caja,
            vendedor: Arc::clone(&vendedor),
            configs: async_runtime::block_on(Config::get_or_def(db.as_ref()))?,
            ventas: (
                async_runtime::block_on(Venta::new(Arc::clone(&vendedor), w1.as_ref()))?,
                async_runtime::block_on(Venta::new(vendedor, w1.as_ref()))?,
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
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }
    fn procesar(
        write_db: Arc<DatabaseConnection>,
        read_db: Arc<DatabaseConnection>,
        proveedores: Vec<Proveedor>,
        relaciones: Vec<RelacionProdProv>,
    ) -> Res<()> {
        let path_productos = "Productos.json";
        println!("procesando");
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
                if entity::medio_pago::Entity::find()
                    .filter(entity::medio_pago::Column::Medio.eq(medio))
                    .one(read_db2.as_ref())
                    .await?
                    .is_none()
                {
                    let model = entity::medio_pago::ActiveModel {
                        medio: Set(medio.to_string()),
                        ..Default::default()
                    };
                    model.insert(write_db2.as_ref()).await?;
                }
            }
            return Ok(());
        });
        if async_runtime::block_on(entity::user::Entity::find().count(read_db.as_ref()))? == 0 {
            async_runtime::spawn(async move {
                let db = Arc::clone(&write_db);
                let model = entity::user::ActiveModel {
                    user_id: Set("admin".to_owned()),
                    pass: Set(get_hash("1234")),
                    rango: Set(Rango::Admin.to_string()),
                    ..Default::default()
                };
                model.insert(db.as_ref()).await.unwrap();
            });
            async_runtime::spawn(cargar_todos_los_valuables(valuables));
            async_runtime::spawn(cargar_todos_los_provs(proveedores));
            async_runtime::spawn(cargar_todas_las_relaciones_prod_prov(relaciones));
        }
        Ok(())
    }
    pub async fn try_login(&mut self, id: &str, pass: i64) -> Res<()> {
        match entity::user::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::user::Column::UserId.eq(id.to_string()))
                    .add(entity::user::Column::Pass.eq(pass)),
            )
            .one(self.read_db())
            .await?
        {
            Some(user) => {
                self.user = Some(User::new(
                    Arc::from(user.user_id),
                    user.pass,
                    user.rango.as_str(),
                ));
                Ok(())
            }
            None => match entity::user::Entity::find()
                .filter(entity::user::Column::UserId.eq(id))
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
    pub async fn pes_filtrado(
        &self,
        filtro: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<(f32, Pesable)>, AppError> {
        let (cant, filtro) = Sistema::splitx(filtro)?;
        let mut prods = Vec::new();
        match filtro.parse::<i64>() {
            Ok(code) => {
                if let Some(model) = entity::pesable::Entity::find()
                    .filter(entity::pesable::Column::Codigo.eq(code))
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
                        res = entity::pesable::Entity::find()
                            .filter(entity::pesable::Column::Descripcion.contains(filtros[i]))
                            .order_by_asc(entity::pesable::Column::Id)
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
                if let Some(model) = entity::rubro::Entity::find()
                    .filter(entity::rubro::Column::Codigo.eq(code))
                    .one(db)
                    .await?
                {
                    prods.push((cant as u8, Mapper::map_model_rub(&model, cant as f64)))
                }
            }
            Err(_) => {
                let filtros = filtro.split(' ').collect::<Vec<&str>>();
                let mut res = Vec::new();
                for i in 0..filtros.len() {
                    if i == 0 {
                        res = entity::rubro::Entity::find()
                            .filter(entity::rubro::Column::Descripcion.contains(filtros[i]))
                            .order_by_asc(entity::rubro::Column::Id)
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
                    prods.push((cant as u8, Mapper::map_model_rub(model, cant as f64)));
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
                if let Some(id) = entity::codigo_barras::Entity::find()
                    .filter(entity::codigo_barras::Column::Codigo.eq(code))
                    .one(db)
                    .await?
                {
                    prods.push({
                        let model = entity::producto::Entity::find_by_id(id.producto)
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
                        res = entity::producto::Entity::find()
                            .filter(
                                Condition::any()
                                    .add(entity::producto::Column::Marca.contains(filtros[i]))
                                    .add(
                                        entity::producto::Column::TipoProducto.contains(filtros[i]),
                                    )
                                    .add(entity::producto::Column::Variedad.contains(filtros[i])),
                            )
                            .order_by_asc(entity::producto::Column::Id)
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
        match entity::proveedor::Entity::find().all(self.read_db()).await {
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
                res = Ok(self.ventas.0.agregar_pago(medio_pago, monto));
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
                res = Ok(self.ventas.1.agregar_pago(medio_pago, monto));
            }
        }

        if let Ok(a) = res {
            if a <= 0.0 {
                self.cerrar_venta(pos)?
            }
        }
        res
    }
    pub fn eliminar_pago(&mut self, pos: bool, index: usize) -> Res<Venta> {
        let res;
        if pos {
            self.ventas.0.eliminar_pago(index);
            res = self.ventas.0.clone()
        } else {
            self.ventas.1.eliminar_pago(index);
            res = self.ventas.1.clone()
        }

        Ok(res)
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
        async_runtime::block_on(async {
            let mut res = entity::config::Entity::find()
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

    fn proveedor_esta(&self, proveedor: &str) -> bool {
        let mut res = false;
        for i in &self.proveedores {
            if i.nombre().eq_ignore_ascii_case(proveedor) {
                res = true;
            }
        }
        res
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
        let prod_model = producto::ActiveModel {
            precio_de_venta: Set(precio_de_venta),
            porcentaje: Set(porcentaje),
            precio_de_costo: Set(precio_de_costo),
            tipo_producto: Set(tipo_producto.to_string()),
            marca: Set(marca.to_owned()),
            variedad: Set(variedad.to_owned()),
            presentacion: Set(presentacion.to_string()),
            updated_at: Set(Utc::now().naive_local()),
            ..Default::default()
        };
        let res_prod = entity::producto::Entity::insert(prod_model)
            .exec(self.write_db())
            .await?;
        let codigos_model: Vec<codigo_barras::ActiveModel> = codigos_de_barras
            .iter()
            .map(|x| codigo_barras::ActiveModel {
                codigo: Set(*x),
                producto: Set(res_prod.last_insert_id),
                ..Default::default()
            })
            .collect();

        entity::codigo_barras::Entity::insert_many(codigos_model)
            .exec(self.write_db())
            .await?;
        for i in 0..codigos_prov.len() {
            let codigo = if codigos_prov[i].len() == 0 {
                None
            } else {
                Some(codigos_prov[i].parse::<i64>()?)
            };
            if let Some(prov) = entity::proveedor::Entity::find()
                .filter(Condition::all().add(entity::proveedor::Column::Nombre.eq(proveedores[i])))
                .one(self.write_db())
                .await?
            {
                let relacion_model = relacion_prod_prov::ActiveModel {
                    producto: Set(res_prod.last_insert_id),
                    proveedor: Set(prov.id),
                    codigo: Set(codigo),
                    ..Default::default()
                };
                entity::relacion_prod_prov::Entity::insert(relacion_model)
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
    pub fn agregar_pesable(&mut self, pesable: Pesable) -> Res<()> {
        // let mut pesables: Vec<Pesable> = self
        //     .productos
        //     .iter()
        //     .map(|x| match x {
        //         V::Pes(a) => Some(a.1.clone()),
        //         _ => None,
        //     })
        //     .flatten()
        //     .collect();
        // pesables.push(pesable.clone());
        // crear_file(&self.path_pesables, &pesables)?;
        let handle = async_runtime::spawn(save(pesable.clone()));
        // self.productos.push(V::Pes((0.0, pesable)));
        Ok(async_runtime::block_on(handle)??)
    }

    pub fn agregar_rubro(&mut self, rubro: Rubro) -> Res<()> {
        // let mut rubros: Vec<Rubro> = self
        //     .productos
        //     .iter()
        //     .map(|x| match x {
        //         V::Rub(a) => Some(a.1.clone()),
        //         _ => None,
        //     })
        //     .flatten()
        //     .collect();
        // rubros.push(rubro.clone());
        let handle = async_runtime::spawn(save(rubro.clone()));
        // crear_file(&self.path_rubros, &rubros)?;
        // self.productos.push(V::Rub((0, rubro)));
        Ok(async_runtime::block_on(handle)??)
    }
    pub fn agregar_proveedor(&mut self, proveedor: &str, contacto: &str) -> Res<()> {
        let handle;
        if self.proveedor_esta(&proveedor) {
            return Err(AppError::ExistingProviderError(proveedor.to_string()));
        } else {
            let prov;
            if contacto.len() > 0 {
                let contacto: String = contacto
                    .chars()
                    .filter(|x| -> bool { x.is_numeric() })
                    .collect();
                let contacto = Some(contacto.parse()?);
                let proveedor = proveedor.to_lowercase();
                prov = Proveedor::new(
                    self.proveedores.len() as i64 + 1,
                    proveedor.as_str(),
                    contacto,
                );
            } else {
                prov = Proveedor::new(self.proveedores.len() as i64 + 1, proveedor, None);
            }
            handle = async_runtime::spawn(save(prov.clone()));
            self.proveedores.push(prov);
        }
        Ok(async_runtime::block_on(handle)??)
    }
    async fn producto(&mut self, id: i64) -> Result<Valuable, AppError> {
        let model;

        match entity::producto::Entity::find_by_id(id)
            .one(self.read_db())
            .await?
        {
            Some(a) => {
                model = a.to_owned();

                return Ok(V::Prod((
                    0,
                    Mapper::map_model_prod(&model, self.read_db()).await?,
                )));
            }
            None => {
                return Err(AppError::ProductNotFound(format!(
                    "No encontrado el producto id {id}"
                )));
            }
        }
    }
    pub async fn agregar_producto_a_venta(&mut self, prod: V, pos: bool) -> Res<Venta> {
        let existe = match &prod {
            Valuable::Prod(a) => entity::producto::Entity::find_by_id(*a.1.id())
                .one(self.read_db())
                .await?
                .is_some(),
            Valuable::Pes(a) => entity::pesable::Entity::find_by_id(*a.1.id())
                .one(self.read_db())
                .await?
                .is_some(),
            Valuable::Rub(a) => entity::rubro::Entity::find_by_id(*a.1.id())
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
            return Err(AppError::ProductNotFound(String::from(
                "Producto inexistente",
            )));
        }

        result
    }
    pub fn descontar_producto_de_venta(&mut self, id: i64, pos: bool) -> Result<Venta, AppError> {
        let res = async_runtime::block_on(self.producto(id))?;
        Ok(if pos {
            self.ventas
                .0
                .restar_producto(res, &self.configs().politica(), &self.configs)?
        } else {
            self.ventas
                .1
                .restar_producto(res, &self.configs().politica(), &self.configs)?
        })
    }
    pub fn incrementar_producto_a_venta(&mut self, id: i64, pos: bool) -> Result<Venta, AppError> {
        let res = async_runtime::block_on(self.producto(id))?;
        let result;
        if pos {
            result =
                self.ventas
                    .0
                    .incrementar_producto(res, &self.configs().politica(), &self.configs);
        } else {
            result =
                self.ventas
                    .1
                    .incrementar_producto(res, &self.configs().politica(), &self.configs);
        }

        result
    }
    pub fn eliminar_producto_de_venta(&mut self, id: i64, pos: bool) -> Result<Venta, AppError> {
        let res = async_runtime::block_on(self.producto(id))?;
        let result;
        if pos {
            if self.ventas.0.productos().len() > 1 {
                result =
                    self.ventas
                        .0
                        .eliminar_producto(res, &self.configs().politica(), &self.configs);
            } else {
                self.ventas.0 = async_runtime::block_on(Venta::new(
                    Arc::clone(&self.vendedor),
                    self.write_db(),
                ))?;
                result = Ok(self.ventas.0.clone());
            }
        } else {
            if self.ventas.1.productos().len() > 1 {
                result =
                    self.ventas
                        .1
                        .eliminar_producto(res, &self.configs().politica(), &self.configs);
            } else {
                self.ventas.1 = async_runtime::block_on(Venta::new(
                    Arc::clone(&self.vendedor),
                    self.write_db(),
                ))?;
                result = Ok(self.ventas.1.clone());
            }
        }

        result
    }
    pub fn venta(&self, pos: bool) -> Venta {
        let res;
        if pos {
            res = self.ventas.0.clone();
        } else {
            res = self.ventas.1.clone();
        }
        res
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Res<Vec<String>> {
        let mut hash = HashSet::new();
        async_runtime::block_on(async {
            entity::producto::Entity::find()
                .filter(entity::producto::Column::Marca.contains(filtro))
                .order_by(entity::producto::Column::Marca, sea_orm::Order::Asc)
                .all(self.read_db())
                .await?
                .iter()
                .for_each(|x| {
                    hash.insert(x.marca.clone());
                });
            Ok(hash.into_iter().collect::<Vec<String>>())
        })
    }

    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Res<Vec<String>> {
        let mut hash = HashSet::new();
        async_runtime::block_on(async {
            entity::producto::Entity::find()
                .filter(entity::producto::Column::TipoProducto.contains(filtro))
                .order_by(entity::producto::Column::TipoProducto, sea_orm::Order::Asc)
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
    fn cerrar_venta(&mut self, pos: bool) -> Res<()> {
        if pos {
            async_runtime::spawn(save(self.ventas.0.clone()));
            self.registro.push(self.ventas.0.clone());
            async_runtime::block_on(self.update_total(self.ventas.0.monto_total()))?;
            self.ventas.0 =
                async_runtime::block_on(Venta::new(Arc::clone(&self.vendedor), self.write_db()))?;
        } else {
            async_runtime::spawn(save(self.ventas.1.clone()));
            self.registro.push(self.ventas.1.clone());
            async_runtime::block_on(self.update_total(self.ventas.1.monto_total()))?;
            self.ventas.1 =
                async_runtime::block_on(Venta::new(Arc::clone(&self.vendedor), self.write_db()))?;
        };

        Ok(())
    }
    pub fn stash_sale(&mut self, pos: bool) -> Res<()> {
        if pos {
            self.stash.push(self.ventas.0.clone());
            self.ventas.0 =
                async_runtime::block_on(Venta::new(Arc::clone(&self.vendedor), self.write_db()))?;
        } else {
            self.stash.push(self.ventas.1.clone());
            self.ventas.1 =
                async_runtime::block_on(Venta::new(Arc::clone(&self.vendedor), self.write_db()))?;
        };
        Ok(())
    }
    pub fn unstash_sale(&mut self, pos: bool, index: usize) -> Res<()> {
        if index < self.stash.len() {
            if pos {
                if self.ventas.0.productos().len() > 0 {
                    self.stash.push(self.ventas.0.to_owned());
                }
                self.ventas.0 = self.stash.remove(index);
                Ok(())
            } else {
                if self.ventas.1.productos().len() > 0 {
                    self.stash.push(self.ventas.1.to_owned());
                }
                self.ventas.1 = self.stash.remove(index);
                Ok(())
            }
        } else {
            Err(AppError::SaleSelection.into())
        }
    }
    pub fn stash(&self) -> &Vec<Venta> {
        &self.stash
    }
    pub async fn update_total(&mut self, monto: f64) -> Result<(), AppError> {
        self.caja.update_total(&self.write_db, monto).await
    }
}
