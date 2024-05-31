use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path)
        .expect(&format!("Cannot read the file: {}", config.file_path));

    println!("With text:\n{contents}");
}
