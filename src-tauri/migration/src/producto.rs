use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Producto::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Producto::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Producto::PrecioDeVenta).double().not_null())
                    .col(ColumnDef::new(Producto::Porcentaje).double().not_null())
                    .col(ColumnDef::new(Producto::PrecioDeCosto).double().not_null())
                    .col(ColumnDef::new(Producto::TipoProducto).string().not_null())
                    .col(ColumnDef::new(Producto::Marca).string().not_null())
                    .col(ColumnDef::new(Producto::Variedad).string().not_null())
                    .col(ColumnDef::new(Producto::Presentacion).string().not_null())
                    .col(ColumnDef::new(Producto::Cantidad).float().not_null())
                    .col(ColumnDef::new(Producto::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Producto::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum Producto {
    Table,
    Id,
    PrecioDeVenta,
    Porcentaje,
    PrecioDeCosto,
    TipoProducto,
    Marca,
    Variedad,
    Presentacion,
    Cantidad,
    UpdatedAt,
}
