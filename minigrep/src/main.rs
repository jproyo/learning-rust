use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
