use std::collections::HashMap;
use std::ptr::NonNull;

use crate::error::Error;
use crate::expression::Expr;
use crate::scanner::TokenType;
use crate::statement::Stmt;
use crate::value::Value;

type Link = Option<NonNull<Env>>;

#[derive(Debug)]
pub struct Env {
    values: HashMap<String, Value>,
    parent: Link,
}

impl Env {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Box<Env>) -> Self {
        let p = unsafe { NonNull::new_unchecked(Box::into_raw(parent)) };
        Self {
            values: HashMap::new(),
            parent: Some(p),
        }
    }

    pub fn new_ptr() -> NonNull<Self> {
        unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Self::new()))) }
    }

    pub fn new_ptr_with_parent(parent: NonNull<Env>) -> NonNull<Self> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                values: HashMap::new(),
                parent: Some(parent),
            })))
        }
    }

    pub fn define(&mut self, key: &str, value: Value) {
        self.values.insert(key.to_string(), value.clone());
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key).or_else(|| {
            if let Some(parent) = self.parent() {
                parent.get(key)
            } else {
                None
            }
        })
    }

    pub fn parent(&self) -> Option<&Env> {
        unsafe { self.parent.map(|node| &(*node.as_ptr())) }
    }

    pub fn parent_mut(&mut self) -> Option<&mut Env> {
        unsafe { self.parent.map(|node| &mut (*node.as_ptr())) }
    }
}

#[derive(Debug)]
pub struct Intercepter {
    current_env: NonNull<Env>,
}

impl Intercepter {
    pub fn new() -> Self {
        let mut global_env = Env::new_ptr();
        unsafe {
            global_env.as_mut().define("VERSION", Value::Int(1));
        }
        Self {
            current_env: global_env,
        }
    }

    pub fn eval(&mut self, statements: &Vec<Stmt>) -> Result<Value, Error> {
        for stmt in statements {
            let val = self.execute_stmt(stmt)?;
            if val != Value::Nil {
                return Ok(val);
            }
        }
        Ok(Value::Nil)
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Value, Error> {
        match stmt {
            Stmt::PrintStmt(expr) => {
                let value = self.execute_expr(expr)?;
                println!("{}", value);
                Ok(Value::Nil)
            }
            Stmt::IfStmt(condition, if_stmt, else_stmt) => {
                let condition = self.execute_expr(condition)?;
                if condition.is_truthy() {
                    self.execute_stmt(if_stmt)
                } else {
                    self.execute_stmt(else_stmt)
                }
            }
            Stmt::LocalStmt(token, expr) => {
                let value = self.execute_expr(expr)?;
                self.assign_variable(token.raw.as_str(), value)?;
                Ok(Value::Nil)
            }
            Stmt::FunctionStmt(name, params, block) => {
                let func = Value::Function(
                    name.raw.clone(),
                    params.iter().map(|p| p.raw.clone()).collect(),
                    block.clone(),
                );
                self.assign_variable(name.raw.as_str(), func)?;
                Ok(Value::Nil)
            }
            Stmt::ReturnStmt(_token, expr) => {
                let value = self.execute_expr(expr)?;
                Ok(value)
            }
            Stmt::Expression(expr) => self.execute_expr(expr),
            Stmt::Block(stmts) => self.execute_block(stmts, HashMap::new()),
            Stmt::None => Ok(Value::Nil),
        }
    }

    fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        params: HashMap<String, Value>,
    ) -> Result<Value, Error> {
        let mut value = Value::Nil;

        let current_env = self.current_env;
        self.current_env = Env::new_ptr_with_parent(self.current_env);

        for (key, param) in params.into_iter() {
            self.assign_variable(key.as_str(), param)?;
        }
        for stmt in stmts {
            value = self.execute_stmt(stmt)?;
            if value != Value::Nil {
                break;
            }
        }

        // Drop the env of the current block
        let boxed: Box<Env> = Box::into(unsafe { Box::from_raw(self.current_env.as_ptr()) });
        drop(boxed);

        self.current_env = current_env;
        Ok(value)
    }

    fn execute_expr(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Call(callee, _token, params) => {
                let func = self.execute_expr(callee)?;
                let mut values = vec![];
                for param in params {
                    let value = self.execute_expr(param)?;
                    values.push(value);
                }
                match func {
                    Value::Function(_name, params, block) => {
                        let mut params_map = HashMap::new();
                        for (i, value) in values.into_iter().enumerate() {
                            params_map.insert(params[i].clone(), value);
                        }
                        let value = self.execute_block(&block, params_map)?;
                        // println!("return value: {}", value);
                        Ok(value)
                    }
                    _ => Err(Error::InterceptError(format!("{} is not Callable", func))),
                }
            }
            Expr::Unary(operator, expr) => {
                let value = self.execute_expr(expr)?;
                match operator.typ {
                    TokenType::Minus => match value {
                        Value::Int(val) => Ok(Value::Int(-val)),
                        _ => Err(Error::InterceptError(format!(
                            "Unexpected unary operator {:?}",
                            operator
                        )))?,
                    },
                    TokenType::Bang => Ok(Value::Bool(!value.is_truthy())),
                    _ => Err(Error::InterceptError(format!(
                        "Unexpected unary operator {:?}",
                        operator
                    )))?,
                }
            }
            Expr::Variable(token) => {
                let value = self.lookup_variable(token.raw.as_str())?;
                Ok(value.clone())
            }
            Expr::Assign(token, expr) => {
                let _ = self.lookup_variable(token.raw.as_str())?;
                let value = self.execute_expr(expr)?;
                self.assign_variable(token.raw.as_str(), value)?;

                Ok(Value::Nil)
            }
            Expr::Binary(left, token, right) => {
                let left_val = self.execute_expr(left)?;
                let right_val = self.execute_expr(right)?;
                match token.typ {
                    TokenType::Minus => return Ok(left_val - right_val),
                    TokenType::Plus => return Ok(left_val + right_val),
                    TokenType::Star => return Ok(left_val * right_val),
                    TokenType::Slash => return Ok(left_val / right_val),
                    TokenType::BangEqual => return Ok(Value::Bool(left_val != right_val)),
                    TokenType::EqualEqual => return Ok(Value::Bool(left_val == right_val)),
                    TokenType::Greater => return Ok(Value::Bool(left_val > right_val)),
                    TokenType::GreaterEqual => return Ok(Value::Bool(left_val >= right_val)),
                    TokenType::Less => return Ok(Value::Bool(left_val < right_val)),
                    TokenType::LessEqual => return Ok(Value::Bool(left_val <= right_val)),
                    _ => {
                        return Err(Error::InterceptError(format!(
                            "Unexpected binary operator {:?}",
                            token
                        )))?
                    }
                }
            }
            Expr::Literal(val) => Ok(val.clone()),
            Expr::None => Ok(Value::Nil),
        }
    }

    fn lookup_variable(&self, name: &str) -> Result<&Value, Error> {
        let env = unsafe { self.current_env.as_ref() };
        env.get(name)
            .ok_or_else(|| Error::InterceptError(format!("Undefined variable {}", name)))
    }

    fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), Error> {
        let env = unsafe { self.current_env.as_mut() };
        env.define(name, value);
        Ok(())
    }
}

