use super::schema::entries;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub body: String,

    pub timestamp: NaiveDateTime,
    pub entry_id: String
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct NewEntry<'a> {
    pub title: &'a str,
    pub body: &'a str,

    pub timestamp: &'a NaiveDateTime,
    pub entry_id: &'a str
}