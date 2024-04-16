use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Config::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Config::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Config::CantidadProductos)
                            .small_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Config::FormatoProducto).string().not_null())
                    .col(ColumnDef::new(Config::ModoMayus).string().not_null())
                    .col(ColumnDef::new(Config::PoliticaRedondeo).double().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Config::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Config {
    Table,
    Id,
    PoliticaRedondeo,
    FormatoProducto,
    ModoMayus,
    CantidadProductos,
}