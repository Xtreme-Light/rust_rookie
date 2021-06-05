use diesel::{QueryResult, insert_into, RunQueryDsl};
use chrono::NaiveDateTime;
use diesel::sqlite::Sqlite;
use super::schema::weids;
use diesel::prelude::*;
#[derive(Queryable, PartialEq, Debug)]
pub struct Weid {
    pub id: i32,
    pub chain_id: i32,
    pub addr: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable)]
#[table_name = "weids"]
pub struct NewWeid {
    pub chain_id: i32,
    pub addr: String,
}


