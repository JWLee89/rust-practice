use std::time;

fn main() {
    // "!" is a macro, 
    // which is used to write other code
    // AKA meta-programming.
    println!("Hello, world!");

    // By default, variables are immutable
    // To make it mutable, we need to add "mut" as a prefix
    let mut data = 10;
    println!("{}", data);
    
    let n1 = 32;
    let n2 = 32.5;
    data = n1 + n2 as i32;
    println!("n3 = {data}");
    
    let some_string = "10213123";
    let mut growable_string = String::from("this string will grow");
    let tuple_element = (4000, "yee");
    let (salary, salary_value) = tuple_element;
    println!("salary: {salary}");

    let number_array = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", number_array);

    let mut array_with_same_elements = [0; 10];
    // Mutable
    array_with_same_elements[0] = 1;
    println!("array with same element: {:?}", array_with_same_elements.len());


    let subset_array = [0, 1, 2, 3, 4];
    let cow = &subset_array[0..=2];
    
}
