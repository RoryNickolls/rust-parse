use crate::regex::Regex;

#[derive(Copy, Clone, Debug)]
pub enum Token {
    TokenAdd,
    TokenSub,
    TokenMul,
    TokenDiv,
    TokenNumber,
}

pub struct Lexer {
    token_expressions: Vec<(Regex, Token)>,
}

impl Lexer {
    pub fn new() -> Lexer {
        let mut token_expressions = Vec::new();
        token_expressions.push((Regex::new(r"^\+$").unwrap(), Token::TokenAdd));
        token_expressions.push((Regex::new(r"^\-$").unwrap(), Token::TokenSub));
        token_expressions.push((Regex::new(r"^\*$").unwrap(), Token::TokenMul));
        token_expressions.push((Regex::new(r"^/$").unwrap(), Token::TokenDiv));
        token_expressions.push((Regex::new(r"^\d+$").unwrap(), Token::TokenNumber));
        Lexer { token_expressions }
    }

    pub fn tokenize(&self, contents: String) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut remaining_contents = contents;
        while let Some(token) = self.next_token(&mut remaining_contents) {
            tokens.push(token);
            println!("{:?}, {}", token, remaining_contents);
        }
        tokens
    }

    fn next_token(&self, contents: &mut String) -> Option<Token> {
        let mut range = contents.len();
        while range > 0 {
            for (regex, tok) in self.token_expressions.iter() {
                if regex.is_match(&contents[0..range]) {
                    *contents = contents[range..].to_string();
                    return Some(*tok);
                }
            }
            range = range - 1;
        }
        return None;
    }
}
