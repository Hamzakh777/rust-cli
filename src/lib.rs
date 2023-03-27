use std::{fs, vec, env};
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // we need the & because implicit moving out of a vector is not allowed, expressions in rust move ownership
        let query = &args[1];
        let file_path = &args[2];

        // the functions for working with env variables are in the `env` module in the standard library
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
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

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    // using shadow variables 
    let query = query.to_lowercase();
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // the back-slash tells Rust not to add a newline character after the beginning of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}