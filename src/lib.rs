use std::{fs};
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // we need the & because implicit moving out of a vector is not allowed, expressions in rust move ownership
        let query = &args[1];
        let file_path = &args[2];

        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    // by calling () inside Ok(), this is the idiomatic way to indicate that we are calling `run` for its side effects only
    Ok(())
} 

// the lifetime parameter specifies which argument lifetime is connected to the lifetime of the return value
// in other words: The data returned by the search function will live as long as the data passed into it
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // the back-slash tells Rust not to add a newline character after the beginning of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}