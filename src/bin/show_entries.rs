extern crate blue_db;
extern crate diesel;

use self::blue_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use blue_db::schema::entries::dsl::*;

    let connection = establish_connection();
    let results = entries
        .limit(5)
        .load::<Entry>(&connection)
        .expect("Error loading entries");

    println!("Displaying {} entries", results.len());
    for entry in results {
        println!("{}, created {} UTC", entry.title, entry.created_at.to_string());
        println!("----------\n");
        println!("{}", entry.body);
    }
}