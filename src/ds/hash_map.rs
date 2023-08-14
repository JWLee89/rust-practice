use std::collections::HashMap;

#[derive(Debug)]
pub enum Items {
    Price(f32),
}

pub fn get_map() -> HashMap<String, Items> {
    let mut map = HashMap::new();
    map.insert(String::from("price"), Items::Price(10.0));
    map.insert(String::from("teemo"), Items::Price(20.0));
        // Look for a specific review
    let price_key: &str = "price";
    println!("\nReview for {:?}", map.get(price_key));

    map
}
