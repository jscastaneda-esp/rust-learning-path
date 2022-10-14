use std::collections::HashMap;

fn hashmap() {
    // Mapas Hash
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate"),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    let book = "Programming in Rust";
    println!("Review for \'{}\': {:?}", book, reviews.get(book));

    let obsolete: &str = "Ancient Roman History";
    println!("\'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    println!("Review for \'{}\': {:?}", obsolete, reviews.get(obsolete))
}
