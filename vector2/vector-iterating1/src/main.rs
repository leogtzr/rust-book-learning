fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut ages = vec![1, 2, 3, 4, 5];
    for i in &mut ages {
        *i += 50;
    }

    println!("{:?}", ages);
}
