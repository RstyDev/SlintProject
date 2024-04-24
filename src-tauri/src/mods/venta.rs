use chrono::Utc;
use entity::prelude::{CliDB, DeudaDB, PagoDB, VentaDB, VentaPesDB, VentaProdDB, VentaRubDB};


use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, Condition, Database, DatabaseConnection, DbErr,
    EntityTrait, IntoActiveModel, IntoSimpleExpr, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::async_runtime;

use Valuable as V;
const CUENTA: &str = "Cuenta Corriente";

use crate::mods::pago::medio_from_db;

use super::{redondeo,Res, AppError, Cli, Cliente, Mapper, MedioPago, Pago, Save, User, Valuable};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Venta {
    id: i32,
    monto_total: f32,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f32,
    vendedor: Option<Arc<User>>,
    cliente: Cliente,
    paga: bool,
    cerrada: bool,
}

impl<'a> Venta {
    pub async fn new(
        vendedor: Option<Arc<User>>,
        db: &DatabaseConnection,
        pos: bool,
    ) -> Res<Venta> {
        let venta = VentaDB::Entity::find()
            .order_by_desc(VentaDB::Column::Id)
            .one(db)
            .await?;
        let id = match venta {
            Some(a) => a.id + 1,
            None => 0,
        };
        let cliente = Cliente::new(None);
        let venta = Venta {
            monto_total: 0.0,
            productos: Vec::new(),
            pagos: Vec::new(),
            monto_pagado: 0.0,
            vendedor,
            id,
            paga: false,
            cliente,
            cerrada: false,
        };
        VentaDB::ActiveModel {
            id: Set(venta.id),
            monto_total: Set(venta.monto_total),
            monto_pagado: Set(venta.monto_pagado),
            time: Set(Utc::now().naive_local()),
            cliente: match &venta.cliente {
                Cliente::Final => NotSet,
                Cliente::Regular(a) => Set(Some(*a.id())),
            },
            cerrada: Set(false),
            paga: Set(false),
            pos: Set(pos),
        }
        .insert(db)
        .await?;
        Ok(venta)
    }
    pub async fn get_or_new(
        vendedor: Option<Arc<User>>,
        db: &DatabaseConnection,
        pos: bool,
    ) -> Res<Venta> {
        match VentaDB::Entity::find()
            .filter(
                Condition::all()
                    .add(VentaDB::Column::Pos.into_simple_expr().eq(pos))
                    .add(VentaDB::Column::Cerrada.into_simple_expr().eq(false)),
            )
            .one(db)
            .await?
        {
            Some(model) => match model.cerrada {
                true => Venta::new(vendedor, db, pos).await,
                false => Mapper::map_model_sale(&model, db, &vendedor).await,
            },
            None => Venta::new(vendedor, db, pos).await,
        }
    }
    pub fn build(
        id: i32,
        monto_total: f32,
        productos: Vec<Valuable>,
        pagos: Vec<Pago>,
        monto_pagado: f32,
        vendedor: Option<Arc<User>>,
        cliente: Cliente,
        paga: bool,
        cerrada: bool,
    ) -> Venta {
        Venta {
            id,
            monto_total,
            productos,
            pagos,
            monto_pagado,
            vendedor,
            paga,
            cliente,
            cerrada,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn empty(&mut self) {
        self.monto_pagado = 0.0;
        self.productos.clear();
        self.monto_total = 0.0;
        self.pagos.clear();
    }
    pub fn monto_total(&self) -> f32 {
        self.monto_total
    }
    pub fn productos(&self) -> Vec<Valuable> {
        self.productos.clone()
    }
    pub fn pagos(&self) -> Vec<Pago> {
        self.pagos.clone()
    }
    pub fn monto_pagado(&self) -> f32 {
        self.monto_pagado
    }
    pub fn set_cantidad_producto(&mut self,index:usize,cantidad: f32)->Res<Self>{
        let producto= self.productos.remove(index);
        let producto=match producto{
            Valuable::Prod((_,prod)) => Valuable::Prod((cantidad as u8,prod)),
            Valuable::Pes((_,pes)) => Valuable::Pes((cantidad,pes)),
            Valuable::Rub((_,rub)) => Valuable::Rub((cantidad as u8,rub)),
        };
        self.productos.insert(index, producto);
        Ok(self.clone())
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f32) -> Res<f32> {
        let mut es_cred: bool = false;
        match medio_pago {
            CUENTA => match &self.cliente {
                Cliente::Final => {
                    return Err(AppError::IncorrectError(String::from(
                        "No esta permitido cuenta corriente en este cliente",
                    )))
                }
                Cliente::Regular(cli) => match cli.credito() {
                    true => {
                        let medio_pago = MedioPago::new(CUENTA, 0);
                        self.pagos.push(Pago::new(medio_pago, monto, Some(0.0)));
                    }
                    false => {
                        return Err(AppError::IncorrectError(String::from(
                            "No esta permitido cuenta corriente en este cliente",
                        )))
                    }
                },
            },
            _ => {
                let model = async_runtime::block_on(medio_from_db(medio_pago));
                let medio_pago = MedioPago::new(&model.medio, model.id);
                self.pagos.push(Pago::new(medio_pago, monto, None));
            }
        }

        self.monto_pagado += monto;
        let res = self.monto_total - self.monto_pagado;

        println!("Venta despues del pago {:#?}", self);
        if res <= 0.0 {
            self.cerrada = true;
        }

        for pago in &self.pagos {
            if pago.medio().eq_ignore_ascii_case(CUENTA) {
                es_cred = true;
                break;
            }
        }
        if self.cerrada && !es_cred {
            self.paga = true;
        }
        Ok(res)
    }
    pub fn agregar_producto(&mut self, producto: Valuable, politica: &f32) {
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    V::Pes(a) => prod = V::Pes((a.0 + 1.0, a.1.clone())),
                    V::Prod(a) => prod = V::Prod((a.0 + 1, a.1.clone())),
                    V::Rub(a) => self.productos.push(V::Rub(a.clone())),
                }
                self.productos.insert(i, prod);
                esta = true;
            }
        }
        if !esta {
            let prod = match producto {
                V::Pes(a) => V::Pes((a.0, a.1.clone())),
                V::Prod(a) => V::Prod((a.0, a.1.clone())),
                V::Rub(a) => V::Rub((a.0, a.1.clone())),
            };
            self.productos.push(prod);
        }
        self.update_monto_total(politica);
    }
    fn update_monto_total(&mut self, politica: &f32) {
        self.monto_total = 0.0;
        for i in &self.productos {
            match &i {
                V::Pes(a) => self.monto_total += redondeo(politica, a.0 * a.1.precio_peso()),
                V::Prod(a) => self.monto_total += a.1.precio_de_venta() * a.0 as f32,
                V::Rub(a) => self.monto_total += a.1.monto().unwrap() * a.0 as f32,
            }
        }
    }
    pub fn eliminar_pago(&mut self, id: u32) -> Res<()> {
        let mut pago = Pago::default();
        let mut esta = false;
        for i in 0..self.pagos.len() {
            if self.pagos[i].id() == id {
                pago = self.pagos.remove(i);
                esta = true;
                break;
            }
        }
        if !esta {
            return Err(AppError::IncorrectError(String::from(
                "Error de id de pago",
            )));
        }
        self.monto_pagado -= pago.monto();
        Ok(())
    }
    pub fn restar_producto(&mut self, index: usize, politica: &f32) -> Result<Venta, AppError> {
        if self.productos().len() > index {
            let mut prod = self.productos.remove(index);
            match &prod {
                V::Pes(a) => {
                    if a.0 > 1.0 {
                        prod = V::Pes((a.0 - 1.0, a.1.clone()))
                    }
                }
                V::Prod(a) => {
                    if a.0 > 1 {
                        prod = V::Prod((a.0 - 1, a.1.clone()))
                    }
                }
                V::Rub(a) => {
                    if a.0 > 1 {
                        prod = V::Rub((a.0 - 1, a.1.clone()))
                    }
                }
            }
            self.productos.insert(index, prod);
            self.update_monto_total(politica);
            Ok(self.clone())
        } else {
            Err(AppError::NotFound {
                objeto: String::from("Indice"),
                instancia: index.to_string(),
            })
        }
    }
    pub fn incrementar_producto(
        &mut self,
        index: usize,
        politica: &f32,
    ) -> Result<Venta, AppError> {
        if self.productos().len() > index {
            let mut prod = self.productos.remove(index);
            match &prod {
                V::Pes(a) => prod = V::Pes((a.0 + 1.0, a.1.clone())),
                V::Prod(a) => prod = V::Prod((a.0 + 1, a.1.clone())),
                V::Rub(a) => prod = V::Rub((a.0 + 1, a.1.clone())),
            }
            self.productos.insert(index, prod);
            self.update_monto_total(politica);
            Ok(self.clone())
        } else {
            Err(AppError::NotFound {
                objeto: String::from("Indice"),
                instancia: index.to_string(),
            })
        }
    }
    pub async fn set_cliente(&mut self, id: i32, db: &DatabaseConnection) -> Res<()> {
        if id == 0 {
            self.cliente = Cliente::Final;
            Ok(())
        } else {
            match CliDB::Entity::find_by_id(id).one(db).await? {
                Some(model) => {
                    self.cliente = Cliente::Regular(Cli::new(
                        model.id,
                        Arc::from(model.nombre),
                        model.dni,
                        model.credito,
                        model.activo,
                        model.created,
                        model.limite,
                    ));
                    Ok(())
                }
                None => Err(AppError::NotFound {
                    objeto: String::from("Cliente"),
                    instancia: id.to_string(),
                }),
            }
        }
    }
    pub fn eliminar_producto(&mut self, index: usize, politica: &f32) -> Result<Venta, AppError> {
        if self.productos().len() > index {
            self.productos.remove(index);
            self.update_monto_total(politica);
            Ok(self.clone())
        } else {
            Err(AppError::NotFound {
                objeto: String::from("Indice"),
                instancia: index.to_string(),
            })
        }
    }
    pub async fn guardar(&self, pos: bool, db: &DatabaseConnection) -> Res<()> {
        match VentaDB::Entity::find_by_id(self.id).one(db).await? {
            Some(model) => {
                let mut model = model.into_active_model();
                model.monto_pagado = Set(self.monto_pagado);
                model.monto_total = Set(self.monto_total);
                model.time = Set(Utc::now().naive_local());
                match &self.cliente {
                    Cliente::Final => {
                        model.paga = Set(true);
                        model.cliente = Set(None)
                    }
                    Cliente::Regular(cli) => {
                        model.paga = Set(self.paga);
                        model.cliente = Set(Some(*cli.id()));
                    }
                }
                model.cerrada = Set(self.cerrada);
                model.pos = Set(pos);
                if let Err(e) = model.update(db).await {
                    println!("Error update venta {:#?}", e);
                    return Err(e.into());
                }
            }
            None => {
                let paga;
                let cliente;
                match &self.cliente {
                    Cliente::Final => {
                        paga = Set(true);
                        cliente = Set(None)
                    }
                    Cliente::Regular(cli) => {
                        paga = Set(self.paga);
                        cliente = Set(Some(*cli.id()));
                    }
                }
                let venta_model = VentaDB::ActiveModel {
                    id: Set(self.id),
                    monto_total: Set(self.monto_total),
                    monto_pagado: Set(self.monto_pagado),
                    time: Set(Utc::now().naive_local()),
                    cliente,
                    cerrada: Set(self.cerrada),
                    paga,
                    pos: Set(pos),
                };
                if let Err(e) = venta_model.insert(db).await {
                    println!("Error de insert venta: {:#?}", e);
                    return Err(e.into());
                }
            }
        };

        let pagos_model = self
            .pagos
            .iter()
            .map(|pago| PagoDB::ActiveModel {
                medio_pago: Set(*pago.medio_pago().id()),
                monto: Set(pago.monto()),
                venta: Set(self.id),
                pagado: Set(*pago.pagado()),
                ..Default::default()
            })
            .collect::<Vec<PagoDB::ActiveModel>>();
        if let Err(e) = PagoDB::Entity::insert_many(pagos_model).exec(db).await {
            println!("Error insert pagos: {:#?}", e);
            return Err(e.into());
        }
        let relaciones_prod_model = self
            .productos
            .iter()
            .filter_map(|prod| match prod {
                Valuable::Prod(p) => Some(VentaProdDB::ActiveModel {
                    producto: Set(*p.1.id()),
                    cantidad: Set(p.0),
                    precio: Set(*p.1.precio_de_venta()),
                    venta: Set(self.id),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect::<Vec<VentaProdDB::ActiveModel>>();
        if let Err(e) = VentaProdDB::Entity::insert_many(relaciones_prod_model)
            .exec(db)
            .await
        {
            println!("Error insert relacionVentaProd {:#?}", e);
        }
        let relaciones_rub_model = self
            .productos
            .iter()
            .filter_map(|prod| match prod {
                Valuable::Rub(rub) => {
                    let precio = match rub.1.monto() {
                        Some(a) => Set(*a),
                        None => NotSet,
                    };
                    Some(VentaRubDB::ActiveModel {
                        cantidad: Set(rub.0),
                        precio: precio,
                        rubro: Set(*rub.1.id()),
                        venta: Set(self.id),
                        ..Default::default()
                    })
                }
                _ => None,
            })
            .collect::<Vec<VentaRubDB::ActiveModel>>();
        if let Err(e) = VentaRubDB::Entity::insert_many(relaciones_rub_model)
            .exec(db)
            .await
        {
            println!("Error insert relacionVentaRub {:#?}", e);
        }
        let relaciones_pes_model = self
            .productos
            .iter()
            .filter_map(|prod| match prod {
                Valuable::Pes(pes) => Some(VentaPesDB::ActiveModel {
                    cantidad: Set(pes.0),
                    precio: Set(*pes.1.precio_peso()),
                    pesable: Set(*pes.1.id()),
                    venta: Set(self.id),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect::<Vec<VentaPesDB::ActiveModel>>();
        if let Err(e) = VentaPesDB::Entity::insert_many(relaciones_pes_model)
            .exec(db)
            .await
        {
            println!("Error insert relacionVentaPes {:#?}", e);
        }
        Ok(())
    }
}
impl Save for Venta {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        let mut venta = VentaDB::Entity::find_by_id(self.id)
            .one(&db)
            .await?
            .unwrap()
            .into_active_model();
        venta.monto_total = Set(self.monto_total);
        venta.monto_pagado = Set(self.monto_pagado);
        venta.cerrada = Set(self.cerrada);
        venta.paga = Set(self.paga);
        venta.time = Set(Utc::now().naive_local());
        let mut pay_models = vec![];
        for pago in &self.pagos {
            if pago.medio().as_ref().eq(CUENTA) {
                pay_models.push(PagoDB::ActiveModel {
                    medio_pago: Set(0),
                    monto: Set(pago.monto()),
                    venta: Set(self.id),
                    ..Default::default()
                })
            } else {
                let model = medio_from_db(&pago.medio().to_string().as_str()).await;
                pay_models.push(PagoDB::ActiveModel {
                    medio_pago: Set(model.id),
                    monto: Set(pago.monto()),
                    venta: Set(self.id),
                    ..Default::default()
                });
            }
        }
        match &self.cliente {
            Cliente::Final => (),
            Cliente::Regular(a) => {
                let deudas = pay_models
                    .iter()
                    .filter_map(|p| match p.medio_pago {
                        NotSet => None,
                        _ => Some(DeudaDB::ActiveModel {
                            cliente: Set(*a.id()),
                            monto: p.monto.clone(),
                            pago: p.id.clone(),
                            ..Default::default()
                        }),
                    })
                    .collect::<Vec<DeudaDB::ActiveModel>>();
                DeudaDB::Entity::insert_many(deudas).exec(&db).await?;
                venta.cliente = Set(Some(*a.id()))
            }
        }
        venta.update(&db).await?;

        if pay_models.len() > 1 {
            PagoDB::Entity::insert_many(pay_models).exec(&db).await?;
        } else {
            PagoDB::Entity::insert(pay_models[0].clone())
                .exec(&db)
                .await?;
        }

        let prod_models: Vec<VentaProdDB::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                V::Prod(a) => Some(VentaProdDB::ActiveModel {
                    producto: Set(*a.1.id()),
                    venta: Set(self.id),
                    cantidad: Set(a.0),
                    precio: Set(*a.1.precio_de_venta()),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        VentaProdDB::Entity::insert_many(prod_models)
            .exec(&db)
            .await?;

        let rub_models: Vec<VentaRubDB::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                V::Rub(a) => Some(VentaRubDB::ActiveModel {
                    cantidad: Set(a.0),
                    rubro: Set(*a.1.id()),
                    venta: Set(self.id),
                    precio: Set(*a.1.monto().unwrap()),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        if rub_models.len() > 1 {
            VentaRubDB::Entity::insert_many(rub_models)
                .exec(&db)
                .await?;
        } else if rub_models.len() == 1 {
            VentaRubDB::Entity::insert(rub_models[0].clone())
                .exec(&db)
                .await?;
        }
        let pes_models: Vec<VentaPesDB::ActiveModel> = self
            .productos
            .iter()
            .filter_map(|x| match x {
                V::Pes(a) => Some(VentaPesDB::ActiveModel {
                    cantidad: Set(a.0),
                    pesable: Set(*a.1.id()),
                    venta: Set(self.id),
                    precio: Set(*a.1.precio_peso()),
                    ..Default::default()
                }),
                _ => None,
            })
            .collect();
        if pes_models.len() > 1 {
            VentaPesDB::Entity::insert_many(pes_models)
                .exec(&db)
                .await?;
        } else if pes_models.len() == 1 {
            VentaPesDB::Entity::insert(pes_models[0].clone())
                .exec(&db)
                .await?;
        }

        Ok(())
    }
}
