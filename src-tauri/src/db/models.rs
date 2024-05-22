use chrono::NaiveDateTime;


pub enum Model{
    MedioPago{ id: i64, medio: String},
    Caja{ id: i64, inicio: NaiveDateTime, cierre: Option<NaiveDateTime>, monto_inicio: f64, monto_cierre: Option<f64>, ventas_totales: f64, cajero: String },
    Cliente{ id: i64, nombre: String, dni: i32, limite: Option<f64>, activo: bool, time: NaiveDateTime },
    Config{ id: i64, politica: f64, formato: String, mayus: String, cantidad: i64 },
    Prov{ id:i64, nombre:String, contacto:i64, updated:NaiveDateTime, config:i64 }
}

// CREATE TABLE IF NOT EXISTS proveedores (
//             id integer PRIMARY KEY AUTOINCREMENT not null,
//             nombre string not null,
//             contacto bigint,
//             updated datetime,
//             config integer,
//             foreign key (config) references config(id)
//         )