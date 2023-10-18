use std::collections::HashMap;

use crate::error::Error;
use crate::expression::Expr;
use crate::scanner::Token;
use crate::statement::Stmt;

// Resolver 语义解析
#[derive(Default)]
pub struct Resolver {
    scopes: Vec<HashMap<String, ()>>,
}

impl Resolver {
    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        self.begin_scope();
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::PrintStmt(expr) => self.resolve_print_stmt(expr)?,
            Stmt::IfStmt(_, _, _) => (),
            Stmt::LocalStmt(_, _) => (),
            Stmt::FunctionStmt(name, params, body) => self.resolve_func_stmt(name, params, body)?,
            Stmt::ReturnStmt(_, _) => (),
            Stmt::Expression(expr) => self.resolve_expr(expr)?,
            Stmt::Block(_) => (),
            Stmt::None => (),
        }
        Ok(())
    }

    fn resolve_print_stmt(&mut self, expr: &Expr) -> Result<(), Error> {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn resolve_func_stmt(
        &mut self,
        name: &Token,
        _params: &Vec<Token>,
        _body: &Vec<Stmt>,
    ) -> Result<(), Error> {
        self.define(name.raw.as_str());
        // others...
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Call(callee, paren, arguments) => {
                self.resolve_call_expr(callee, paren, arguments)?
            }
            Expr::Unary(_, _) => (),
            Expr::Variable(token) => {
                let found = self.find_define(token.raw.as_str());
                if !found {
                    return Err(Error::ResolveError(format!(
                        "{} identifier not found",
                        token.raw.as_str()
                    )));
                }
            }
            Expr::Assign(token, expr) => {
                let found = self.find_define(token.raw.as_str());
                if !found {
                    return Err(Error::ResolveError(format!(
                        "{} identifier not found",
                        token.raw.as_str()
                    )));
                }

                self.resolve_expr(expr)?;
            }
            Expr::Binary(_, _, _) => (),
            Expr::Literal(_) => (),
            Expr::None => (),
        }
        Ok(())
    }

    fn resolve_call_expr(
        &mut self,
        callee: &Box<Expr>,
        _paren: &Token,
        _arguments: &Vec<Expr>,
    ) -> Result<(), Error> {
        self.resolve_expr(callee.as_ref())?;
        Ok(())
    }

    fn define(&mut self, name: &str) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), ());
    }

    fn find_define(&self, name: &str) -> bool {
        self.scopes.last().unwrap().contains_key(name)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::resolver::Resolver;
    use crate::scanner::Scanner;

    #[test]
    fn test_resolve() {
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
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 2);

        let mut resolver = Resolver::default();
        let r = resolver.resolve(result.as_ref().unwrap());
        assert_eq!(r.is_err(), false);
    }

    #[test]
    fn test_resolve_err() {
        let source = r#"
        a = 1 + 2 + 3;
        "#;

        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 9);

        let mut parser = Parser::new(tokens.clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 1);

        let mut resolver = Resolver::default();
        let r = resolver.resolve(result.as_ref().unwrap());
        println!("{:#?}", r);
        assert_eq!(r.is_err(), true);
    }
}
