use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rubro::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rubro::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rubro::Codigo).big_integer().not_null())
                    .col(ColumnDef::new(Rubro::Monto).float())
                    .col(ColumnDef::new(Rubro::Descripcion).string().not_null())
                    .col(ColumnDef::new(Rubro::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rubro::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum Rubro {
    Table,
    Id,
    Codigo,
    Monto,
    Descripcion,
    UpdatedAt,
}
