use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
    let Config { query, file_path } = config.unwrap();
    if let Some(config) = ;
    println!("Searching for {query}");
    println!("In file path {file_path}");

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read file.");
    // println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
