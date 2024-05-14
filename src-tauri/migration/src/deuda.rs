use sea_orm_migration::prelude::*;

use crate::{cliente::Cliente, pago::Pago};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Deuda::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Deuda::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Deuda::Cliente).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("cliente_fk")
                            .from(Deuda::Table, Deuda::Cliente)
                            .to(Cliente::Table, Cliente::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Deuda::Monto).float().not_null())
                    .col(ColumnDef::new(Deuda::Pago).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("pago_fk")
                            .from(Deuda::Table, Deuda::Pago)
                            .to(Pago::Table, Pago::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Deuda::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Deuda {
    Table,
    Id,
    Cliente,
    Pago,
    Monto,
}
