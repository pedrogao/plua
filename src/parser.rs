use std::borrow::Borrow;

use crate::scanner::{Token, TokenType};
use crate::statement::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result</*Vec<dyn Stmt>*/(), String> {
        Err("err".to_string())
    }

    fn consume(&mut self, typ: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(typ) {
            return Ok(self.advance());
        }
        Err(format!("parse at '{:?}' err: {}", self.peek(), message))
    }

    fn match_tokens(&mut self, types: Vec<TokenType>) -> bool {
        for typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.check(typ) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().typ == typ
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens[self.current].borrow()
    }

    fn previous(&self) -> &Token {
        self.tokens[self.current - 1].borrow()
    }
}