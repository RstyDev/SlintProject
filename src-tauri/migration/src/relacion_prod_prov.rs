use sea_orm_migration::prelude::*;

use crate::{producto::Producto, proveedor::Proveedor};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RelacionProdProv::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionProdProv::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionProdProv::Producto)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("producto_fk")
                            .from(RelacionProdProv::Table, RelacionProdProv::Producto)
                            .to(Producto::Table, Producto::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionProdProv::Proveedor)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("proveedor_fk")
                            .from(RelacionProdProv::Table, RelacionProdProv::Proveedor)
                            .to(Proveedor::Table, Proveedor::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(RelacionProdProv::Codigo).big_integer())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RelacionProdProv::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum RelacionProdProv {
    Table,
    Id,
    Producto,
    Proveedor,
    Codigo,
}
