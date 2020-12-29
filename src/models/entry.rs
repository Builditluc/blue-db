use crate::models::category::Category;
use crate::schema::entries;

use diesel::associations::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[belongs_to(Category)]
#[table_name="entries"]
pub struct Entry {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub body: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct NewEntry<'a> {
    pub id: &'a str,
    pub category_id: &'a str,
    pub title: &'a str,
    pub body: &'a str,

    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime
}

type AllColumns = (
    entries::id,
    entries::category_id,
    entries::title,
    entries::body,
    entries::created_at,
    entries::updated_at
);

const ALL_COLUMNS: AllColumns = (
    entries::id,
    entries::category_id,
    entries::title,
    entries::body,
    entries::created_at,
    entries::updated_at
);

type All = diesel::dsl::Select<entries::table, AllColumns>;
type WithTitle<'a> = diesel::dsl::Eq<entries::title, &'a str>;
type WithId<'a> = diesel::dsl::Eq<entries::id, &'a str>;
type ByTitle<'a> = diesel::dsl::Filter<All, WithTitle<'a>>;
type ById<'a> = diesel::dsl::Filter<All, WithId<'a>>;

fn with_title(title: &str) -> WithTitle {
    entries::title.eq(title)
}
fn with_id(id: &str) -> WithId { entries::id.eq(id) }

impl Entry {
    pub fn all() -> All { entries::table.select(ALL_COLUMNS) }
    pub fn by_id(id: &str) -> ById { Self::all().filter(with_id(id)) }
    pub fn by_title(title: &str) -> ByTitle {
        Self::all().filter(with_title(title))
    }
}