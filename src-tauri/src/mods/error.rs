use sea_orm::DbErr;
use std::{
    io,
    num::{ParseFloatError, ParseIntError},
    time::SystemTimeError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de monto, el monto a pagar es: {a_pagar:?},el monto pagado es: {pagado:?}")]
    AmountError { a_pagar: f64, pagado: f64 },
    #[error("Solo existen dos posiciones para venta")]
    SaleSelection,
    #[error("Presentacion seleccionada incorrecta, no existe {0}")]
    SizeSelection(String),
    #[error("Proveedor {0} existente")]
    ExistingProviderError(String),
    #[error("No encontrado el producto de id {0}")]
    ProductNotFound(String),
    #[error("Error de archivo")]
    FileSystemError(#[from] io::Error),
    #[error("Error de hora del sistema")]
    SystemTimeError(#[from] SystemTimeError),
    #[error("Error de bases de datos")]
    DbError(#[from] DbErr),
    #[error("Error de conversion de flotante")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Error de conversion de enteros")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error de tauri")]
    TauriError(#[from] tauri::Error),
}
