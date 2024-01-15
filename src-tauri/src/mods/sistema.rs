type Res<T> = std::result::Result<T, AppError>;
use chrono::Utc;
use entity::codigo_barras;
use entity::*;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::{Database, EntityTrait};
use tauri::async_runtime;

use super::error::AppError;
use super::lib::camalize;

use super::lib::cargar_todas_las_relaciones_prod_prov;
use super::lib::cargar_todos_los_provs;
use super::lib::cargar_todos_los_valuables;
use super::lib::save;
use super::lib::update_data_provs;
use super::lib::update_data_valuable;
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
#[derive(Clone)]
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
    pub fn new() -> Res<Sistema> {
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
        let mut proveedores: Vec<Proveedor> = Vec::new();
        leer_file(&mut proveedores, path_proveedores)?;
        let hay_cambios_desde_db = async_runtime::block_on(update_data_valuable(
            &mut rubros,
            &mut pesables,
            &mut productos,
            path_rubros,
            path_pesables,
            path_productos,
        ))?;
        let hay_cambios_provs_db =
            async_runtime::block_on(update_data_provs(&mut proveedores, path_proveedores))?;

        let mut rubros_valuable: Vec<Valuable> = rubros
            .iter()
            .map(|a| Valuable::Rub((0, a.to_owned())))
            .collect();
        let mut pesables_valuable: Vec<Valuable> = pesables
            .iter()
            .map(|a| Valuable::Pes((0.0, a.to_owned())))
            .collect();
        let mut valuables: Vec<Valuable> = productos
            .clone()
            .iter()
            .map(|a| Valuable::Prod((0, a.to_owned())))
            .collect();
        valuables.append(&mut pesables_valuable);
        valuables.append(&mut rubros_valuable);

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
            productos: valuables,
            ventas: (Venta::new(), Venta::new()),
            proveedores: proveedores.clone(),
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
        for i in 0..sis.productos.len() {
            sis.productos[i].unifica_codes()
        }
        let prov_load_handle =
            async_runtime::spawn(cargar_todos_los_provs(sis.proveedores.clone()));
        let prod_load_handle =
            async_runtime::spawn(cargar_todos_los_valuables(sis.productos.clone()));
        let rel_load_handle = async_runtime::spawn(cargar_todas_las_relaciones_prod_prov(
            sis.relaciones.clone(),
        ));
        async_runtime::block_on(prov_load_handle)??;
        async_runtime::block_on(prod_load_handle)??;
        async_runtime::block_on(rel_load_handle)??;
        if hay_cambios_desde_db {
            crear_file(path_pesables, &pesables)?;
            crear_file(path_productos, &productos)?;
            crear_file(path_rubros, &rubros)?;
        }
        if hay_cambios_provs_db {
            crear_file(path_proveedores, &proveedores)?;
        }
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
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f64, pos: usize) -> Res<f64> {
        let res;
        match pos {
            0 => {
                if !medio_pago.eq("Efectivo")
                    && self.ventas.0.get_monto_pagado() + monto > self.ventas.0.get_monto_total()
                {
                    return Err(AppError::AmountError {
                        a_pagar: self.ventas.0.get_monto_total() - self.ventas.0.get_monto_pagado(),
                        pagado: monto,
                    });
                } else {
                    res = Ok(self.ventas.0.agregar_pago(medio_pago, monto));
                }
            }
            1 => {
                if !medio_pago.eq("Efectivo")
                    && self.ventas.1.get_monto_pagado() + monto > self.ventas.1.get_monto_total()
                {
                    return Err(AppError::AmountError {
                        a_pagar: self.ventas.1.get_monto_total() - self.ventas.1.get_monto_pagado(),
                        pagado: monto,
                    });
                } else {
                    res = Ok(self.ventas.1.agregar_pago(medio_pago, monto));
                }
            }
            _ => return Err(AppError::SaleSelection),
        }
        if let Ok(a) = res {
            if a <= 0.0 {
                self.cerrar_venta(pos)?
            }
        }
        res
    }
    pub fn eliminar_pago(&mut self, pos: usize, index: usize) -> Res<()> {
        match pos {
            0 => self.ventas.0.eliminar_pago(index),
            1 => self.ventas.1.eliminar_pago(index),
            _ => return Err(AppError::SaleSelection.into()),
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
    ) -> Res<Producto> {
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
                .filter(Condition::all().add(entity::proveedor::Column::Nombre.eq(proveedores[i])))
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
    pub fn agregar_pesable(&mut self, pesable: Pesable) -> Res<()> {
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
        let handle = async_runtime::spawn(save(pesable.clone()));
        self.productos.push(Valuable::Pes((0.0, pesable)));
        Ok(async_runtime::block_on(handle)??)
    }

    pub fn agregar_rubro(&mut self, rubro: Rubro) -> Res<()> {
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
        let handle = async_runtime::spawn(save(rubro.clone()));
        crear_file(&self.path_rubros, &rubros)?;
        self.productos.push(Valuable::Rub((0, rubro)));
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
            handle = async_runtime::spawn(save(prov.clone()));
            self.proveedores.push(prov);
            crear_file(&self.path_proveedores, &self.proveedores)?;
        }
        Ok(async_runtime::block_on(handle)??)
    }
    fn get_producto(&mut self, id: i64) -> Result<Valuable, AppError> {
        let mut res = Err(AppError::ProductNotFound(id.to_string()));
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
    pub fn agregar_producto_a_venta(&mut self, id: i64, pos: usize) -> Res<Venta> {
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
            _ => result = Err(AppError::SaleSelection.into()),
        }

        result
    }
    pub fn descontar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<Venta, AppError> {
        let res = self.get_producto(id)?;
        Ok(match pos {
            0 => self.ventas.0.restar_producto(
                res,
                self.get_configs().get_politica(),
                &self.configs,
            )?,
            1 => self.ventas.1.restar_producto(
                res,
                self.get_configs().get_politica(),
                &self.configs,
            )?,
            _ => return Err(AppError::SaleSelection.into()),
        })
    }
    pub fn incrementar_producto_a_venta(&mut self, id: i64, pos: usize) -> Result<Venta, AppError> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self.ventas.0.incrementar_producto(
                    res,
                    self.get_configs().get_politica(),
                    &self.configs,
                );
            }
            1 => {
                result = self.ventas.1.incrementar_producto(
                    res,
                    self.get_configs().get_politica(),
                    &self.configs,
                );
            }
            _ => result = Err(AppError::SaleSelection),
        }
        result
    }
    pub fn eliminar_producto_de_venta(&mut self, id: i64, pos: usize) -> Result<Venta, AppError> {
        let res = self.get_producto(id)?;
        let result;
        match pos {
            0 => {
                result = self.ventas.0.eliminar_producto(
                    res,
                    self.get_configs().get_politica(),
                    &self.configs,
                );
            }
            1 => {
                result = self.ventas.1.eliminar_producto(
                    res,
                    self.get_configs().get_politica(),
                    &self.configs,
                );
            }
            _ => result = Err(AppError::SaleSelection),
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
    fn cerrar_venta(&mut self, pos: usize) -> Res<()> {
        let handle;
        match pos {
            0 => {
                handle = async_runtime::spawn(save(self.ventas.0.clone()));
                self.registro.push(self.ventas.0.clone());
                self.ventas.0 = Venta::new();
            }
            1 => {
                handle = async_runtime::spawn(save(self.ventas.1.clone()));
                self.registro.push(self.ventas.1.clone());
                self.ventas.1 = Venta::new();
            }
            _ => return Err(AppError::SaleSelection.into()),
        };

        Ok(async_runtime::block_on(handle)??)
    }
    pub fn stash_sale(&mut self, pos: usize) -> Res<()> {
        match pos {
            0 => {
                self.stash.push(self.ventas.0.clone());
                self.ventas.0 = Venta::new();
            }
            1 => {
                self.stash.push(self.ventas.1.clone());
                self.ventas.1 = Venta::new();
            }
            _ => return Err(AppError::SaleSelection.into()),
        };
        Ok(())
    }
    pub fn unstash_sale(&mut self, pos: usize, index: usize) -> Res<()> {
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
                _ => Err(AppError::SaleSelection.into()),
            }
        } else {
            Err(AppError::SaleSelection.into())
        }
    }
    pub fn get_stash(&self) -> &Vec<Venta> {
        &self.stash
    }
}
