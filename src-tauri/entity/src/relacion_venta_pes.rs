//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "relacion_venta_pes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Double")]
    pub cantidad: f32,
    #[sea_orm(column_type = "Double")]
    pub precio: f32,
    pub pesable: i32,
    pub venta: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pesable::Entity",
        from = "Column::Venta",
        to = "super::pesable::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Pesable,
    #[sea_orm(
        belongs_to = "super::venta::Entity",
        from = "Column::Venta",
        to = "super::venta::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Venta,
}

impl Related<super::pesable::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pesable.def()
    }
}

impl Related<super::venta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Venta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
