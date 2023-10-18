use crate::error::Error;
use crate::expression::Expr;
use crate::scanner::{Token, TokenType};
use crate::statement::Stmt;
use crate::value::Value;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.match_token(TokenType::Function) {
            return self.function();
        }
        if self.match_token(TokenType::Local) {
            return self.local_declaration();
        }
        self.statement()
    }

    fn function(&mut self) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, "expect function name")?
            .clone();
        let _ = self.consume(TokenType::LeftParen, "expect '(' after function name")?;
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            parameters.push(
                self.consume(TokenType::Identifier, "expect parameter name")?
                    .clone(),
            );
            while self.match_token(TokenType::Comma) {
                parameters.push(
                    self.consume(TokenType::Identifier, "expect parameter name")?
                        .clone(),
                );
            }
        }
        let _ = self.consume(TokenType::RightParen, "expect ')' after parameters")?;
        let body = self.block()?;
        Ok(Stmt::FunctionStmt(name, parameters, body))
    }

    fn local_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, "expect variable name")?
            .clone();
        let mut initializer = Expr::None;
        if self.match_token(TokenType::Equal) {
            initializer = self.expression()?;
        }
        let _ = self.consume(
            TokenType::Semicolon,
            "expect ';' after variable declaration",
        )?;
        Ok(Stmt::LocalStmt(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(TokenType::If) {
            return self.if_statement();
        }
        if self.match_token(TokenType::Print) {
            return self.print_statement();
        }
        if self.match_token(TokenType::Return) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        let condition = self.expression()?;
        let _ = self.consume(TokenType::Then, "expect 'then' after condition")?;
        let then_branch = self.statement()?;
        let mut else_branch = Stmt::None;
        if self.match_token(TokenType::Else) {
            else_branch = self.statement()?;
        }
        let _ = self.consume(TokenType::End, "expect 'end' after if body")?;
        Ok(Stmt::IfStmt(
            condition,
            Box::new(then_branch),
            Box::new(else_branch),
        ))
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let _ = self.consume(TokenType::LeftParen, "expect '(' after print")?;
        let value = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "expect ')' after print expr")?;
        let _ = self.consume(TokenType::Semicolon, "expect ';' after value")?;
        Ok(Stmt::PrintStmt(value))
    }

    fn return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword = self.previous().clone();
        let mut value = Expr::None;
        if !self.check(TokenType::Semicolon) {
            value = self.expression()?;
        }
        let _ = self.consume(TokenType::Semicolon, "expect ';' after return value")?;
        Ok(Stmt::ReturnStmt(keyword, value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "expect ';' after return value")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.check(TokenType::End) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        let _ = self.consume(TokenType::End, "expect 'end' after block")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.equality()?;
        if self.match_token(TokenType::Equal) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            return match expr {
                Expr::Variable(name) => Ok(Expr::Assign(name, Box::new(value))),
                _ => Err(Error::ParseError(format!(
                    "{:?} invalid assignment target",
                    equals
                ))),
            };
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;
        if self.match_token(TokenType::LeftParen) {
            expr = self.finish_call(expr)?;
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments = Vec::new();
        arguments.push(self.expression()?);
        while self.match_token(TokenType::Comma) {
            arguments.push(self.expression()?);
        }
        let paren = self
            .consume(TokenType::RightParen, "expect ')' after arguments")?
            .clone();
        Ok(Expr::Call(Box::new(callee), paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        // TODO: 暂时只支持 number
        if self.match_token(TokenType::Number) {
            return Ok(Expr::Literal(self.previous().value.clone()));
        }
        if self.match_token(TokenType::Nil) {
            return Ok(Expr::Literal(Value::Nil));
        }
        if self.match_token(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        // TODO: 暂时不支持 grouping，即 (1 + 2)
        Err(Error::ParseError(format!("expect expression")))
    }

    fn consume(&mut self, typ: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(typ) {
            return Ok(self.advance());
        }
        Err(Error::ParseError(format!(
            "parse at '{:?}' err: {}",
            self.peek(),
            message
        )))
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
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn test_parse_expr() {
        let mut scanner = Scanner::new("local a = 1 + 2 * 3 - 4;".to_string());
        let tokens = scanner.scan_tokens().unwrap();
        println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 12);

        let mut parser = Parser::new(tokens.clone());
        let stmts = parser.parse().unwrap();
        println!("{:#?}", stmts);
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_expr_err() {
        let mut scanner = Scanner::new("a = 1 + 2 * 3 - 4;".to_string());
        let tokens = scanner.scan_tokens().unwrap();
        println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 11);

        let mut parser = Parser::new(tokens.clone());
        let stmts = parser.parse().unwrap();
        println!("{:#?}", stmts);
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_func() {
        let source = r#"
        function fib(n)
          if n < 2 then
            return n;
          end

          local n1 = fib(n-1);
          local n2 = fib(n-2);
          return n1 + n2;
        end

        print(fib(4));
        "#;

        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 49);

        let mut parser = Parser::new(tokens.clone());
        let stmts = parser.parse().unwrap();
        println!("{:#?}", stmts);
        assert_eq!(stmts.len(), 2);
        assert_eq!(stmts[0].as_function_stmt().unwrap().0.raw, "fib");
    }
}
