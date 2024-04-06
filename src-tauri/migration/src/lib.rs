pub use sea_orm_migration::prelude::*;
mod caja;
mod cliente;
mod codigo_barras;
mod config;
mod deuda;
mod medio_pago;
mod movimiento;
mod pago;
mod pesable;
mod producto;
mod proveedor;
mod relacion_prod_prov;
mod relacion_venta_pes;
mod relacion_venta_prod;
mod relacion_venta_rub;
mod rubro;
mod user;
mod venta;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(caja::Migration),
            Box::new(cliente::Migration),
            Box::new(codigo_barras::Migration),
            Box::new(config::Migration),
            Box::new(deuda::Migration),
            Box::new(medio_pago::Migration),
            Box::new(movimiento::Migration),
            Box::new(pago::Migration),
            Box::new(pesable::Migration),
            Box::new(producto::Migration),
            Box::new(proveedor::Migration),
            Box::new(relacion_prod_prov::Migration),
            Box::new(relacion_venta_pes::Migration),
            Box::new(relacion_venta_prod::Migration),
            Box::new(relacion_venta_rub::Migration),
            Box::new(rubro::Migration),
            Box::new(user::Migration),
            Box::new(venta::Migration),
            ]
    }
}
