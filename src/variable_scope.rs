
pub(crate) fn square(number: i32) -> i32 {
    number * number
}

pub(crate) fn strings_heap() {
    let mut s1 = String::from("yee");
    // We cannot borrow s1 here.
    // let s2 = &s1;

    // Clone creates a new string value on the heap
    // and assigns it to s3.
    let mut s3 = s1.clone();
    s1.push_str("yee");
    s3.push_str(" -- cowbell");

    let s2 = &s1;

    println!("s1: {s1}, s2: {s2}, s3: {s3}")
}
