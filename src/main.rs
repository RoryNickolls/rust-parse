pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;

extern crate regex;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Could not parse filename");
    let file_contents =
        fs::read_to_string(filename).expect(&format!("Failed to read file {}", filename));

    let interpret_result = interpreter::interpret(file_contents.clone());
    match interpret_result {
        Ok(num) => println!("{} = {}", file_contents, num),
        Err(msg) => println!("Runtime Error: {}", msg),
    }
}
