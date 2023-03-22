use std::env;
use std::fs;
use std::io::ErrorKind;
use rust_grepcli::parse_config;

fn main() {
    println!("Hello, world!");

    // let mut args = env::args().peekable();

    // while args.peek().is_some() {
    //     let test = args.next().unwrap();
    //     println!("{}", test);
    // }

    // `collect` is one of the few methods un Rust that we need to annotate types as it can't be inferred
    let args_collection: Vec<String> = env::args().collect();

    let config = parse_config(&args_collection);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // dbg!(args_collection);

    let file_as_string = fs::read_to_string(config.file_path).expect("e");
    println!("file as string {}", file_as_string);
}
