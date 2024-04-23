use std::collections::HashSet;

fn main() {
    let nums: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5]);
    println!("Num of elements: {}", nums.len());
}