mod ownership;

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
    println!("Hello, world!");
    let mut vec_num = vec![1, 2, 123,123_123, 12312, 1_209_523,1];
    vec_num.push(100);

    vec_num.remove(1);
    println!("yee: {:?}", vec_num.contains(&2));
    print_name("hello yee", 100);
    println!("yee: {}", multiply(2, 5));

    // let mut m = String::new();
    // std::io::stdin()
    //     .read_line(& mut m)
    //     .expect("failed to read results");

    ownership::ownership();
    
}
