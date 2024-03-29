use std::collections::HashMap;

use substring::Substring;

use crate::error::Error;
use crate::value::Value;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    // (
    LeftParen,
    // )
    RightParen,
    // {
    LeftBrace,
    // }
    RightBrace,
    // ,
    Comma,
    // .
    Dot,
    // -
    Minus,
    // +
    Plus,
    // ;
    Semicolon,
    // /
    Slash,
    // *
    Star,

    // One or two character tokens.
    // !
    Bang,
    // !=
    BangEqual,
    // =
    Equal,
    // ==
    EqualEqual,
    // >
    Greater,
    // >=
    GreaterEqual,
    // <
    Less,
    // <=
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    // and
    And,
    // else
    Else,
    // false
    False,
    // function
    Function,
    // end
    End,
    // for
    For,
    // if
    If,
    // then
    Then,
    // nil
    Nil,
    // or
    Or,
    // print
    Print,
    // return
    Return,
    // true
    True,
    // local
    Local,
    // while
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub raw: String,
    pub value: Value,
    pub line: usize,
}

impl Token {
    pub fn new(typ: TokenType, raw: String, value: Value, line: usize) -> Self {
        Self {
            typ,
            raw,
            value,
            line,
        }
    }
}

pub struct Scanner {
    pub source: String,
    chars: Vec<char>,

    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        // 目前只支持英文，直接 chars
        let chars: Vec<char> = source.chars().collect();
        Self {
            source,
            chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("function".to_string(), TokenType::Function),
                ("end".to_string(), TokenType::End),
                ("for".to_string(), TokenType::For),
                ("if".to_string(), TokenType::If),
                ("then".to_string(), TokenType::Then),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("true".to_string(), TokenType::True),
                ("local".to_string(), TokenType::Local),
                ("while".to_string(), TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        // EOF token
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Value::Nil,
            self.line,
        ));

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                // 注释
                if self.match_char('/') {
                    // 跳过注释部分
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // 忽略空格
            '\n' => self.line += 1, // 换行
            '"' => self.string()?,  // 字符串
            'o' => {
                if self.match_char('r') {
                    self.add_token(TokenType::Or);
                }
            }
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    return Err(Error::ScanError(format!(
                        "Unexpected character '{}' at {}",
                        c, self.line
                    )));
                }
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::ScanError(format!(
                "Unterminated string at {}",
                self.line
            )));
        }
        self.advance(); // "
        let _sub = self.source.substring(self.start + 1, self.current - 1);
        // TODO: 目前只支持 int，所以加入 nil
        self.add_token2(TokenType::String, Value::Nil);
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // TODO: 支持小数点
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // 跳过.
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let sub = self.source.substring(self.start, self.current);
        let n = sub.parse::<i32>().unwrap(); // 目前只支持i32
        self.add_token2(TokenType::Number, Value::Int(n))
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let sub = self.source.substring(self.start, self.current);
        let typ = self
            .keywords
            .get(sub)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        self.add_token(typ);
    }

    fn add_token(&mut self, typ: TokenType) {
        self.add_token2(typ, Value::Nil)
    }

    fn add_token2(&mut self, typ: TokenType, val: Value) {
        let sub = self.source.substring(self.start, self.current);
        self.tokens
            .push(Token::new(typ, sub.to_string(), val, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        self.current += 1; // 只有 true 才++
        return true;
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.chars.len() {
            return '\0';
        }
        return self.chars[self.current + 1];
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.chars[self.current];
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        return c;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, TokenType};

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new("1+2*3-4".to_string());
        let tokens = scanner.scan_tokens().unwrap();
        println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].typ, TokenType::Number);
        assert_eq!(tokens[1].typ, TokenType::Plus);
        assert_eq!(tokens[7].typ, TokenType::Eof);
    }

    #[test]
    fn test_scan_tokens_with_mul_lines() {
        let script = r#"
        local a = 1 + 2;
        local b = 4 - 3;
        local c = a + b;
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 22);
        assert_eq!(tokens[0].typ, TokenType::Local);
        assert_eq!(tokens[7].typ, TokenType::Local);
        assert_eq!(tokens[14].typ, TokenType::Local);
    }
}
