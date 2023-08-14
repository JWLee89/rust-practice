
pub(crate) fn loops() {
    /*loop {
        println!("this is an infinite loop");
    }*/
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("i : {i}");
    }

    for i in 0..=5 {
        println!("i = {i}");
    }

    let mut some_vec = vec![45, 30, 85, 90, 41, 39];
    // This changes the ownership of the variables inside of the vector
    // to i.
    // The scope of i is limited to the body inside of the for loop
    // To prevent change in ownership, use the iter() function.
    // Iter changes the value of i32 to &i32 (reference)
    for i in some_vec.iter_mut() {
        // This can be used to update values inside of the vector
        // Watch out though, this can be dangerous
        *i = 10;
    }
    // Print will raise error
    println!("{}, ", some_vec[0]);
}