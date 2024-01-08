type Result<T> = std::result::Result<T, Box<dyn Error>>;
use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct SaleSelectionError;
impl fmt::Display for SaleSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error, hay solo dos posiciones para ventas")
    }
}

#[derive(Debug)]
pub struct ProductNotFoundError;
impl fmt::Display for ProductNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error, producto no encontrado")
    }
}
#[derive(Debug)]
pub struct ExistingProviderError;
impl fmt::Display for ExistingProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error, proveedor existente")
    }
}
#[derive(Debug)]
pub struct AmountError;
impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "El monto no puede ser superior al resto con el medio de pago actual"
        )
    }
}
#[derive(Debug)]
pub struct SizeSelecionError;
impl fmt::Display for SizeSelecionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error, las presentaciones habilitadas son: Gr Un Lt Ml CC Kg"
        )
    }
}
impl std::error::Error for SaleSelectionError {}
impl std::error::Error for ExistingProviderError {}
impl std::error::Error for SizeSelecionError {}
impl std::error::Error for AmountError {}
impl std::error::Error for ProductNotFoundError {}

use chrono::Utc;
use entity::codigo_barras;
use entity::*;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tauri::async_runtime;
use tauri::async_runtime::block_on;

use super::lib::camalize;
use super::lib::get_updated_time_db_pesable;
use super::lib::get_updated_time_db_producto;
use super::lib::get_updated_time_db_rubro;
use super::lib::get_updated_time_file;
use super::proveedor::Proveedor;
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
    configs: Config,
    productos: Vec<Valuable>,
    ventas: (Venta, Venta),
    proveedores: Vec<Proveedor>,
    path_productos: String,
    path_proveedores: String,
    path_relaciones: String,
    path_configs: String,
    path_pesables: String,
    path_rubros: String,
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}

