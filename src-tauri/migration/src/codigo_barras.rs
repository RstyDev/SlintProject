use sea_orm_migration::prelude::*;

use crate::producto::Producto;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CodigoBarras::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CodigoBarras::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CodigoBarras::Codigo)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(CodigoBarras::Producto)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("producto_fk")
                            .from(CodigoBarras::Table, CodigoBarras::Producto)
                            .to(Producto::Table, Producto::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CodigoBarras::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum CodigoBarras {
    Table,
    Id,
    Codigo,
    Producto,
}
