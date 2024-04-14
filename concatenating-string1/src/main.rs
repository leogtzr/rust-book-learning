use std::ops::Add;

fn main() {
    let mut s = "Leonardo".to_string();
    println!("The value is: {}", s);

    let s2 = String::from("Gutierrez");

    let s3 = s + &s2;
    let s4 = "Gutierrez";

    let s5 = String::from("Leonardo") + s4;

    println!("Value is: {}", s3);
    println!("Value is: {}", s5);

    let apellido = String::from("Leonardo").add(" Gutiérrez");
    println!("Value is: {}", apellido);

    // Using format!

    let nombre = String::from("Leonardo");
    let apellido = String::from("Gutierrez");
    let segundoApellido = String::from("Ramirez");

    let fullName = format!("{} {} {}", nombre, apellido, segundoApellido);
    println!("Full Name is: {}", fullName);
}
