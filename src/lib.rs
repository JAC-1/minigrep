use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&contents, &config.query)
    } else {
        search(&contents, &config.query)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // Before learning about itterator adapters...
    // let mut result = vec![];
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line.trim())
    //     }
    // }
    // result
}

pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

    // let mut result = vec![];
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line.trim())
    //     }
    // }
    // result
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn:t get a file path."),
        };
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "fast";
        let contents = "\
            Rust:
            safe, fast, productive.
            pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(contents, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "FaST";
        let contents = "\
            Rust:
            safe, fast, productive.
            pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(contents, query)
        );
    }
}
