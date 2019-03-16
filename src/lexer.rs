use crate::token::*;

use std::iter::Peekable;

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(input: &String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut it = input.chars().peekable();
        while let Some(&c) = it.peek() {
            match c {
                '+' => {
                    it.next();
                    tokens.push(Token::TokenAdd);
                }
                '-' => {
                    it.next();
                    tokens.push(Token::TokenSub);
                }
                '*' => {
                    it.next();
                    tokens.push(Token::TokenMul);
                }
                '/' => {
                    it.next();
                    tokens.push(Token::TokenDiv);
                }
                '0'...'9' => {
                    let i = Lexer::get_number(c, &mut it).unwrap();
                    tokens.push(Token::TokenNum(i));
                }
                ' ' | '\n' | '\t' => {
                    it.next();
                }
                _ => {
                    return Err(format!("Lexical Error: Unexpected symbol '{}'", c));
                }
            };
        }
        Ok(tokens)
    }

    fn get_number(
        c: char,
        it: &mut Peekable<std::str::Chars>,
    ) -> Result<u64, std::num::ParseIntError> {
        let mut num = c.to_string();
        it.next();

        while let Some(&n) = it.peek() {
            if Lexer::is_digit(n) {
                num.push(n);
                it.next();
            } else {
                return num.parse::<u64>();
            }
        }

        return num.parse::<u64>();
    }

    fn is_digit(c: char) -> bool {
        return c == '0'
            || c == '1'
            || c == '2'
            || c == '3'
            || c == '4'
            || c == '5'
            || c == '6'
            || c == '7'
            || c == '8'
            || c == '9';
    }
}
