use std::{env, process};
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Err(err) => {
            println!("Application error: {}", err);
            process::exit(1);
        }
        Ok(config) => config,
    };

    if let Err(err) = mini_grep::run(config) {
        println!("Application error: {}", err);

        process::exit(1);
    }
}
