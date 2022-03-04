use crate::scanner::Token;
use crate::value::Value;

// Expr 表达式 trait
#[derive(Debug)]
pub enum Expr {
    Call(Box<Expr>, Token, Vec<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Value),
    None,
}