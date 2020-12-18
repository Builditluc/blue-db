use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::associations::*;

#[derive(Queryable, Associations)]
#[table_name="entries"]
#[belongs_to(Category)]
pub struct Entry {
    pub id: i32,
    pub entry_id: String,
    pub category_id: i32,
    pub title: String,
    pub body: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct NewEntry<'a> {
    pub entry_id: &'a str,
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

impl Entry {
    fn all() -> All {
        entries::table
            .select(ALL_COLUMNS)
    }

    fn with_title(title: &str) -> WithTitle {
        entries::title.eq(title)
    }

    fn by_title(title: &str) -> ByTitle {
        Self::all().filter(Self::with_title(title))
    }
}

#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub title: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub id: &'a i32,
    pub title: &'a String,

    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime
}