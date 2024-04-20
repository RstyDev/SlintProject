use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pesable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pesable::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pesable::Codigo).big_integer().not_null())
                    .col(ColumnDef::new(Pesable::PrecioPeso).float().not_null())
                    .col(ColumnDef::new(Pesable::Porcentaje).float().not_null())
                    .col(ColumnDef::new(Pesable::CostoKilo).float().not_null())
                    .col(ColumnDef::new(Pesable::Descripcion).string().not_null())
                    .col(ColumnDef::new(Pesable::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pesable::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum Pesable {
    Table,
    Id,
    Codigo,
    PrecioPeso,
    Porcentaje,
    CostoKilo,
    Descripcion,
    UpdatedAt,
}
