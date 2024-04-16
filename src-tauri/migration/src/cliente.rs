use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cliente::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cliente::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cliente::Nombre).string().not_null())
                    .col(ColumnDef::new(Cliente::Dni).big_integer().not_null())
                    .col(ColumnDef::new(Cliente::Credito).boolean().not_null())
                    .col(ColumnDef::new(Cliente::Limite).float())
                    .col(ColumnDef::new(Cliente::Activo).boolean().not_null())
                    .col(ColumnDef::new(Cliente::Created).string().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cliente::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum Cliente {
    Table,
    Id,
    Nombre,
    Dni,
    Credito,
    Limite,
    Activo,
    Created,
}
