fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutability = || list.push(7);

    println!("Before calling closure: {:?}", list);
    borrows_mutability();

    println!("After defining closure: {:?}", list);
}
