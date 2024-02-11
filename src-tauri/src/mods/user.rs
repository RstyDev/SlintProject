use std::sync::Arc;
#[derive(Debug)]
pub struct User {
    id: Arc<str>,
    pass: i64,
    permiso: Rango,
}
#[derive(Debug)]
pub enum Rango {
    Admin,
    Cajero,
}
impl User {
    pub fn new(id: Arc<str>, pass: i64, permiso: &str) -> User {
        let permiso = match permiso {
            "Admin" => Rango::Admin,
            "Cajero" => Rango::Cajero,
            _ => panic!("No existe"),
        };
        User { id, pass, permiso }
    }
}
impl ToString for Rango {
    fn to_string(&self) -> String {
        match self {
            Rango::Admin => String::from("Admin"),
            Rango::Cajero => String::from("Cajero"),
        }
    }
}
