use sea_orm_migration::prelude::*;

use crate::{producto::Producto, venta::Venta};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RelacionVentaProd::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionVentaProd::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Producto)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("producto_fk")
                            .from(RelacionVentaProd::Table, RelacionVentaProd::Producto)
                            .to(Producto::Table, Producto::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Cantidad)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Precio)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Venta)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta_fk")
                            .from(RelacionVentaProd::Table, RelacionVentaProd::Venta)
                            .to(Venta::Table, Venta::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RelacionVentaProd::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RelacionVentaProd {
    Table,
    Id,
    Cantidad,
    Precio,
    Producto,
    Venta,
}