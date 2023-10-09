use std::collections::HashMap;

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

fn main() {
    let mut nums: HashMap<i32, i32> = HashMap::new();
    let mut key = 2;
    let mut value = 10;
    let val = nums.raw_entry_mut(key);

    for (key, value) in nums.iter() {
        println!("Key: {key}, value: {value}");
    }

}
