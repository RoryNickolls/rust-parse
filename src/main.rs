pub mod lexer;
use lexer::Lexer;

extern crate regex;

fn main() {
    let lexer = Lexer::new();
    let tokens = lexer.tokenize(String::from("330+300"));
    println!("{:?}", tokens);
}
