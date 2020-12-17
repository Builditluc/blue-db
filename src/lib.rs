#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::Utc;
use uuid::Uuid;

use crate::models::Entry;
use crate::models::NewEntry;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub fn create_entry<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Entry {
    use schema::entries;

    let new_entry = NewEntry {
        title,
        body,
        timestamp: &Utc::now().naive_utc(),
        entry_id: &Uuid::new_v4().to_string()
    };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving a new entry")
}