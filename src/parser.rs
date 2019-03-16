use crate::lexer::Lexer;
use crate::token::*;

#[derive(Debug)]
pub struct ExprNode {
    token: Token,
    pub left: Option<Box<ExprNode>>,
    pub right: Option<Box<ExprNode>>,
}

impl ExprNode {
    pub fn token(&self) -> Token {
        return self.token;
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(input: String) -> Result<ExprNode, String> {
        let lex_result = Lexer::tokenize(&input);
        match lex_result {
            Ok(tokens) => {
                let mut parser = Parser { tokens, pos: 0 };
                return parser.parse_expr();
            }
            Err(msg) => return Err(msg),
        };
    }

    fn parse_expr(&mut self) -> Result<ExprNode, String> {
        let expression = self.parse_op().or(self.parse_num())?;
        return Ok(expression);
    }

    fn parse_op(&mut self) -> Result<ExprNode, String> {
        let node = self.parse_num().and_then(|x| {
            self.accept(Token::TokenAdd)
                .or(self.accept(Token::TokenSub))
                .or(self.accept(Token::TokenMul))
                .or(self.accept(Token::TokenDiv))
                .map_err(|t| format!("Unexpected token {:?} when parsing operation", t))
                .and_then(|y| {
                    return self.parse_expr().and_then(|z| {
                        return Ok(ExprNode {
                            token: y,
                            left: Some(Box::new(x)),
                            right: Some(Box::new(z)),
                        });
                    });
                })
        });

        return node;
    }

    fn parse_num(&mut self) -> Result<ExprNode, String> {
        match self.accept(Token::TokenNum(0)) {
            Ok(num) => {
                return Ok(ExprNode {
                    token: num,
                    left: None,
                    right: None,
                });
            }
            Err(Some(tok)) => {
                return Err(format!("Expecting TokenNum but received {:?}", tok));
            }
            _ => return Err(format!("Failed to parse number")),
        }
    }

    fn accept(&mut self, token: Token) -> Result<Token, Option<Token>> {
        let current = self.tokens.get_mut(self.pos);
        match current {
            Some(tok) => {
                let mut result = Err(Some(*tok));
                if std::mem::discriminant(tok) == std::mem::discriminant(&token) {
                    result = Ok(*tok);
                    self.next();
                }

                return result;
            }
            None => {
                self.backtrack();
                return Err(None);
            }
        }
    }

    fn backtrack(&mut self) {
        self.pos = self.pos - 1;
    }

    fn next(&mut self) {
        self.pos = self.pos + 1;
    }
}
