use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();

    let some_file = File::open("file.txt").
        expect("file.txt should be included in this project.");
}
