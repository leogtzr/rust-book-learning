use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // This actually generates a warning.
    scores.insert(String::from("Blue"), 10);
    //scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
