use sea_orm_migration::prelude::*;

use crate::{rubro::Rubro, venta::Venta};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RelacionVentaRub::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionVentaRub::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaRub::Cantidad)
                            .small_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RelacionVentaRub::Precio).float().not_null())
                    .col(
                        ColumnDef::new(RelacionVentaRub::Rubro)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("rubro_fk")
                            .from(RelacionVentaRub::Table, RelacionVentaRub::Rubro)
                            .to(Rubro::Table, Rubro::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaRub::Venta)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta_fk")
                            .from(RelacionVentaRub::Table, RelacionVentaRub::Venta)
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
            .drop_table(Table::drop().table(RelacionVentaRub::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum RelacionVentaRub {
    Table,
    Id,
    Cantidad,
    Rubro,
    Precio,
    Venta,
}
