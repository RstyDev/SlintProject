use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Caja::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Caja::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Caja::Inicio).string().not_null())
                    .col(ColumnDef::new(Caja::Cierre).string())
                    .col(ColumnDef::new(Caja::MontoInicio).double().not_null())
                    .col(ColumnDef::new(Caja::MontoCierre).double())
                    .col(ColumnDef::new(Caja::VentasTotales).double().not_null())
                    .col(ColumnDef::new(Caja::Cajero).string())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Caja::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum Caja {
    Table,
    Id,
    Inicio,
    Cierre,
    MontoInicio,
    MontoCierre,
    VentasTotales,
    Cajero,
}
