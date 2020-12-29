use blue_db::Blue;
use std::io::{stdin, Read};

fn main() {
    let blue = Blue::new();

    println!("Please enter the title of the new entry");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("Ok! Now write {} (Press CTRL+D when finished)", title);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    println!("Now enter the id of the category");
    let mut entry_id = String::new();
    stdin().read_line(&mut entry_id).unwrap();
    let entry_id = &entry_id[..(entry_id.len() - 1)];

    let entry = blue.create_entry(title, &body, entry_id);
    println!("Saved your new entry with the id: {}", entry.id)
}