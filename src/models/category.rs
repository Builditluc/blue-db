use crate::schema::categories;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Category {
    pub id: String,
    pub title: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub id: &'a String,
    pub title: &'a String,

    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime
}