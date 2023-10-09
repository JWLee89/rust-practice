

/**
 * Two types of strings in Rust:
 * 1. &str: Static string (stack allocated)
 * 2. String: growable string (heap allocated)
 */
pub fn basic_str() {
    let mut static_str = "A static string";
    let mut growable = String::from("A growable string");
    growable += " with more text";
    // static_str += "yee";
    println!("Growable string: {}", growable);
}
