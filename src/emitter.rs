use crate::bytecode::ByteCode;
use crate::error::Error;
use crate::expression::Expr;
use crate::scanner::{Token, TokenType};
use crate::statement::Stmt;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub codes: Vec<ByteCode>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            codes: vec![],
            constants: vec![],
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn add_bytecode(&mut self, bytecode: ByteCode) {
        self.codes.push(bytecode);
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: usize, // arguments count
    pub value_count: usize,
    chunk: Chunk,
}

impl Function {
    pub fn new(name: String) -> Self {
        Self {
            name,
            arity: 0,
            value_count: 0,
            chunk: Chunk::new(),
        }
    }

    pub fn set_arity(&mut self, arity: usize) {
        self.arity = arity;
    }

    pub fn incr_value_count(&mut self) {
        self.value_count += 1;
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
}

impl Default for Function {
    fn default() -> Self {
        Self::new("<script>".to_string())
    }
}

pub struct Emitter {
    functions: Vec<Function>,
    current: usize,
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Emitter {
    pub fn new() -> Self {
        let script = Function::default();
        Self {
            functions: vec![script],
            current: 0,
        }
    }

    pub fn emit(&mut self, statements: &Vec<Stmt>) -> Result<&Chunk, Error> {
        self.emit_stmts(statements)?;
        Ok(&self.current().chunk)
    }

    pub fn emit_all(&mut self, statements: &Vec<Stmt>) -> Result<&Vec<Function>, Error> {
        self.emit_stmts(statements)?;
        Ok(&self.functions)
    }

    fn emit_stmts(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in statements {
            self.emit_stmt(stmt)?;
        }

        self.emit_bytecode(ByteCode::Nil);
        self.emit_bytecode(ByteCode::Ret);

        Ok(())
    }

    fn emit_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::PrintStmt(expr) => {
                self.emit_expr(expr)?;
                self.emit_bytecode(ByteCode::Print);
                Ok(())
            }
            Stmt::IfStmt(condition, then_branch, else_branch) => {
                self.emit_if_stmt(condition, then_branch.as_ref(), else_branch.as_ref())
            }
            Stmt::LocalStmt(name, init) => self.emit_local_stmt(name, init),
            Stmt::FunctionStmt(name, params, body) => self.emit_func_stmt(name, params, body),
            Stmt::ReturnStmt(keyword, value) => self.emit_return_stmt(keyword, value),
            Stmt::Expression(expr) => self.emit_expr(expr),
            Stmt::Block(stmts) => self.emit_block(stmts),
            Stmt::None => Ok(()),
        }
    }

