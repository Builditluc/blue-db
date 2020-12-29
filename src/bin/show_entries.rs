extern crate blue_db;

use blue_db::blue::Blue;

fn main() {
    let blue = Blue::new();

    let entries = blue.get_entries();
    println!("Showing {} entries", entries.len());
    for entry in entries.iter() {
        println!("{} | Last edited: {} | id: {}", entry.title, entry.updated_at, entry.id);
        println!("--------------------");
        println!("{}", entry.body);
    }
}