impl<'a> Sistema {
    pub fn new() -> Result<Sistema> {
        let path_productos = "Productos.json";
        let path_proveedores = "Proveedores.json";
        let path_relaciones = "Relaciones.json";
        let path_configs = "Configs.json";
        let path_rubros = "Rubros.json";
        let path_pesables = "Pesables.json";
        let mut productos: Vec<Producto> = Vec::new();
        let mut rubros: Vec<Rubro> = Vec::new();
        let mut pesables: Vec<Pesable> = Vec::new();
        let stash = Vec::new();
        let registro = Vec::new();
        leer_file(&mut rubros, path_rubros)?;
        leer_file(&mut pesables, path_pesables)?;
        leer_file(&mut productos, path_productos)?;

        let mut rubros: Vec<Valuable> = rubros
            .iter()
            .map(|a| Valuable::Rub((0, a.to_owned())))
            .collect();
        let mut pesables: Vec<Valuable> = pesables
            .iter()
            .map(|a| Valuable::Pes((0.0, a.to_owned())))
            .collect();
        let mut valuables: Vec<Valuable> = productos
            .clone()
            .iter()
            .map(|a| Valuable::Prod((0, a.to_owned())))
            .collect();
        valuables.append(&mut pesables);
        valuables.append(&mut rubros);

        let mut proveedores: Vec<Proveedor> = Vec::new();
        leer_file(&mut proveedores, path_proveedores)?;
        let mut relaciones = Vec::new();
        leer_file(&mut relaciones, path_relaciones)?;
        let mut configs = Vec::<Config>::new();
        leer_file(&mut configs, path_configs)?;
        if configs.len() == 0 {
            configs.push(Config::default());
            crear_file(path_configs, &mut configs)?;
        }

        let mut sis = Sistema {
            configs: configs[0].clone(),
            productos: valuables.clone(),
            ventas: (Venta::new(), Venta::new()),
            proveedores,
            path_productos: path_productos.to_string(),
            path_proveedores: path_proveedores.to_string(),
            path_relaciones: path_relaciones.to_string(),
            path_configs: path_configs.to_string(),
            path_pesables: path_pesables.to_string(),
            path_rubros: path_rubros.to_string(),
            relaciones,
            stash,
            registro,
        };
        // for i in 0..sis.productos.len() {
        // sis.productos[i].unifica_codes()
        // }
        async_runtime::block_on(sis.update_data_valuable())?;
        async_runtime::block_on(sis.cargar_todos_los_provs())?;
        async_runtime::block_on(sis.cargar_todos_los_valuables())?;
        async_runtime::block_on(sis.cargar_todas_las_relaciones_prod_prov())?;
        Ok(sis)
    }
    pub fn get_productos(&self) -> &Vec<Valuable> {
        &self.productos
    }
    pub fn get_productos_cloned(&self) -> Vec<Valuable> {
        self.productos.clone()
    }
    pub fn get_proveedores(&self) -> &Vec<Proveedor> {
        &self.proveedores
    }
    pub fn get_configs(&self) -> &Config {
        &self.configs
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f64, pos: usize) -> Result<f64> {
        let res;
        match pos {
            0 => {
                if !medio_pago.eq("Efectivo")
                    && self.ventas.0.get_monto_pagado() + monto > self.ventas.0.get_monto_total()
                {
                    return Err(AmountError.into());
                } else {
                    res = Ok(self.ventas.0.agregar_pago(medio_pago, monto));
                }
            }
            1 => {
                if !medio_pago.eq("Efectivo")
                    && self.ventas.1.get_monto_pagado() + monto > self.ventas.1.get_monto_total()
                {
                    return Err(AmountError.into());
                } else {
                    res = Ok(self.ventas.1.agregar_pago(medio_pago, monto));
                }
            }
            _ => return Err(SaleSelectionError.into()),
        }
        if let Ok(a) = res {
            if a <= 0.0 {
                self.cerrar_venta(pos)?
            }
        }
        res
    }
    pub fn eliminar_pago(&mut self, pos: usize, index: usize) -> Result<()> {
        match pos {
            0 => self.ventas.0.eliminar_pago(index),
            1 => self.ventas.1.eliminar_pago(index),
            _ => return Err(SaleSelectionError.into()),
        }
        Ok(())
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
        if let Err(e) = crear_file(&self.path_configs, &vec![&self.configs]) {
            panic!("{e}");
        }
    }
    pub fn imprimir(&self) {
        println!("Printed from rust");
    }
    fn proveedor_esta(&self, proveedor: &str) -> bool {
        let mut res = false;
        for i in &self.proveedores {
            if i.get_nombre().eq_ignore_ascii_case(proveedor) {
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
    ) -> Result<Producto> {
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;

        let tipo_producto = camalize(tipo_producto);
        let marca = camalize(marca);
        let variedad = camalize(variedad);
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
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        let res_prod = entity::producto::Entity::insert(prod_model)
            .exec(&db)
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
            .exec(&db)
            .await?;
        for i in 0..codigos_prov.len() {
            let codigo = if codigos_prov[i].len() == 0 {
                None
            } else {
                Some(codigos_prov[i].parse::<i64>()?)
            };
            if let Some(prov) = entity::proveedor::Entity::find()
                .filter(
                    Condition::all()
                        .add(entity::proveedor::Column::Nombre.eq(proveedores[i].clone())),
                )
                .one(&db)
                .await?
            {
                let relacion_model = relacion_prod_prov::ActiveModel {
                    producto: Set(res_prod.last_insert_id),
                    proveedor: Set(prov.id),
                    codigo: Set(codigo),
                    ..Default::default()
                };
                entity::relacion_prod_prov::Entity::insert(relacion_model)
                    .exec(&db)
                    .await?;
            }
        }

        let producto = Producto::new(
            res_prod.last_insert_id,
            codigos_de_barras,
            precio_de_venta,
            porcentaje,
            precio_de_costo,
            tipo_producto,
            marca,
            variedad,
            presentacion,
        );

        let result = Ok(producto.clone());

        let mut productos: Vec<Producto> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Prod(a) => Some(a.1.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        for i in 0..proveedores.len() {
            match codigos_prov[i].parse::<i64>() {
                Ok(a) => self.relaciones.push(RelacionProdProv::new(
                    self.productos.len() as i64,
                    i as i64,
                    Some(a),
                )),
                Err(_) => self.relaciones.push(RelacionProdProv::new(
                    self.productos.len() as i64,
                    i as i64,
                    None,
                )),
            };
        }
        productos.push(producto.clone());
        crear_file(&self.path_productos, &productos)?;
        crear_file(&self.path_relaciones, &self.relaciones)?;

        self.productos.push(Valuable::Prod((0, producto)));

        result
    }
    pub fn agregar_pesable(&mut self, pesable: Pesable) -> Result<()> {
        let mut pesables: Vec<Pesable> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Pes(a) => Some(a.1.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        pesables.push(pesable.clone());
        crear_file(&self.path_pesables, &pesables)?;
        async_runtime::block_on(pesable.clone().save())?;
        self.productos.push(Valuable::Pes((0.0, pesable)));
        Ok(())
    }

    pub fn agregar_rubro(&mut self, rubro: Rubro) -> Result<()> {
        let mut rubros: Vec<Rubro> = self
            .productos
            .iter()
            .map(|x| match x {
                Valuable::Rub(a) => Some(a.1.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        rubros.push(rubro.clone());
        crear_file(&self.path_rubros, &rubros)?;
        async_runtime::block_on(rubro.clone().save())?;
        self.productos.push(Valuable::Rub((0, rubro)));
        Ok(())
    }
    pub fn agregar_proveedor(&mut self, proveedor: &str, contacto: &str) -> Result<()> {
        if self.proveedor_esta(&proveedor) {
            return Err(ExistingProviderError.into());
        } else {
            let prov;
            if contacto.len() > 0 {
                let contacto: String = contacto
                    .chars()
                    .filter(|x| -> bool { x.is_numeric() })
                    .collect();
                let contacto = Some(contacto.parse()?);
                prov = Proveedor::new(
                    self.proveedores.len() as i64 + 1,
                    camalize(proveedor),
                    contacto,
                );
            } else {
                prov = Proveedor::new(
                    self.proveedores.len() as i64 + 1,
                    proveedor.to_string(),
                    None,
                );
            }
            async_runtime::block_on(prov.save())?;
            self.proveedores.push(prov);
            crear_file(&self.path_proveedores, &self.proveedores)?;
        }
        Ok(())
    }
    fn get_producto(&mut self, id: i64) -> Result<Valuable> {
        let mut res = Err(ProductNotFoundError.into());
        for p in &self.productos {
            match p {
                Valuable::Pes(a) => {
                    if a.1.id == id {
                        res = Ok(p.clone());
                    }
                }
                Valuable::Prod(a) => {
                    if a.1.get_id() == id {
                        res = Ok(p.clone());
                    }
                }
                Valuable::Rub(a) => {
                    if a.1.id == id {
                        res = Ok(p.clone());
                    }
                }
            }
        }
        res
    }
    pub fn agregar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<Venta> {
        let res = self
            .get_producto(id)?
            .redondear(self.get_configs().get_politica());
        let result;
        match pos {
            0 => {
                result = Ok(self
                    .ventas
                    .0
                    .agregar_producto(res, self.get_configs().get_politica()))
            }
            1 => {
                result = Ok(self
                    .ventas
                    .1
                    .agregar_producto(res, self.get_configs().get_politica()))
            }
            _ => result = Err(SaleSelectionError.into()),
        }

        result
    }
    pub fn descontar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<Venta> {
        let res = self.get_producto(id)?;
        match pos {
            0 => self
                .ventas
                .0
                .restar_producto(res, self.get_configs().get_politica()),
            1 => self
                .ventas
                .1
                .restar_producto(res, self.get_configs().get_politica()),
            _ => Err(SaleSelectionError.into()),
        }
    }
    pub fn incrementar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<Venta> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self
                    .ventas
                    .0
                    .incrementar_producto(res, self.get_configs().get_politica());
            }
            1 => {
                result = self
                    .ventas
                    .1
                    .incrementar_producto(res, self.get_configs().get_politica());
            }
            _ => result = Err(SaleSelectionError.into()),
        }
        result
    }
    pub fn eliminar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<Venta> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self
                    .ventas
                    .0
                    .eliminar_producto(res, self.get_configs().get_politica());
            }
            1 => {
                result = self
                    .ventas
                    .1
                    .eliminar_producto(res, self.get_configs().get_politica());
            }
            _ => result = Err(SaleSelectionError.into()),
        }
        result
    }
    pub fn get_venta(&self, pos: usize) -> Venta {
        let res;
        if pos == 0 {
            res = self.ventas.0.clone();
        } else {
            res = self.ventas.1.clone();
        }
        res
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Vec<String> {
        let iter = self.productos.iter();
        let mut res: Vec<String> = iter
            .filter_map(|x| match x {
                Valuable::Prod(a) => {
                    if a.1.marca.to_lowercase().contains(&filtro.to_lowercase()) {
                        Some(a.1.marca.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        res.sort();
        res.dedup();

        res
    }

    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Vec<String> {
        let iter = self.productos.iter();
        let mut res: Vec<String> = iter
            .filter_map(|x| match x {
                Valuable::Prod(a) => {
                    if a.1
                        .tipo_producto
                        .to_lowercase()
                        .contains(&filtro.to_lowercase())
                    {
                        Some(a.1.tipo_producto.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        res.sort();
        res.dedup();
        res
    }
    fn cerrar_venta(&mut self, pos: usize) -> Result<()> {
        match pos {
            0 => {
                block_on(self.ventas.0.save())?;
                self.registro.push(self.ventas.0.clone());
                self.ventas.0 = Venta::new();
            }
            1 => {
                block_on(self.ventas.1.save())?;
                self.registro.push(self.ventas.1.clone());
                self.ventas.1 = Venta::new();
            }
            _ => return Err(SaleSelectionError.into()),
        };
        Ok(())
    }
    pub fn stash_sale(&mut self, pos: usize) -> Result<()> {
        match pos {
            0 => {
                self.stash.push(self.ventas.0.clone());
                self.ventas.0 = Venta::new();
            }
            1 => {
                self.stash.push(self.ventas.1.clone());
                self.ventas.1 = Venta::new();
            }
            _ => return Err(SaleSelectionError.into()),
        };
        Ok(())
    }
    pub fn unstash_sale(&mut self, pos: usize, index: usize) -> Result<()> {
        if index < self.stash.len() {
            match pos {
                0 => {
                    if self.ventas.0.get_productos().len() > 0 {
                        self.stash.push(self.ventas.0.to_owned());
                    }
                    self.ventas.0 = self.stash.remove(index);
                    Ok(())
                }
                1 => {
                    if self.ventas.1.get_productos().len() > 0 {
                        self.stash.push(self.ventas.1.to_owned());
                    }
                    self.ventas.1 = self.stash.remove(index);
                    Ok(())
                }
                _ => Err(SaleSelectionError.into()),
            }
        } else {
            Err(SaleSelectionError.into())
        }
    }
    pub fn get_stash(&self) -> &Vec<Venta> {
        &self.stash
    }
    pub async fn cargar_todos_los_valuables(&self) -> Result<()> {
        println!("Guardando productos en DB");
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        self.cargar_todos_los_productos(&db).await?;
        self.cargar_todos_los_pesables(&db).await?;
        self.cargar_todos_los_rubros(&db).await?;
        Ok(())
    }
    async fn cargar_todos_los_productos(&self, db: &DatabaseConnection) -> Result<()> {
        for producto in &self.productos {
            match producto {
                Valuable::Prod(a) => {
                    let prod_model = producto::ActiveModel {
                        precio_de_venta: Set(a.1.precio_de_venta),
                        porcentaje: Set(a.1.porcentaje),
                        precio_de_costo: Set(a.1.precio_de_costo),
                        tipo_producto: Set(a.1.tipo_producto.clone()),
                        marca: Set(a.1.marca.clone()),
                        variedad: Set(a.1.variedad.clone()),
                        presentacion: Set(a.1.presentacion.to_string()),
                        updated_at: Set(Utc::now().naive_utc()),
                        ..Default::default()
                    };
                    let res_prod = entity::producto::Entity::insert(prod_model)
                        .exec(db)
                        .await?;
                    let codigos_model: Vec<codigo_barras::ActiveModel> =
                        a.1.codigos_de_barras
                            .iter()
                            .map(|x| codigo_barras::ActiveModel {
                                codigo: Set(*x),
                                producto: Set(res_prod.last_insert_id),
                                ..Default::default()
                            })
                            .collect();
                    if codigos_model.len() > 1 {
                        entity::codigo_barras::Entity::insert_many(codigos_model)
                            .exec(db)
                            .await?;
                    } else if codigos_model.len() == 1 {
                        entity::codigo_barras::Entity::insert(codigos_model[0].to_owned())
                            .exec(db)
                            .await?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
    async fn cargar_todos_los_pesables(&self, db: &DatabaseConnection) -> Result<()> {
        let pesables: Vec<pesable::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                Valuable::Pes(a) => Some(pesable::ActiveModel {
                    codigo: Set(a.1.codigo),
                    precio_peso: Set(a.1.precio_peso),
                    porcentaje: Set(a.1.porcentaje),
                    costo_kilo: Set(a.1.costo_kilo),
                    descripcion: Set(a.1.descripcion.clone()),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        if pesables.len() > 1 {
            entity::pesable::Entity::insert_many(pesables)
                .exec(db)
                .await?;
        } else if pesables.len() == 1 {
            entity::pesable::Entity::insert(pesables[0].to_owned())
                .exec(db)
                .await?;
        }
        Ok(())
    }
    async fn cargar_todos_los_rubros(&self, db: &DatabaseConnection) -> Result<()> {
        let rubros: Vec<rubro::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                Valuable::Rub(a) => Some(rubro::ActiveModel {
                    monto: Set(a.1.monto),
                    descripcion: Set(a.1.descripcion.clone()),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        if rubros.len() > 1 {
            entity::rubro::Entity::insert_many(rubros).exec(db).await?;
        } else if rubros.len() == 1 {
            entity::rubro::Entity::insert(rubros[0].to_owned())
                .exec(db)
                .await?;
        }
        Ok(())
    }
    async fn cargar_todas_las_relaciones_prod_prov(&self) -> Result<()> {
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        let mut relaciones_model = Vec::new();
        for x in &self.relaciones {
            let a = entity::producto::Entity::find_by_id(x.get_id_producto())
                .one(&db)
                .await?
                .is_some();
            let b = entity::proveedor::Entity::find_by_id(x.get_id_proveedor())
                .one(&db)
                .await?
                .is_some();
            if a && b {
                relaciones_model.push(entity::relacion_prod_prov::ActiveModel {
                    producto: Set(x.get_id_producto()),
                    proveedor: Set(x.get_id_proveedor()),
                    codigo: Set(x.get_codigo_interno()),
                    ..Default::default()
                })
            }
        }
        entity::relacion_prod_prov::Entity::insert_many(relaciones_model)
            .exec(&db)
            .await?;
        Ok(())
    }
    pub async fn update_data_valuable(&mut self) -> Result<()> {
        let mut prods: Vec<Valuable>;

        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        let updated_prods = self.update_productos_from_db(&db).await?;
        prods = updated_prods
            .iter()
            .map(|x| Valuable::Prod((0, x.clone())))
            .collect();

        let updated_pes = self.update_pesables_from_db(&db).await?;
        prods.append(
            &mut updated_pes
                .iter()
                .map(|x| Valuable::Pes((0.0, x.clone())))
                .collect(),
        );
        let updated_rub = self.update_rubro_from_db(&db).await?;
        prods.append(
            &mut updated_rub
                .iter()
                .map(|x| Valuable::Rub((0, x.clone())))
                .collect(),
        );

        if self.productos.len() != prods.len() {
            self.productos = prods;
        }
        println!("Cantidad de propductos: {}", self.productos.len());

        Ok(())
    }
    pub async fn update_productos_from_db(&self, db: &DatabaseConnection) -> Result<Vec<Producto>> {
        let mut res;
        let prods_db_model = entity::producto::Entity::find().all(db).await?;
        let date_local = get_updated_time_file(&self.path_productos)?;
        let mut date_db = DateTimeUtc::MIN_UTC;
        if prods_db_model.len() > 0 {
            date_db = get_updated_time_db_producto(prods_db_model.clone()).await;
        }
        if date_local > date_db {
            println!("Ultimo actualizado: productos de archivo");
            let productos_db = self
                .productos
                .iter()
                .filter_map(|x| match x {
                    Valuable::Prod(a) => Some(a.1.clone()),
                    _ => None,
                })
                .collect();
            res = productos_db;
        } else {
            res = Vec::new();
            println!("Ultimo actualizado: productos de bases de datos");
            for i in 0..prods_db_model.len() {
                let b = self
                    .get_codigos_db_filtrado(db, prods_db_model[i].id)
                    .await?;
                res.push(map_model_prod(&prods_db_model[i], b)?);
            }
        }
        Ok(res)
    }
    pub async fn update_pesables_from_db(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<super::pesable::Pesable>> {
        let mut res;
        let pesables_db_model = entity::pesable::Entity::find().all(db).await?;
        let mut date_db = DateTimeUtc::MIN_UTC;
        if pesables_db_model.len() > 0 {
            date_db = get_updated_time_db_pesable(pesables_db_model.clone()).await;
        }
        let date_local = get_updated_time_file(&self.path_pesables)?;
        if date_db < date_local {
            let pesables_db: Vec<Pesable> = self
                .productos
                .iter()
                .filter_map(|x| match x {
                    Valuable::Pes(a) => Some(a.1.clone()),
                    _ => None,
                })
                .collect();
            res = pesables_db
        } else {
            res = Vec::new();
            for i in 0..pesables_db_model.len() {
                res.push(Pesable {
                    id: pesables_db_model[i].id,
                    codigo: pesables_db_model[i].codigo,
                    precio_peso: pesables_db_model[i].precio_peso,
                    porcentaje: pesables_db_model[i].porcentaje,
                    costo_kilo: pesables_db_model[i].costo_kilo,
                    descripcion: pesables_db_model[i].descripcion.clone(),
                });
            }
        }
        Ok(res)
    }
    pub async fn update_rubros_from_db(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<super::rubro::Rubro>> {
        let mut res = Vec::new();
        let rubros_db_model = entity::rubro::Entity::find().all(db).await?;
        let mut date_db = DateTimeUtc::MIN_UTC;
        if rubros_db_model.len() > 0 {
            date_db = get_updated_time_db_rubro(rubros_db_model.clone()).await;
        }
        let date_local = get_updated_time_file(&self.path_rubros)?;
        if date_db < date_local {
            let rubros_db: Vec<Rubro> = self
                .productos
                .iter()
                .filter_map(|x| match x {
                    Valuable::Rub(a) => Some(a.1.clone()),
                    _ => None,
                })
                .collect();
            res = rubros_db
        } else {
            for i in 0..rubros_db_model.len() {
                res.push(Rubro {
                    id: rubros_db_model[i].id,
                    monto: rubros_db_model[i].monto,
                    descripcion: rubros_db_model[i].descripcion.clone(),
                });
            }
        }
        Ok(res)
    }
    pub async fn update_rubro_from_db(&self, db: &DatabaseConnection) -> Result<Vec<Rubro>> {
        let a = entity::rubro::Entity::find().all(db).await?;
        Ok(a.iter().map(|x| map_model_rub(x.clone())).collect())
    }
    pub async fn get_codigos_db_filtrado(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<Vec<i64>> {
        let a = entity::codigo_barras::Entity::find()
            .filter(Condition::all().add(entity::codigo_barras::Column::Producto.eq(id)))
            .all(db)
            .await?;
        Ok(a.iter().map(|x| x.id).collect())
    }
    async fn cargar_todos_los_provs(&self) -> Result<()> {
        let db = Database::connect("postgres://postgres:L33tsupa@localhost:5432/Tauri").await?;
        let provs: Vec<proveedor::ActiveModel> = self
            .proveedores
            .iter()
            .map(|x| {
                let contacto = match x.get_contacto() {
                    Some(a) => Some(*a),
                    None => None,
                };
                proveedor::ActiveModel {
                    id: Set(*x.get_id()),
                    nombre: Set(x.get_nombre().clone()),
                    contacto: Set(contacto),
                }
            })
            .collect();
        proveedor::Entity::insert_many(provs).exec(&db).await?;

        Ok(())
    }
}

fn map_model_prod(prod: &entity::producto::Model, cods: Vec<i64>) -> Result<Producto> {
    let mut parts = prod.presentacion.split(' ');
    let p1 = parts.next().unwrap();
    let p2 = parts.next().unwrap();
    let presentacion = match p2 {
        "Gr" => Presentacion::Gr(p1.parse()?),
        "Un" => Presentacion::Un(p1.parse()?),
        "Lt" => Presentacion::Lt(p1.parse()?),
        "Ml" => Presentacion::Ml(p1.parse()?),
        "CC" => Presentacion::CC(p1.parse()?),
        "Kg" => Presentacion::Kg(p1.parse()?),
        _ => return Err(SizeSelecionError.into()),
    };
    Ok(Producto::new(
        prod.id,
        cods,
        prod.precio_de_venta,
        prod.porcentaje,
        prod.precio_de_costo,
        prod.tipo_producto.clone(),
        prod.marca.clone(),
        prod.variedad.clone(),
        presentacion,
    ))
}
fn map_model_rub(rub: entity::rubro::Model) -> Rubro {
    Rubro {
        id: rub.id,
        monto: rub.monto,
        descripcion: rub.descripcion,
    }
}

fn map_model_pes(pes: entity::pesable::Model) -> Pesable {
    Pesable {
        id: pes.id,
        codigo: pes.codigo,
        precio_peso: pes.precio_peso,
        porcentaje: pes.porcentaje,
        costo_kilo: pes.costo_kilo,
        descripcion: pes.descripcion,
    }
}