    fn emit_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            self.emit_stmt(stmt)?;
        }
        Ok(())
    }

    fn emit_return_stmt(&mut self, _keyword: &Token, value: &Expr) -> Result<(), Error> {
        self.emit_expr(value)?;
        // self.bytecodes.push(ByteCode::Ret);
        self.emit_bytecode(ByteCode::Ret);
        Ok(())
    }

    fn emit_func_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), Error> {
        self.begin_scope(name.raw.as_str());
        self.current().set_arity(params.len());

        for stmt in body {
            self.emit_stmt(stmt)?;
        }

        self.end_scope();

        let func_name = name.raw.as_str();
        let idx = self.add_constant(Value::String(func_name.to_string()));
        let mut indexes = vec![];
        for param in params {
            indexes.push(self.add_constant(Value::String(param.raw.clone())));
        }
        let idx = self.add_constant(Value::Closure(idx, indexes));
        self.emit_bytecode(ByteCode::Closure(idx));

        let idx = self.add_constant(Value::String(func_name.to_string()));
        self.emit_bytecode(ByteCode::DefineGlabal(idx));

        Ok(())
    }

    fn emit_local_stmt(&mut self, name: &Token, init: &Expr) -> Result<(), Error> {
        self.emit_expr(init)?;

        let name = name.raw.as_str();
        let index = self.add_constant(Value::String(name.to_string()));
        self.emit_bytecode(ByteCode::DefineGlabal(index));
        Ok(())
    }

    fn emit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Stmt,
    ) -> Result<(), Error> {
        self.emit_expr(condition)?;
        // let then_jmp = self.bytecodes.len();
        // self.bytecodes.push(ByteCode::JE(0));
        // self.bytecodes.push(ByteCode::Pop);

        self.emit_stmt(then_branch)?;
        // let else_jmp = self.bytecodes.len();
        // self.bytecodes.push(ByteCode::Jump(0));

        // self.bytecodes[then_jmp] = ByteCode::JE(self.bytecodes.len());
        // self.bytecodes.push(ByteCode::Pop);

        self.emit_stmt(else_branch)?;
        // self.bytecodes[else_jmp] = ByteCode::Jump(self.bytecodes.len());
        Ok(())
    }

    fn emit_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Call(callee, paren, args) => self.emit_call(callee.as_ref(), paren, args)?,
            Expr::Unary(operator, right) => self.emit_unary(operator, right.as_ref())?,
            Expr::Variable(name) => self.emit_variable(name)?,
            Expr::Assign(name, value) => self.emit_assign(name, value)?,
            Expr::Binary(left, operator, right) => {
                self.emit_binary(left.as_ref(), operator, right.as_ref())?
            }
            Expr::Literal(value) => self.emit_literal(value)?,
            Expr::None => (),
        }

        Ok(())
    }

    fn emit_literal(&mut self, val: &Value) -> Result<(), Error> {
        // self.bytecodes.push(ByteCode::Push(val.clone()));
        let index = self.add_constant(val.clone());
        self.emit_bytecode(ByteCode::Constant(index));
        Ok(())
    }

    fn emit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<(), Error> {
        self.emit_expr(left)?;
        self.emit_expr(right)?;
        // left op right
        match operator.typ {
            TokenType::Equal => self.emit_bytecode(ByteCode::Equal),
            TokenType::Greater => self.emit_bytecode(ByteCode::Greater),
            TokenType::Less => self.emit_bytecode(ByteCode::Less),
            TokenType::Plus => self.emit_bytecode(ByteCode::Add),
            TokenType::Minus => self.emit_bytecode(ByteCode::Sub),
            TokenType::Star => self.emit_bytecode(ByteCode::Mul),
            TokenType::Slash => self.emit_bytecode(ByteCode::Div),
            _ => {
                return Err(Error::EmitError(format!(
                    "{:?} operator not support",
                    operator.typ
                )));
            }
        }
        Ok(())
    }

    fn emit_assign(&mut self, _name: &Token, _value: &Expr) -> Result<(), Error> {
        Ok(())
    }

    fn emit_variable(&mut self, name: &Token) -> Result<(), Error> {
        let index = self.add_constant(Value::String(name.raw.clone()));
        if self.current > 0 {
            self.emit_bytecode(ByteCode::GetLocal(index));
        } else {
            self.emit_bytecode(ByteCode::GetGlobal(index));
        }
        self.current().incr_value_count();
        Ok(())
    }

    fn emit_unary(&mut self, _operator: &Token, _right: &Expr) -> Result<(), Error> {
        Ok(())
    }

    fn emit_call(&mut self, callee: &Expr, _paren: &Token, args: &Vec<Expr>) -> Result<(), Error> {
        // println!("emit call, callee:{:?}, args: {:?}", callee, args);

        self.emit_expr(callee)?;
        for expr in args {
            self.emit_expr(expr)?;
        }

        self.emit_bytecode(ByteCode::Call(args.len()));

        Ok(())
    }

    fn current(&mut self) -> &mut Function {
        let current = self.current;
        self.functions.get_mut(current).unwrap()
    }

    fn begin_scope(&mut self, name: &str) {
        self.functions.push(Function::new(name.to_string()));
        self.current += 1;
    }

    fn end_scope(&mut self) {
        self.current -= 1;
    }

    fn emit_bytecode(&mut self, code: ByteCode) {
        self.current().chunk_mut().add_bytecode(code);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.current().chunk_mut().add_constant(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::debug::{debug, debug_all};
    use crate::emitter::Emitter;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn test_emit_local() {
        let source = r#"
        local a = 1 + 2 + 34;
        print(a);
        "#;
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 15);

        let mut parser = Parser::new(tokens.clone());
        let result = parser.parse().unwrap();
        assert_eq!(result.len(), 2);

        let mut emitter = Emitter::default();
        let r = emitter.emit(&result).unwrap();
        assert_eq!(r.codes.len(), 10);
        debug(r);
    }

    #[test]
    fn test_simple_func() {
        let source = r#"
        function fib(n)
            return n + 3;
        end

        print(fib(4));
        "#;
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 20);

        let mut parser = Parser::new(tokens.clone());
        let result = parser.parse().unwrap();
        assert_eq!(result.len(), 2);

        let mut emitter = Emitter::default();
        let r = emitter.emit_all(&result).unwrap();
        assert_eq!(r.len(), 2);
        debug_all(r);
    }

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
        let tokens = scanner.scan_tokens().unwrap();
        // println!("{:#?}", tokens);
        assert_eq!(tokens.len(), 49);

        let mut parser = Parser::new(tokens.clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        // println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 2);

        let mut emitter = Emitter::default();
        let r = emitter.emit_all(result.as_ref().unwrap()).unwrap();
        assert_eq!(r.len(), 2);
        debug_all(r);
    }
}
