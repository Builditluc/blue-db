extern crate blue_db;
extern crate diesel;

use self::blue_db::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use blue_db::schema::entries::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let conn = establish_connection();
    let num_deleted = diesel::delete(entries.filter(title.like(pattern)))
        .execute(&conn)
        .expect("Error deleting entries");

    println!("Deleted {} entries", num_deleted);
}