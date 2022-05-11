use std::{env, process};
use mini_grep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(err) = mini_grep::run(config) {
        eprintln!("Application error: {}", err);

        process::exit(1);
    }
}
