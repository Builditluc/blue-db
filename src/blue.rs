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
            id: &Uuid::new_v4().to_string(),
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

        diesel::delete(entries::table.filter(entries::id.eq(entry_id)))
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

impl Blue {
    pub fn get_entries(&self) -> Vec<Entry> {
        Entry::all()
            .load::<Entry>(&self.conn)
            .expect("Error selecting an entry")
    }
    pub fn get_entry_by_title(&self, title: &str) -> Vec<Entry> {
        Entry::by_title(title)
            .load::<Entry>(&self.conn)
            .expect("Error selecting an entry")
    }
    pub fn get_entry_by_id(&self, entry_id: &str) -> Entry {
        Entry::by_id(entry_id)
            .first::<Entry>(&self.conn)
            .expect("Error selecting an entry")
    }
    pub fn get_entries_by_category(&self, category_id: &str) -> Vec<Entry> {
        Entry::belonging_to(&self.get_category_by_id(category_id))
            .load::<Entry>(&self.conn)
            .expect("Error selecting an entry")
    }
}

impl Blue {
    pub fn get_categories(&self) -> Vec<Category> {
        Category::all()
            .load::<Category>(&self.conn)
            .expect("Error selecting an category")
    }
    pub fn get_category_by_title(&self, title: &str) -> Vec<Category> {
        Category::by_title(title)
            .load::<Category>(&self.conn)
            .expect("Error selecting an category")
    }
    pub fn get_category_by_id(&self, id: &str) -> Category {
        Category::by_id(id)
            .first::<Category>(&self.conn)
            .expect("Error selecting an category")
    }
    pub fn get_category_by_entry(&self, entry: &Entry) -> Category {
        self.get_category_by_id(&entry.category_id)
    }
}

impl Blue {
    pub fn update_entry(&self, entry: &Entry) -> Entry {
        use crate::schema::entries;

        diesel::update(&self.get_entry_by_id(&entry.id))
            .set((
                entries::title.eq(&entry.title),
                entries::body.eq(&entry.body),
                entries::updated_at.eq(&Utc::now().naive_utc())
                ))
            .get_result(&self.conn)
            .expect("Error updating an entry")
    }

    pub fn update_category(&self, category: &Category) -> Category {
        use crate::schema::categories;

        diesel::update(&self.get_category_by_id(&category.id))
            .set((
                categories::title.eq(&category.title),
                categories::updated_at.eq(&Utc::now().naive_utc())
                ))
            .get_result(&self.conn)
            .expect("Error updating an category")
    }
}