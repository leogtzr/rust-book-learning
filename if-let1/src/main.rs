fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Using if-let syntax:
    if let Some(max) = config_max {
        println!("The value is = {}", max);
    }

    let x: Option<i32> = None;

    if let Some(val) = x {
        println!("Some value = {}", val);
    } else {
        println!("It is None");
    }
}
