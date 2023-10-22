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

fn main() {
    dummy_projects::bank::yee();
    // 
    let mut num = String::new();
    let yee = std::io::stdin()
        .read_line(&mut num)
        .expect("Invalid input");

    let num: i32 = num.trim().parse().expect("Invalid input. Should be a number");
    if num > 1 {
        println!("num greater than 1: {}", num);
    }

    let p1 = Person{
        name: String::from("Yee"),
        age: 8,
        country: String::from("China"),
    };

    let info = p1.info();
    println!("Info is: {:?}", info);


    match num {
        1 => {
            println!("Number is {}", num);
        },
        0|-1 => {
            println!("Number is 0 or -1");
        },
        2..=100 => {
            println!("Between 2 and 100");
        },
        _ => {
            println!("Other");
        }
    }
}
