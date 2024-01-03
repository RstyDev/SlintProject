use entity::producto::Model;

use sea_orm::prelude::DateTimeUtc;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};

pub fn crear_file<'a>(path: &String, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", format!("{}", buf))?;
    Ok(())
}

pub fn camalize(data: &mut String) {
    let mut es = true;
    let iter = data.clone();

    for (i, a) in iter.char_indices() {
        if es {
            data.replace_range(i..i + 1, a.to_ascii_uppercase().to_string().as_str())
        }
        if a == ' ' {
            es = true;
        } else {
            es = false
        }
    }
}

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
        Err(e) => panic!("No se pudo porque {}", e),
    }
    Ok(())
}
pub fn get_updated_time_file(path: &String) -> Result<DateTimeUtc, String> {
    let res = match fs::metadata(path) {
        Ok(a) => match a.modified() {
            Ok(a) => match a.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                Ok(a) => Ok(a),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    };
    let date=match res{
        Ok(a)=>a,
        Err(e)=>return Err(e.to_string()),
    };
    match make_elapsed_to_date(date){
        Some(a)=>Ok(a),
        None=>Err("No se pudo convertir".to_string())
    }
}
pub async fn get_updated_time_db(vec:Vec<Model>)->DateTimeUtc{
    vec.iter().max_by_key(|x|{x.updated_at}).unwrap().updated_at.and_utc()
}

fn make_elapsed_to_date(date: std::time::Duration) -> Option<DateTimeUtc> {
    let (sec, nsec) = (date.as_secs() as i64, date.subsec_nanos());
    DateTimeUtc::from_timestamp(sec, nsec)
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
