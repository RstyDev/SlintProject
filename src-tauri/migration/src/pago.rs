use sea_orm_migration::prelude::*;

use crate::{medio_pago::MedioPago, venta::Venta};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pago::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pago::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pago::MedioPago).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("medio_pago_fk")
                            .from(Pago::Table, Pago::MedioPago)
                            .to(MedioPago::Table, MedioPago::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Pago::Monto).double().not_null())
                    .col(ColumnDef::new(Pago::Venta).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta-fk")
                            .from(Pago::Table, Pago::Venta)
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
            .drop_table(Table::drop().table(Pago::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum Pago {
    Table,
    Id,
    MedioPago,
    Monto,
    Venta,
}
