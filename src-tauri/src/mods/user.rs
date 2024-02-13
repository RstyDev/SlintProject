use std::sync::Arc;
#[derive(Debug)]
pub struct User {
    id: Arc<str>,
    pass: i64,
    rango: Rango,
}
#[derive(Debug)]
pub enum Rango {
    Admin,
    Cajero,
}
impl User {
    pub fn new(id: Arc<str>, pass: i64, rango: &str) -> User {
        let rango = match rango {
            "Admin" => Rango::Admin,
            "Cajero" => Rango::Cajero,
            _ => panic!("No existe"),
        };
        User { id, pass, rango }
    }
    pub fn rango(&self) -> &Rango {
        &self.rango
    }
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    pub fn pass(&self) -> &i64 {
        &self.pass
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
