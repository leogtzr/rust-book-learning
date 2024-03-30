enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Calling something...");
    }
}

fn main() {
    let m = Message::Write(String::from("Hello, World"));
    m.call();
}
