use std::env;
use std::process;

use rust_grepcli::{Config, run};

fn main() {
    // let mut args = env::args().peekable();

    // while args.peek().is_some() {
    //     let test = args.next().unwrap();
    //     println!("{}", test);
    // }

    // `collect` is one of the few methods un Rust that we need to annotate types as it can't be inferred
    let args_collection: Vec<String> = env::args().collect();

    let config = Config::build(&args_collection).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // dbg!(args_collection);

    // Because `run` doesn't return a value we only care about 
    // detecting the error, so we don't need the `unwrap_or_else` to return unwrapped values
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
