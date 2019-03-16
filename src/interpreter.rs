use crate::parser;
use crate::token::Token;
use parser::ExprNode;

pub fn interpret(input: String) -> Result<u64, String> {
    let ast = parser::Parser::parse(input)?;
    interpret_expr(ast)
}

fn interpret_expr(expr: ExprNode) -> Result<u64, String> {
    match expr.token() {
        Token::TokenAdd | Token::TokenDiv | Token::TokenMul | Token::TokenSub => interpret_op(expr),
        Token::TokenNum(_) => interpret_num(expr),
    }
}

fn interpret_num(expr: ExprNode) -> Result<u64, String> {
    if let Token::TokenNum(x) = expr.token() {
        return Ok(x);
    }

    Err(format!("Could not read int"))
}

fn interpret_op(expr: ExprNode) -> Result<u64, String> {
    let token = expr.token();
    let left = expr.left.ok_or(format!("Error reading left expression"))?;
    let right = expr
        .right
        .ok_or(format!("Error reading right expression"))?;

    if let Token::TokenAdd = token {
        return Ok(interpret_expr(*left)? + interpret_expr(*right)?);
    } else if let Token::TokenSub = token {
        return Ok(interpret_expr(*left)? - interpret_expr(*right)?);
    } else if let Token::TokenDiv = token {
        let right_result = interpret_expr(*right)?;
        if right_result == 0 {
            return Err(format!("Divide by zero"));
        }
        return Ok(interpret_expr(*left)? / right_result);
    } else if let Token::TokenMul = token {
        return Ok(interpret_expr(*left)? * interpret_expr(*right)?);
    }

    Err(format!("Could not interpret expression"))
}
