fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();

    for x in v2 {
        println!("x: {}", x);
    }
    //v1.iter().map(|x| x + 1).for_each(|x| println!("Value: {}", x));
}
