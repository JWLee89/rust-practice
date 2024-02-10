

fn main() {

    use rust_practice::Student;

    // It should be set automatically
    let std_1 = Student::new(String::from("John Doe")).unwrap_or_default();

    println!("std 1 {:?}", std_1);

}
