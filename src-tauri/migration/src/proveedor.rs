use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Proveedor::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Proveedor::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Proveedor::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Proveedor::Nombre).string().not_null())
                    .col(ColumnDef::new(Proveedor::Contacto).big_integer())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Proveedor::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Proveedor {
    Table,
    Id,
    Nombre,
    Contacto,
    UpdatedAt,
}
