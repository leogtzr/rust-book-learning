fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s  = data.to_string();
    println!("Value: {}", s);


    /// Pushing to a string
    let mut s1 = String::from("foo");

    let s2 = "bar";

    s1.push_str(s2);
    println!("s2 is {s2}, {s1}");
}
