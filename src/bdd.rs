use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "youpgres";
const PG_ROOT_USER: &str = "youpgres";
const PG_ROOT_PWD: &str = "azertyui";
// app db
const PG_APP_DB: &str = "app_db";
const PG_APP_USER: &str = "app_user";
const PG_APP_PWD: &str = "app_pwd_to_change";
const PG_APP_MAX_CON: u32 = 5;

pub type Db = Pool<Postgres>;

struct Ticket {
    id: i64,
    name: String,
}

// pub async fn new_db_pool(
//     host: &str,
//     db: &str,
//     user: &str,
//     pwd: &str,
//     max_con: u32,
// ) -> Pool<Postgres> {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:password@localhost/test")
//         .await?;

//     return pool;
// }

// methode qui affiche le contenu de la table ticket++

pub async fn requete() {
    //let pool = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 2);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://youpgres:azertyui@localhost/youpgres")
        .await;
    let rows = sqlx::query("SELECT * FROM ticket").fetch_one(&pool).await;
    let str_result = rows
        .iter()
        .map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
        .collect::<Vec<String>>()
        .join(", ");
    println!("\n== select tickets with PgRows:\n{}", str_result);
}
