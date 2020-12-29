use crate::schema::categories;
use chrono::NaiveDateTime;

use diesel::prelude::*;

#[derive(Queryable, Identifiable)]
#[table_name="categories"]
pub struct Category {
    pub id: String,
    pub title: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub id: &'a str,
    pub title: &'a str,

    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime
}

type AllColumns = (
    categories::id,
    categories::title,
    categories::created_at,
    categories::updated_at
);

const ALL_COLUMNS: AllColumns = (
    categories::id,
    categories::title,
    categories::created_at,
    categories::updated_at
);

type All = diesel::dsl::Select<categories::table, AllColumns>;
type WithTitle<'a> = diesel::dsl::Eq<categories::title, &'a str>;
type WithId<'a> = diesel::dsl::Eq<categories::id, &'a str>;
type ByTitle<'a> = diesel::dsl::Filter<All, WithTitle<'a>>;
type ById<'a> = diesel::dsl::Filter<All, WithId<'a>>;

fn with_title(title: &str) -> WithTitle { categories::title.eq(title) }
fn with_id(id: &str) -> WithId { categories::id.eq(id) }

impl Category {
    pub fn all() -> All { categories::table.select(ALL_COLUMNS) }
    pub fn by_title(title: &str) -> ByTitle { Self::all().filter(with_title(title)) }
    pub fn by_id(id: &str) -> ById { Self::all().filter(with_id(id)) }
}