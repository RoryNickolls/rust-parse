#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    TokenAdd,
    TokenSub,
    TokenMul,
    TokenDiv,
    TokenNum(u64),
}
