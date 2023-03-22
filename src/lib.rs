use std::{env, io::Error};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

pub fn parse_config(args: &Vec<String>) -> Config {
    // we need the & because implicit moving out of a vector is not allowed, expressions in rust move ownership
    let query = &args[1];
    let file_path = &args[2];

    Config {
        query,
        file_path,
    }
}

// pub fn parse_arguments<'a>() -> Result<(&'a String, &'a String), Error> {
//     let args_collection: Vec<String> = env::args().collect();

//     // we need the & because implicit moving out of a vector is not allowed, expressions in rust move ownership
//     let query = match &args_collection.get(1) {
//         Ok(val) => val,
//         (_) => Err()
//     };
//     let file_path = &args_collection[2];
// }

// pub fn parse_file() {}
