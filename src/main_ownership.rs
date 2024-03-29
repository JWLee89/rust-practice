mod ownership;
mod variable_scope;
mod structure;

fn main() {
    // "!" is a macro, 
    // which is used to write other code
    // AKA meta-programming.
    

    ownership::ownership();
    println!("Square output: {}", variable_scope::square(2));
    variable_scope::strings_heap();
    let yee = 987;
    // Even though we updated the 
    // value inside of stack_fn, because it is a primitive,
    // it is stored on the stack. Thus, once it goes out of scope,
    // the local instance is deleted.
    // Only objects in the stack are maintained until the owner 
    // goes out of scope.
    ownership::stack_fn(yee);
    println!("Yee: {yee}");

    let mut numbers = vec![1, 2, 3, 4];
    
    // Need to pass by reference
    // &: Ownership will be maintained by "vector" 
    // inside stack_fn_vec
    // mut: Enable updating the vector
    // "&mut" - Pass in a mutable reference.
    ownership::stack_fn_vec(&mut numbers);
    println!("numbers: {:?}", numbers);

    let mut vec1 = vec!{1, 2, 3};
    // Correct syntax for mutable reference
    let ref1 = &mut vec1;
    ref1.push(4);
    println!("vec1: {:?}", vec1);

    // let s1 = String::from("yeeee");
    // // s2 now has the pointer to "yeeee" in the heap. s1 is dropped from the stack.
    // let s2 = s1;
    // // This will raise an exception
    // println!("s1: {s1}")

}

