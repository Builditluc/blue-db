use blue_db::Blue;
use std::io::stdin;

fn main() {
    let blue = Blue::new();

    println!("Please enter your title for the new category");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];

    let category = blue.create_category(title);
    println!("Saved the category with id {}", category.id);
}