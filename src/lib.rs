use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = config.file_path;
    let query = config.query;
    let contents = fs::read_to_string(&path)?;
    // println!("Query: {query}\nFilepath: {path}\nFile contents: {contents}");
    for line in search(&query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The data being returned by vec will live as long as the contents string slice, 
    // but the query slice will not.
    // 'contents' contains all the information we need, and will return parts of it.

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim())
        }
    }

    results
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
   pub  fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "fast";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let empty_vec: Vec<&str> = Vec::new();
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        assert_eq!(empty_vec, search("poop", contents));
    }
}

