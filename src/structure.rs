
pub struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

// Functions that we can have in structure
impl Person {
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}

// TODO inherit from person
pub struct User {
    username: String,
    password: String
}

impl User {
    fn login(&self, username: String, password: String) -> bool {
        self.username == username && self.password == password
    }
}


pub(crate) fn test() {
    println!("The values are {}", "nam")
}
