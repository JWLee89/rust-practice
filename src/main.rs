mod ownership;
mod variable_scope;

fn print_name(name: &str, salary: i32) {
    println!("My name is: {name}");
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn main() {
    // "!" is a macro, 
    // which is used to write other code
    // AKA meta-programming.
    

    ownership::ownership();
    println!("Square output: {}", variable_scope::square(2));
    variable_scope::strings_heap();
    let yee = 987;
    ownership::stack_fn(yee);
    println!("Yee: {yee}");
}
