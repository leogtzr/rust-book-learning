use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file(filename: String) -> Result<String, io::Error> {
    let username_file = File::open(filename);
    let mut username = String::new();

    username_file.unwrap().read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2(filename: String) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(filename).unwrap().read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file3(filename: String) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    /*let option = text.lines().next();
    match option {
        None => None,
        Some(x) => {
            x.chars().last()
        }
    }*/

    // Si no hay "next", se regresa un None..
    text.lines().next()?.chars().last()
}

fn main() {
    let file_name = "hello.txt";

    match read_username_from_file(file_name.to_string()) {
        Ok(username) => {
            println!("Username = {}", username.replace("\n", ""));
        }
        Err(e) => {
            panic!("error = {}", e.to_string());
        }
    }
}
