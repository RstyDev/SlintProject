//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "relacion_venta_prod")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub producto: i64,
    pub cantidad: i16,
    pub venta: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::producto::Entity",
        from = "Column::Producto",
        to = "super::producto::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Producto,
    #[sea_orm(
        belongs_to = "super::venta::Entity",
        from = "Column::Venta",
        to = "super::venta::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Venta,
}

impl Related<super::producto::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Producto.def()
    }
}

impl Related<super::venta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Venta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}