#![allow(unused)]
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
mod bdd;

#[tokio::main]

async fn main() {
    //let pool = bdd::bdd::new_db_pool("localhost", "youpgres", "youpgres", "azertyui", 5);
    bdd::requete();
}
