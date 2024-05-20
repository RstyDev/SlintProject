use dotenvy::dotenv;

use sqlx::{Executor, Pool, Sqlite};
use tauri::async_runtime::block_on;

pub fn up(db:&Pool<Sqlite>){
    dotenv().ok();
    // let db= block_on(SqlitePool::connect(env::var("DATABASE_URL").expect("DATABASE must be set").as_str())).expect("Error connectando a la DB");

    let algo= block_on(db.execute(sqlx::query("CREATE TABLE IF NOT EXISTS productos (
        id integer PRIMARY KEY AUTOINCREMENT,
        nombre string
    )"))).unwrap();
    // block_on(db.execute(sqlx::query!(
    //     "
    //     select * from productos
    //     "
    // )));
}


pub fn down(db:&Pool<Sqlite>){
    // let db= block_on(SqlitePool::connect(env::var("DATABASE_URL").expect("DATABASE must be set").as_str())).expect("Error connectando a la DB");
    let algo= block_on(db.execute(sqlx::query("drop table if exists productos
    "))).unwrap();
}

pub fn fresh(db:&Pool<Sqlite>){
    down(db);
    up(db);
}