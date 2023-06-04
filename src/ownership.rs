/**
 * Ownership
 * - Each value in rust has a variable that's called its owner
 * - There can only be one owner at the time
 * - When the owner goes out of scope, the value is dropped.
 * 
 */
pub(crate) fn ownership() {
    let s1 = String::from("yeeeee");
    // Moved ownership from s1 to s2.
    // Need to get reference to stop error
    // Which can be done using .clone or & (reference) operator
    let s2 = s1.clone();
    println!("s1: {s1}, s2: {s2}");
}
