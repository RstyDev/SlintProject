use crate::db::map::{BigIntDB, ClienteDB, VentaDB};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, Pool, Sqlite};
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use tokio::{spawn, task::spawn_blocking};

use Valuable as V;
const CUENTA: &str = "Cuenta Corriente";

use crate::{db::Mapper, mods::pago::medio_from_db};

use super::{
    redondeo, AppError, Cli, Cliente, Cuenta::Auth, Cuenta::Unauth, MedioPago, Pago, Res, User,
    Valuable,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Venta {
    id: i64,
    monto_total: f32,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f32,
    vendedor: Option<Arc<User>>,
    cliente: Cliente,
    paga: bool,
    cerrada: bool,
    time: NaiveDateTime,
}

impl<'a> Venta {
    pub async fn new(vendedor: Option<Arc<User>>, db: &Pool<Sqlite>, pos: bool) -> Res<Venta> {
        let time = Utc::now().naive_local();
        let res= query(
            "insert into ventas (time, monto_total, monto_pagado, cliente, cerrada, paga, pos ) values (?, ?, ?, ?, ?, ?, ?)").bind(time).bind(0.0).bind(0.0).bind(None::<i64>).bind(false).bind(false).bind(pos).execute(db).await?;
        let id = res.last_insert_rowid();
        let cliente = Cliente::new(None);
        Ok(Venta {
            monto_total: 0.0,
            productos: Vec::new(),
            pagos: Vec::new(),
            monto_pagado: 0.0,
            vendedor,
            id,
            paga: false,
            cliente,
            cerrada: false,
            time,
        })
    }
    pub async fn get_or_new(
        vendedor: Option<Arc<User>>,
        db: &Pool<Sqlite>,
        pos: bool,
    ) -> Res<Venta> {
        let qres: Option<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id, time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente, cerrada, paga, pos from ventas where pos = ? and cerrada = ?"#,
            pos,
            false
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(model) => match model.cerrada {
                true => Venta::new(vendedor, db, pos).await,
                false => Mapper::venta(db, model, &vendedor).await,
            },
            None => Venta::new(vendedor, db, pos).await,
        }
    }
    pub fn build(
        id: i64,
        monto_total: f32,
        productos: Vec<Valuable>,
        pagos: Vec<Pago>,
        monto_pagado: f32,
        vendedor: Option<Arc<User>>,
        cliente: Cliente,
        paga: bool,
        cerrada: bool,
        time: NaiveDateTime,
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
            time,
        }
    }
    pub fn id(&self) -> &i64 {
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
    pub fn set_cantidad_producto(
        &mut self,
        index: usize,
        cantidad: f32,
        politica: &f32,
    ) -> Res<Self> {
        let producto = self.productos.remove(index);
        let producto = match producto {
            Valuable::Prod((_, prod)) => Valuable::Prod((cantidad as u8, prod)),
            Valuable::Pes((_, pes)) => Valuable::Pes((cantidad, pes)),
            Valuable::Rub((_, rub)) => Valuable::Rub((cantidad as u8, rub)),
        };
        self.productos.insert(index, producto);
        self.update_monto_total(politica);
        Ok(self.clone())
    }
    pub fn agregar_pago(&mut self, medio_pago: &str, monto: f32, db: &Pool<Sqlite>) -> Res<f32> {
        let mut es_cred: bool = false;
        match medio_pago {
            CUENTA => match &self.cliente {
                Cliente::Final => {
                    return Err(AppError::IncorrectError(String::from(
                        "No esta permitido cuenta corriente en este cliente",
                    )))
                }
                Cliente::Regular(cli) => match cli.limite() {
                    Auth(_) => {
                        let medio_pago = MedioPago::build(CUENTA, 0);
                        self.pagos.push(Pago::new(medio_pago, monto, Some(0.0)));
                    }
                    Unauth => {
                        return Err(AppError::IncorrectError(String::from(
                            "No esta permitido cuenta corriente en este cliente",
                        )))
                    }
                },
            },
            _ => {
                let rt=Runtime::new().unwrap();
                let spawn = spawn_blocking(move || medio_from_db(medio_pago,db));
                let res=rt.block_on(async {medio_from_db(medio_pago,db).await});
                //let medio_db = async_runtime::block_on(medio_from_db(medio_pago, db));
                let medio_pago = MedioPago::build(&medio_db.medio, medio_db.id);
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
    pub fn eliminar_pago(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        let mut pago = Pago::def(db);
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
    pub async fn set_cliente(&mut self, id: i64, db: &Pool<Sqlite>) -> Res<()> {
        if id == 0 {
            self.cliente = Cliente::Final;
            Ok(())
        } else {
            let qres: Option<ClienteDB> =
                sqlx::query_as!(ClienteDB, r#"select id, nombre, dni as "dni:_", limite as "limite:_", activo, time from clientes where id = ? limit 1"#, id)
                    .fetch_optional(db)
                    .await?;
            match qres {
                Some(model) => {
                    self.cliente = Cliente::Regular(Cli::build(
                        model.id,
                        Arc::from(model.nombre),
                        model.dni,
                        model.activo,
                        model.time,
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
    pub async fn guardar(&self, pos: bool, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from ventas where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                let paga;
                let cliente;
                match &self.cliente {
                    Cliente::Final => {
                        paga = true;
                        cliente = None;
                    }
                    Cliente::Regular(cli) => {
                        paga = self.paga;
                        cliente = Some(*cli.id());
                    }
                }
                sqlx::query("update ventas set time = ?, monto_total = ?, monto_pagado = ?, cliente = ?, cerrada = ?, paga = ?, pos = ? where id = ?")
                .bind(Utc::now().naive_local()).bind(self.monto_total)
                .bind(self.monto_pagado).bind(cliente).bind(self.cerrada).bind(paga).bind(pos).bind(self.id).execute(db).await?;
            }
            None => {
                let paga;
                let cliente;
                match &self.cliente {
                    Cliente::Final => {
                        paga = true;
                        cliente = None;
                    }
                    Cliente::Regular(cli) => {
                        paga = self.paga;
                        cliente = Some(*cli.id());
                    }
                }
                sqlx::query("insert into ventas (time, monto_total, monto_pagado, cliente, cerrada, paga, pos)
                values (?, ?, ?, ?, ?, ?, ?)").bind(Utc::now().naive_local()).bind(self.monto_total).bind(self.monto_pagado).bind(cliente).bind(self.cerrada)
                .bind(paga).bind(pos).execute(db).await?;
            }
        }
        let mut pagos_sql = String::from(
            "INSERT INTO pagos (medio_pago, monto, pagado, venta) VALUES (?, ?, ?, ?)",
        );
        let mut venta_prod_sql = String::from("INSERT INTO relacion_venta_prod (venta, producto, cantidad, precio) VALUES (?, ?, ?, ?)");
        let mut venta_pes_sql = String::from("INSERT INTO relacion_venta_pes (venta, pesable, cantidad, precio_kilo) VALUES (?, ?, ?, ?)");
        let mut venta_rub_sql = String::from(
            "INSERT INTO relacion_venta_rub (venta, rubro, cantidad, precio) VALUES (?, ?, ?, ?)",
        );
        let row = ", (?, ?, ?, ?)";
        for _ in 1..self.pagos.len() {
            pagos_sql.push_str(row);
        }
        for prod in &self.productos {
            match prod {
                Valuable::Prod(_) => venta_prod_sql.push_str(row),
                Valuable::Pes(_) => venta_pes_sql.push_str(row),
                Valuable::Rub(_) => venta_rub_sql.push_str(row),
            }
        }
        let mut pagos_query = sqlx::query(pagos_sql.as_str());
        let mut prod_query = sqlx::query(venta_prod_sql.as_str());
        let mut pes_query = sqlx::query(venta_pes_sql.as_str());
        let mut rub_query = sqlx::query(venta_rub_sql.as_str());
        for pago in &self.pagos {
            let aux = pagos_query;
            pagos_query = aux
                .bind(*pago.medio_pago().id())
                .bind(pago.monto())
                .bind(*pago.pagado())
                .bind(self.id);
        }
        for prod in &self.productos {
            match prod {
                Valuable::Prod((c, p)) => {
                    let aux = prod_query;
                    prod_query = aux
                        .bind(self.id)
                        .bind(*p.id())
                        .bind(*c)
                        .bind(*p.precio_de_venta());
                }
                Valuable::Pes((c, p)) => {
                    let aux = pes_query;
                    pes_query = aux
                        .bind(self.id)
                        .bind(*p.id())
                        .bind(*c)
                        .bind(*p.precio_peso());
                }
                Valuable::Rub((c, r)) => {
                    let aux = rub_query;
                    rub_query = aux.bind(self.id).bind(*r.id()).bind(*c).bind(r.monto());
                }
            }
        }
        pagos_query.execute(db).await?;
        prod_query.execute(db).await?;
        pes_query.execute(db).await?;
        rub_query.execute(db).await?;
        Ok(())
    }
}
