extern crate blue_db;
extern crate diesel;

use blue_db::blue::Blue;

fn main() {
    let blue = Blue::new();

    let result = blue.create_category(&"Tests");
    println!("The newly created category has the id: {}", result.id);
}