use chrono::NaiveDateTime;
use serde::Serialize;
use std::sync::Arc;
#[derive(Serialize, Clone, Debug)]
pub enum Cliente {
    Final(Arc<str>),
    Regular(Cli),
}
#[derive(Serialize, Clone, Debug)]
pub struct Cli {
    id: i64,
    nombre: Arc<str>,
    dni: i64,
    credito: bool,
    activo: bool,
    created: NaiveDateTime,
}
impl Cli {
    pub async fn new_to_db(
        id: i64,
        nombre: Arc<str>,
        dni: i64,
        credito: bool,
        activo: bool,
        created: NaiveDateTime,
    ) -> Cli {
        Cli {
            id,
            nombre,
            dni,
            credito,
            activo,
            created,
        }
    }
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
impl Default for Cliente {
    fn default() -> Self {
        Cliente::Final(Arc::from("Consumidor final"))
    }
}
