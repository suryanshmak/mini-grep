use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    run(config)
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename).expect("Couldn't read the file");

    println!("Text of file: {}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}