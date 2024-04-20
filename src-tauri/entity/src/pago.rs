//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pago")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub medio_pago: i32,
    #[sea_orm(column_type = "Double")]
    pub monto: f32,
    pub venta: i32,
    #[sea_orm(column_type = "Double")]
    pub pagado: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::deuda::Entity")]
    Deuda,
    #[sea_orm(
        belongs_to = "super::medio_pago::Entity",
        from = "Column::MedioPago",
        to = "super::medio_pago::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MedioPago,
    #[sea_orm(
        belongs_to = "super::venta::Entity",
        from = "Column::Venta",
        to = "super::venta::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Venta,
}

impl Related<super::deuda::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deuda.def()
    }
}

impl Related<super::medio_pago::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MedioPago.def()
    }
}

impl Related<super::venta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Venta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
