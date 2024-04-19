use std::fs::File;
use std::io::{self, Error, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        // Recordar que estamos regresando algo aquí:
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn main() {
    let username_from_file = read_username_from_file();
    match username_from_file {
        Ok(username) => {
            println!("Username is: {}", username);
        }
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}
