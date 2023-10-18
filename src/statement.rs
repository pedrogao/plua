use enum_as_inner::EnumAsInner;

use crate::expression::Expr;
use crate::scanner::Token;

// Stmt 语句 trait
#[derive(Debug, Clone, EnumAsInner)]
pub enum Stmt {
    PrintStmt(Expr),
    IfStmt(Expr, Box<Stmt>, Box<Stmt>),
    LocalStmt(Token, Expr),
    FunctionStmt(Token, Vec<Token>, Vec<Stmt>),
    ReturnStmt(Token, Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    None,
}
