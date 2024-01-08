use entity::producto::Model;
use core::fmt;
use sea_orm::prelude::DateTimeUtc;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};

type Result<T>=std::result::Result<T,Box<dyn Error>>;
use std::error::Error;
#[derive(Debug)]
pub struct DateFormatError;
impl fmt::Display for DateFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No se pudo formatear de Systemtime a DatetimeUTC")
    }
}
impl std::error::Error for DateFormatError {}


pub fn crear_file<'a>(path: &str, escritura: &impl Serialize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string_pretty(escritura)?;
    write!(f, "{}", format!("{}", buf))?;
    Ok(())
}

pub fn camalize(data: &str) -> String {
    let mut es = true;
    let mut datos = String::new();
    for i in 0..data.len() {
        if es {
            if data.chars().nth(i) == None {
                datos.push('Ñ');
            } else {
                datos.push(
                    data.chars()
                        .nth(i)
                        .unwrap()
                        .to_uppercase()
                        .to_string()
                        .chars()
                        .last()
                        .unwrap(),
                )
            }
        } else {
            if data.chars().nth(i) == None {
                datos.push('ñ');
            } else {
                datos.push(
                    data.chars()
                        .nth(i)
                        .unwrap()
                        .to_lowercase()
                        .to_string()
                        .chars()
                        .last()
                        .unwrap(),
                )
            }
        }

        if data.chars().nth(i).is_some() && data.chars().nth(i).unwrap() == ' ' {
            es = true;
        } else {
            es = false;
        }
    }

    // for (i, mut a) in iter.char_indices() {
    //     println!("llego");
    //     if es {
    //         if a == 'ñ' || a == 'Ñ' {
    //             println!("es {}", a);
    //             data.replace_range(i..i + 1, 'Ñ'.to_string().as_str());
    //             println!("reemplazado");
    //         } else {
    //             a.make_ascii_uppercase();
    //             data.replace_range(i..i + 1, a.to_string().as_str());
    //         }
    //     } else {
    //         if a == 'ñ' || a == 'Ñ' {
    //             println!("es {}", a);
    //             data.replace_range(i..i + 1, 'ñ'.to_string().as_str());
    //             println!("reemplazado");
    //         } else {
    //             a.make_ascii_lowercase();
    //             data.replace_range(i..i + 1, a.to_string().as_str());
    //         }
    //     }
    //     if a == ' ' {
    //         es = true;
    //     } else {
    //         es = false
    //     }
    // }
    datos
}

pub fn leer_file<T: DeserializeOwned + Clone + Serialize>(
    buf: &mut T,
    path: &str,
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
pub fn get_updated_time_file(path: &str) -> Result<DateTimeUtc> {
    let a = fs::metadata(path)?.modified()?;
    let res = a.duration_since(std::time::SystemTime::UNIX_EPOCH)?;

    make_elapsed_to_date(res).ok_or_else(|| DateFormatError.into())
}
pub async fn get_updated_time_db_producto(vec: Vec<Model>) -> DateTimeUtc {
    vec.iter()
        .max_by_key(|x| x.updated_at)
        .unwrap()
        .updated_at
        .and_utc()
}
pub async fn get_updated_time_db_pesable(vec: Vec<entity::pesable::Model>) -> DateTimeUtc {
    vec.iter()
        .max_by_key(|x| x.updated_at)
        .unwrap()
        .updated_at
        .and_utc()
}
pub async fn get_updated_time_db_rubro(vec: Vec<entity::rubro::Model>) -> DateTimeUtc {
    vec.iter()
        .max_by_key(|x| x.updated_at)
        .unwrap()
        .updated_at
        .and_utc()
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
