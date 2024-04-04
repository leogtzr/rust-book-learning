use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IOResult;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);

    println!("Value = {:?}", map);
}