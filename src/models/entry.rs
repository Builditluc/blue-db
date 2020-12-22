use crate::models::category::Category;
use crate::schema::entries;

use diesel::associations::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Category)]
#[table_name="entries"]
pub struct Entry {
    pub id: i32,
    pub entry_id: String,
    pub category_id: String,
    pub title: String,
    pub body: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct NewEntry<'a> {
    pub entry_id: &'a str,
    pub category_id: &'a str,
    pub title: &'a str,
    pub body: &'a str,

    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime
}

type AllColumns = (
    entries::entry_id,
    entries::title,
    entries::body,
    entries::created_at,
    entries::updated_at
);

const ALL_COLUMNS: AllColumns = (
    entries::entry_id,
    entries::title,
    entries::body,
    entries::created_at,
    entries::updated_at
);

type All = diesel::dsl::Select<entries::table, AllColumns>;
type WithTitle<'a> = diesel::dsl::Eq<entries::title, &'a str>;
type ByTitle<'a> = diesel::dsl::Filter<All, WithTitle<'a>>;

fn with_title(title: &str) -> WithTitle {
    entries::title.eq(title)
}

impl Entry {
    pub fn all() -> All {
        entries::table.select(ALL_COLUMNS)
    }

    pub fn by_title(title: &str) -> ByTitle {
        Self::all().filter(with_title(title))
    }
}