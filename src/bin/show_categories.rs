use blue_db::Blue;

fn main() {
    let blue = Blue::new();

    let categories = blue.get_categories();
    println!("Showing {} categories", categories.len());
    for category in categories.iter() {
        println!("Found a category with the title: {} and the id: {}", category.title, category.id);
    }
}