impl Drop for Intercepter {
    fn drop(&mut self) {
        let boxed: Box<Env> = Box::into(unsafe { Box::from_raw(self.current_env.as_ptr()) });
        drop(boxed);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner};

    use super::*;

    #[test]
    fn env_basic_operations() {
        let env = Env::new_ptr();
        let env = unsafe { &mut (*env.as_ptr()) };
        env.define("a", Value::Int(1));
        env.define("b", Value::Int(2));
        env.define("c", Value::Int(3));
        assert_eq!(env.get("a").unwrap(), &Value::Int(1));
        assert_eq!(env.get("b").unwrap(), &Value::Int(2));
        assert_eq!(env.get("c").unwrap(), &Value::Int(3));
        assert_eq!(env.get("d"), None);
        env.define("a", Value::Int(4));
        assert_eq!(env.get("a").unwrap(), &Value::Int(4));
    }

    #[test]
    fn env_with_parent() {
        let parent = Env::new_ptr();
        let parent_ref = unsafe { &mut (*parent.as_ptr()) };
        parent_ref.define("a", Value::Int(1));
        parent_ref.define("b", Value::Int(2));
        parent_ref.define("c", Value::Int(3));

        let env_raw = Env::new_ptr_with_parent(parent);
        let env = unsafe { &mut (*env_raw.as_ptr()) };
        env.define("d", Value::Int(4));
        env.define("e", Value::Int(5));
        env.define("f", Value::Int(6));

        assert_eq!(env.get("a").unwrap(), &Value::Int(1));
        assert_eq!(env.get("b").unwrap(), &Value::Int(2));
        assert_eq!(env.get("c").unwrap(), &Value::Int(3));
        assert_eq!(env.get("d").unwrap(), &Value::Int(4));
        assert_eq!(env.get("e").unwrap(), &Value::Int(5));
        assert_eq!(env.get("f").unwrap(), &Value::Int(6));
        assert_eq!(env.get("g"), None);

        let env_raw = Env::new_ptr_with_parent(env_raw);
        let env = unsafe { &mut (*env_raw.as_ptr()) };
        env.define("g", Value::Int(7));
        assert_eq!(env.get("g").unwrap(), &Value::Int(7));
        assert_eq!(env.get("a").unwrap(), &Value::Int(1));
        assert_eq!(env.get("b").unwrap(), &Value::Int(2));
        assert_eq!(env.get("c").unwrap(), &Value::Int(3));
        assert_eq!(env.parent().unwrap().get("a").unwrap(), &Value::Int(1));
        assert_eq!(
            env.parent().unwrap().parent().unwrap().get("b").unwrap(),
            &Value::Int(2)
        );
    }

    #[test]
    fn intercepter_print_env() {
        let script = r#"
        print(VERSION);
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn intercepter_print_variable() {
        let script = r#"
        local a = 1 + 2 * 3; 
        print(a);
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn intercepter_return_variable() {
        let script = r#"
        local a = 1 + 2 * 3; 
        return a;
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Int(7));
    }

    #[test]
    fn intercepter_function_define() {
        let script = r#"
        function fib(n)
          if n < 2 then
            return n;
          end

          return n + 3;
        end

        print(fib(4));
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn intercepter_function_recursion() {
        let script = r#"
        function fib(n)
          if n < 2 then
            return n;
          end

          return fib(n - 1);
        end

        print(fib(4));
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn intercepter_function_recursion2() {
        let script = r#"
        function fib(n)
            if n < 3 then
                return n;
            end
     
            local n1 = fib(n-1);
            local n2 = fib(n-2);
            return n1 + n2;
        end
        
        print(fib(5));
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Nil);
    }

    #[test]
    fn intercepter_many_functions() {
        let script = r#"
        function add1(n)
            return n + 1;
        end

        function add2(n)
            return n + 2;
        end
        
        local n1 = add1(3) + add2(4);
        local n2 = 2;
        local n = n1 + n2;
        
        return n;
        "#;
        let mut scanner = Scanner::new(script.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse().unwrap();
        // println!("{:?}", statements);

        let mut intercepter = Intercepter::new();
        let result = intercepter.eval(&statements);
        assert_eq!(result.unwrap(), Value::Int(12));
    }
}
