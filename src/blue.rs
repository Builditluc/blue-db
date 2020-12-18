use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use chrono::Utc;

use crate::establish_connection;

use crate::models::Entry;
use crate::models::NewEntry;

pub struct Blue {
}

impl Blue {
    fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connection to {}", database_url))
    }
}

impl Blue {
    fn create_entry<'a>(title: &'a str, body: &'a str) -> Entry {
        use crate::schema::entries;

        let conn = &establish_connection();

        let new_entry = NewEntry {
            entry_id: &Uuid::new_v4().to_string(),
            title,
            body,
            created_at: &Utc::now().naive_utc(),
            updated_at: &Utc::now().naive_utc()
        };

        diesel::insert_into(entries::table)
            .values(&new_entry)
            .get_result(conn)
            .expect("Error saving a new entry")
    }

    /*fn delete_entry() -> usize {

    }

    fn create_category() -> Category {

    }

    fn delete_category() -> usize {

    }*/
}