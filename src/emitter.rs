use std::collections::HashMap;
use std::process::id;

use crate::bytecode::ByteCode;
use crate::expression::Expr;
use crate::scanner::{Token, TokenType};
use crate::statement::Stmt;
use crate::value::Value;

#[derive(Default)]
pub struct Emitter {
    scopes: Vec<HashMap<String, usize>>,
    bytecodes: Vec<ByteCode>,
}

impl Emitter {
    pub fn emit(&mut self, statements: &Vec<Stmt>) -> Result<&Vec<ByteCode>, String> {
        for stmt in statements {
            self.emit_stmt(stmt)?;
        }
        Ok(self.bytecodes.as_ref())
    }

    fn emit_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::PrintStmt(expr) => self.emit_expr(expr)?,
            Stmt::IfStmt(condition, then_branch, else_branch)
            => self.emit_if_stmt(condition, then_branch.as_ref(), else_branch.as_ref())?,
            Stmt::LocalStmt(name, init) => self.emit_local_stmt(name, init)?,
            Stmt::FunctionStmt(name, params, body)
            => self.emit_func_stmt(name, params, body)?,
            Stmt::ReturnStmt(keyword, value) => self.emit_return_stmt(keyword, value)?,
            Stmt::Expression(expr) => self.emit_expr(expr)?,
            Stmt::Block(stmts) => self.emit_block(stmts)?,
            Stmt::None => self.emit_nop()?,
        }

        Ok(())
    }

    fn emit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.emit_stmt(stmt)?;
        }
        Ok(())
    }

    fn emit_return_stmt(&mut self, _keyword: &Token, value: &Expr) -> Result<(), String> {
        self.bytecodes.push(ByteCode::Ret);
        self.emit_expr(value)?;
        Ok(())
    }

    fn emit_func_stmt(&mut self, name: &Token, params: &Vec<Token>,
                      body: &Vec<Stmt>) -> Result<(), String> {
        let idx = self.bytecodes.len();
        self.bytecodes.push(ByteCode::Jump(0));
        self.begin_scope();
        let mut i = 0;
        for param in params {
            self.bytecodes.push(ByteCode::SetArg(i));
            let idx = self.scopes.last().unwrap().len();
            self.define(param.raw.as_str(), idx);
            i += 1;
        }
        for stmt in body {
            self.emit_stmt(stmt)?;
        }
        self.end_scope();
        // 生成完后再更改
        let jmp = self.bytecodes.len();
        self.bytecodes[idx] = ByteCode::Jump(jmp);
        Ok(())
    }

    fn emit_local_stmt(&mut self, name: &Token, init: &Expr) -> Result<(), String> {
        let idx = self.scopes.last().unwrap().len();
        self.define(name.raw.as_str(), idx);
        self.emit_expr(init)?;
        Ok(())
    }

    fn emit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt,
                    else_branch: &Stmt) -> Result<(), String> {
        self.emit_expr(condition)?;
        self.emit_stmt(then_branch)?;
        self.emit_stmt(else_branch)?;
        Ok(())
    }

    fn emit_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Call(callee, paren, args)
            => self.emit_call(callee.as_ref(), paren, args)?,
            Expr::Unary(operator, right) => self.emit_unary(operator, right.as_ref())?,
            Expr::Variable(name) => self.emit_variable(name)?,
            Expr::Assign(name, value) => self.emit_assign(name, value)?,
            Expr::Binary(left, operator, right)
            => self.emit_binary(left.as_ref(), operator, right.as_ref())?,
            Expr::Literal(value) => self.emit_literal(value)?,
            Expr::None => (),
        }

        Ok(())
    }

    fn emit_literal(&mut self, val: &Value) -> Result<(), String> {
        self.bytecodes.push(ByteCode::Push(val.clone()));
        Ok(())
    }

    fn emit_binary(&mut self, left: &Expr, operator: &Token,
                   right: &Expr) -> Result<(), String> {
        self.emit_expr(left)?;
        self.emit_expr(right)?;
        // left op right
        match operator.typ {
            TokenType::Equal => self.bytecodes.push(ByteCode::EqualEqual),
            TokenType::Greater => self.bytecodes.push(ByteCode::Greater),
            TokenType::Less => self.bytecodes.push(ByteCode::Less),
            _ => {
                return Err(format!("{:?} operator not support", operator.typ));
            }
        }
        Ok(())
    }

    fn emit_assign(&mut self, name: &Token, value: &Expr) -> Result<(), String> {
        Ok(())
    }

    fn emit_variable(&mut self, name: &Token) -> Result<(), String> {
        let idx = self.get_define(name.raw.as_str());
        self.bytecodes.push(ByteCode::GetArg(idx));
        Ok(())
    }

    fn emit_unary(&mut self, operator: &Token, right: &Expr) -> Result<(), String> {
        Ok(())
    }

    fn emit_call(&mut self, callee: &Expr, paren: &Token,
                 args: &Vec<Expr>) -> Result<(), String> {

        self.bytecodes.push(ByteCode::Call(1));
        Ok(())
    }

    fn emit_nop(&mut self) -> Result<(), String> {
        self.bytecodes.push(ByteCode::Noop);
        Ok(())
    }

    fn define(&mut self, name: &str, idx: usize) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), idx);
    }

    fn get_define(&self, name: &str) -> usize {
        *self.scopes.last().unwrap().get(name).unwrap()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}

mod tests {
    use crate::emitter::Emitter;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn test_emit_func() {
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
        let tokens = scanner.scan_tokens();
        println!("{:#?}", tokens.as_ref().unwrap());
        assert_eq!(tokens.as_ref().unwrap().len(), 49);

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 2);

        let mut emitter = Emitter::default();
        let r = emitter.emit(result.as_ref().unwrap());
        assert_eq!(r.is_err(), false);
        assert_eq!(r.unwrap().len(), 10);
    }
}
