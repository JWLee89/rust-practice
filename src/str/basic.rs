

/**
 * Two types of strings in Rust:
 * 1. &str: Static string (stack allocated)
 * 2. String: growable string (heap allocated)
 */
pub fn basic_str() {
    let mut static_str = "A static string";
    let mut growable = String::from("A growable string");
    growable += " with more text";
    growable.push_str(" and even more text");

    let len_of_str = growable.len();

    // static_str += "yee";
    println!("Growable string: {}, len: {len}", growable);
    // is_empty = check if string is empty
    // length = length of string
    // bytes = number of bytes taken up by string
    // contains = see if it contains a substring
    // replace = replace a substring with another
    // trim() = Trim whitespaces.
}
