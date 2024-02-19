use chrono::NaiveDateTime;
use std::sync::Arc;
pub enum Cliente {
    Final(Arc<str>),
    Regular(Cli),
}
pub struct Cli {
    id: i64,
    nombre: Arc<str>,
    dni: i64,
    credito: bool,
    activo: bool,
    created: NaiveDateTime,
}
impl Cli {
    pub fn id(&self) -> &i64 {
        &self.id
    }
}

impl Cliente {
    pub fn new(cli: Option<Cli>) -> Cliente {
        match cli {
            Some(a) => Cliente::Regular(a),
            None => Cliente::Final(Arc::from("Consumidor final")),
        }
    }
}
