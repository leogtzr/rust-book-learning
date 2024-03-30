/*
    enum Option<T> {
        None,
        Some(T),
    }
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let five: Option<i32> = Option::Some(5);
    let six = plus_one(five);

    match six {
        None => {

        },
        Some(six) => {
            println!("Value = {}", six);
        }
    }
}
