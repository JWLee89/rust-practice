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

pub(crate) fn stack_fn(mut num: i32) {
    num = 567;
    println!("num: {num}")
}

pub(crate) fn stack_fn_vec(vector: &mut Vec<i32>) {
    // once a vector is passed, the new owner of the variable
    // is now "vector" variable stored in the stack, which
    // points to the vector values stored on the heap
    vector.push(-1);

    // 
    let mut yee: Vec<i32> = vec![4, 5, 6];
    
    // let ref1 = &mut yee;
    // // only one mutable reference at a time
    // let ref2 = &mut yee;
    // println!("ref1: {:?}, ref2: {:?}", ref1, ref2)
    // println!("ref1 = {ref1}, ref2 = {ref2}");
    
    // Rule 2: many mutable references can exist
    // let ref1 = &yee;
    // let ref2 = &yee; 
    // println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    
    // Rule 3. Mutable and immutable cannot co-exist
    // let ref1 = &yee;
    // // Cannot borrow from mutable as immutable ref already exists.
    // let ref2 = &mut yee;
    // println!("Mutable cannot co-exist: {:?} yee {:?}", ref1, ref2);

    // Rule 4: You can have mutable and immutable references
    // only inside of a scope.
    // The scope of reference is from when 
    // it is initialized to when it is last used.

    // let ref1 = &yee;
    // let ref2 =  &yee;
    // println!("Mutable cannot co-exist: {:?} yee {:?}", ref1, ref2);
    // // ref1, ref2 is now out of scope. 
    // // Since there are no more immutable references, we can create 
    // // a mutable reference.
    // let ref3 = &mut yee;
    // println!("I can now make a mutable reference: {:?}", ref3);

    // // Last rule: Data should not change when immutable references are in scope!!
    // let ref1 = &yee;
    // yee.push(100);
    // // We should not change original data when immutable reference is in scope.
    // println!("Yee: {:?}", ref1);


}
