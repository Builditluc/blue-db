use crate::models::entry::Entry;
use crate::models::entry::NewEntry;

use crate::models::category::Category;
use crate::models::category::NewCategory;

use diesel::prelude::*;
use diesel::pg::Pg;

use chrono::Utc;
use uuid::Uuid;

pub struct Blue {
    conn: PgConnection
}

impl Blue {
    pub fn new() -> Self {
        Blue {
            conn: Blue::establish_connection()
        }
    }

    fn establish_connection() -> PgConnection {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connection to {}", database_url))
    }
}

impl Blue {
    pub fn create_entry<'a>(&self, title: &'a str, body: &'a str, category_id: &'a str) -> Entry {
        use crate::schema::entries;

        let new_entry = NewEntry {
            entry_id: &Uuid::new_v4().to_string(),
            category_id,
            title,
            body,
            created_at: &Utc::now().naive_utc(),
            updated_at: &Utc::now().naive_utc()
        };

        diesel::insert_into(entries::table)
            .values(&new_entry)
            .get_result(&self.conn)
            .expect("Error saving a new entry")

    }

    pub fn delete_entry(&self, entry_id: &str) -> usize {
        use crate::schema::entries;

        diesel::delete(entries::table.filter(entries::entry_id.eq(entry_id)))
            .execute(&self.conn)
            .expect("Error deleting an existing entry")
    }

    pub fn create_category<'a>(&self, title: &'a str) -> Category {
        use crate::schema::categories;

        let new_category = NewCategory {
            id: &Uuid::new_v4().to_string(),
            title,
            created_at: &Utc::now().naive_utc(),
            updated_at: &Utc::now().naive_utc()
        };

        diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result(&self.conn)
            .expect("Error saving a new category")
    }

    pub fn delete_category(&self, category_id: &str) -> usize {
        use crate::schema::categories;

        diesel::delete(categories::table.filter(categories::id.eq(category_id)))
            .execute(&self.conn)
            .expect("Error deleting an existing category")
    }
}