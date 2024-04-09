fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("V: {:?}", v);

    let v2 = vec![1, 2, 3, 4, 5, 6];
    println!("V2: {:?}", v2);

    let third = v2[2];
    println!("The third element is: {}", third);

    let thirdE1: &i32 = &v[2];
    let val = 10 * (*thirdE1);

    let third: Option<&i32> = v.get(2);

    println!("Arithm: {}", val);

    match third {
        None => {}
        Some(someVal) => {
            println!("Some value = {}", someVal);
        }
    }

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let does_not_exist = &nums[123];
    let does_not_exist = nums.get(123);
    match does_not_exist {
        None => {},
        Some(theVal) => {}
    }
}
