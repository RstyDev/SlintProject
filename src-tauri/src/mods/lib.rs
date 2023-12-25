use super::Producto;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::{Read,Write};


pub fn crear_file<'a>(path: &String, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", format!("{}", buf))?;
    Ok(())
}

// pub fn push(pr: Producto, path: &String) {
//     let mut prods = Vec::new();
//     if let Err(e) = leer_file(&mut prods, path) {
//         panic!("{}", e);
//     }
//     prods.push(pr);
//     match crear_file(&path, &prods) {
//         Ok(_) => (),
//         Err(e) => panic!("No se pudo pushear porque {}", e),
//     };
// }

pub fn leer_file<T: DeserializeOwned + Clone + Serialize>(
    buf: &mut T,
    path: &String,
) -> std::io::Result<()> {
    let file2 = File::open(path.clone());
    let mut file2 = match file2 {
        Ok(file) => file,
        Err(_) => {
            let esc: Vec<String> = Vec::new();
            crear_file(path, &esc)?;
            File::open(path.clone())?
        }
    };

    let mut buf2 = String::new();
    file2.read_to_string(&mut buf2)?;
    match serde_json::from_str::<T>(&buf2.clone()) {
        Ok(a) => *buf = a.clone(),
        Err(e) => {
            panic!("No se pudo porque {}", e)
        }
    }
    Ok(())
}


