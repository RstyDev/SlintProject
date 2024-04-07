use sea_orm_migration::prelude::*;

use crate::caja::Caja;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Movimiento::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Movimiento::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Movimiento::Caja).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("caja_fk")
                            .from(Movimiento::Table, Movimiento::Caja)
                            .to(Caja::Table, Caja::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Movimiento::Tipo).boolean().not_null())
                    .col(ColumnDef::new(Movimiento::Monto).double().not_null())
                    .col(ColumnDef::new(Movimiento::Descripcion).string())
                    .col(ColumnDef::new(Movimiento::Time).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Movimiento::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Movimiento {
    Table,
    Id,
    Caja,
    Tipo,
    Monto,
    Descripcion,
    Time,
}
