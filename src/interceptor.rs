use std::collections::HashMap;

use crate::statement::Stmt;
use crate::value::Value;

#[derive(Default)]
pub struct Interceptor {
    scopes: Vec<HashMap<String, ()>>,
}

impl Interceptor {
    pub fn eval(&mut self, statements: &Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::PrintStmt(_) => {}
            Stmt::IfStmt(_, _, _) => {}
            Stmt::LocalStmt(_, _) => {}
            Stmt::FunctionStmt(_, _, _) => {}
            Stmt::ReturnStmt(_, _) => {}
            Stmt::Expression(_) => {}
            Stmt::Block(_) => {}
            Stmt::None => {}
        }

        Ok(())
    }
}