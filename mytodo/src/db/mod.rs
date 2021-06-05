pub mod models;
pub mod schema;

use diesel::{prelude::*, sqlite::SqliteConnection};

pub fn establish_connection() -> SqliteConnection{
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_|panic!("无法连接数据库"))

}
pub fn create_task(connect: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };
    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connect)
        .expect("Error inserting new Task");
}
pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}
pub fn remove_task(connection: &SqliteConnection, param: &str) {
    use self::schema::task::dsl::*;
    let pattern = format!("%{}%", param);
    diesel::delete(task.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error delete task");
}