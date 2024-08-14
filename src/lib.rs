/// # Minigrep
///
/// mini_grep is a learning project from **The Book**.
///
///  Commented out code is what was learned initially. The refactored version is currently in use.
use std::error::Error;
use std::{env, fs};

/// ## Runs the program:

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

/// Searches the contents of the found file and returns lines that contain the query term.
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

/// A case insensitive search function that shifts both the lines and the query to lowercase.

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
    /// Implementation build associated function that recieves a iterator that contains strings as a type object.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /// Next() skips the first argument as the first argument is created by the io stream.
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
