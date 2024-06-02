use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        //println!("Problem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //let _ = run(config);
    if let Err(e) = minigrep::run(config) {
        //println!("Application error: {}", e);
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

    /*run(config).unwrap_or_else(|e| {
        println!("Application error: {}", e);

        process::exit(1);
    });*/
}
