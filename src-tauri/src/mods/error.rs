use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No se pudo formatear la fecha")]
    DateFormat,
    #[error("Solo existen dos posiciones para fecha")]
    SaleSelecion,
    #[error("No encontrado el producto de id {0}")]
    ProductNotFound(String)
}





#[derive(Debug, Error)]
pub struct ProductNotFoundError;
impl fmt::Display for ProductNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error, producto no encontrado")
    }
}
#[derive(Debug, Error)]
pub struct ExistingProviderError;
impl fmt::Display for ExistingProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error, proveedor existente")
    }
}
#[derive(Debug, Error)]
pub struct AmountError;
impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "El monto no puede ser superior al resto con el medio de pago actual"
        )
    }
}
#[derive(Debug, Error)]
pub struct SizeSelecionError;
impl fmt::Display for SizeSelecionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error, las presentaciones habilitadas son: Gr Un Lt Ml CC Kg"
        )
    }
}
