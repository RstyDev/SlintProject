use sea_orm_migration::prelude::*;

use crate::cliente::Cliente;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Venta::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Venta::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Venta::MontoTotal).double().not_null())
                    .col(ColumnDef::new(Venta::MontoPagado).double().not_null())
                    .col(ColumnDef::new(Venta::Time).date_time().not_null())
                    .col(ColumnDef::new(Venta::Cliente).big_integer())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("cliente_fk")
                            .from(Venta::Table, Venta::Cliente)
                            .to(Cliente::Table, Cliente::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Venta::Cerrada).boolean().not_null())
                    .col(ColumnDef::new(Venta::Paga).boolean().not_null())
                    .col(ColumnDef::new(Venta::Pos).boolean().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Venta::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Venta {
    Table,
    Id,
    Time,
    MontoTotal,
    MontoPagado,
    Cliente,
    Cerrada,
    Paga,
    Pos,
}
