use sea_orm_migration::prelude::*;

use crate::{pesable::Pesable, venta::Venta};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RelacionVentaPes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionVentaPes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaPes::Cantidad)
                            .float()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RelacionVentaPes::Precio).double().not_null())
                    .col(
                        ColumnDef::new(RelacionVentaPes::Pesable)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("pesable_fk")
                            .from(RelacionVentaPes::Table, RelacionVentaPes::Venta)
                            .to(Pesable::Table, Pesable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaPes::Venta)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta_fk")
                            .from(RelacionVentaPes::Table, RelacionVentaPes::Venta)
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
            .drop_table(Table::drop().table(RelacionVentaPes::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum RelacionVentaPes {
    Table,
    Id,
    Cantidad,
    Precio,
    Pesable,
    Venta,
}
