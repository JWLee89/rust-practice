use std::collections::HashMap;
mod dummy_projects;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize>= HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        match map.get(&(target - num)) {
            Some(old_value_index) => return vec![*old_value_index as i32, i as i32],
            None => map.insert(num, i),
        };
    }
    unreachable!()
}

struct Person {
    name: String,
    age: u8,
    country: String,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8);
    fn country(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8) {
        (&self.name, self.age)
    }

    fn country(&self) -> &str {
        &self.country
    }
}

pub fn square<T>(x: T) -> T 
    where T: std::ops::Mul<Output = T> + Copy{
    x * x
}

struct Point<T, U> {
    x: T,
    y: U
}

impl <T, U> Point <T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn print(&self) {
        println!("The values of the coordinates are: {} and {}", self.x, self.y);
    }
}

fn main() {
   
}
