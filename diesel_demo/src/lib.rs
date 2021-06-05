#[macro_use]
extern crate diesel;

extern crate dotenv;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::insert_into;
use crate::schema::weids::dsl::weids;
use crate::models::NewWeid;
use diesel::associations::HasTable;

pub mod schema;
pub mod models;
pub fn insert_default_values(conn: &SqliteConnection) -> QueryResult<usize> {
    insert_into(weids).default_values().execute(conn)
}
pub fn create_weid(conn: &SqliteConnection, c_id: i32, address: &str) -> usize {


    let new_weid = NewWeid {chain_id: c_id, addr: address.to_string()};

    diesel::insert_into(weids::table)
        .values(&new_weid)
        .execute(conn)
        .expect("Error saving new weid")
}
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}