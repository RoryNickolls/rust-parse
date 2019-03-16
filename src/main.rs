pub mod lexer;
pub mod parser;
use crate::parser::Parser;

pub mod tokens;

extern crate regex;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Could not parse filename");
    let file_contents =
        fs::read_to_string(filename).expect(&format!("Failed to read file {}", filename));
    let parse_result = Parser::parse(file_contents);
    match parse_result {
        Ok(node) => println!("{:?}", node),
        Err(msg) => println!("{}", msg),
    }
}
