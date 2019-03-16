pub mod lexer;
use crate::lexer::Lexer;

pub mod parser;
use crate::parser::Parser;

pub mod tokens;

extern crate regex;

fn main() {
    let parse_result = Parser::parse(String::from("2342+2343*226/32"));
    match parse_result {
        Ok(node) => println!("{:?}", node),
        Err(msg) => println!("{}", msg),
    }
}
