use std::collections::HashMap;

use crate::bytecode::ByteCode;
use crate::emitter::Function;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct VM {
    globals: HashMap<String, Value>,
    frames: Vec<Frame>,
    stack: Vec<Value>,
    current: usize,
}

#[derive(Debug)]
pub struct Frame {
    sp: usize,
    ip: usize,
    current: usize,
}

impl Frame {
    pub fn new(sp: usize, ip: usize, current: usize) -> Self {
        Self { sp, ip, current }
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            frames: Vec::new(),
            stack: Vec::new(),
            current: 0,
        }
    }

    pub fn interpret(&mut self, funcs: &Vec<Function>) -> Value {
        let func = funcs.get(self.current);
        let mut ip = 0;

        if let Some(func) = func {
            let mut chunk = func.chunk();
            let mut arg_count = func.arity;
            let mut code = &chunk.codes;
            let mut constant = &chunk.constants;
            let mut ret = Value::Nil;

            while let Some(op) = code.get(ip) {
                ip += 1;
                match op {
                    ByteCode::Push(d) => self.stack.push(d.clone()),
                    ByteCode::Pop => {
                        self.stack.pop();
                    }
                    ByteCode::Add => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        self.stack.push(a.unwrap() + b.unwrap())
                    }
                    ByteCode::Sub => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        self.stack.push(b.unwrap() - a.unwrap())
                    }
                    ByteCode::Mul => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        self.stack.push(a.unwrap() * b.unwrap())
                    }
                    ByteCode::Div => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        self.stack.push(b.unwrap() / a.unwrap())
                    }
                    ByteCode::Incr => {
                        *self.stack.last_mut().unwrap() += Value::Int(1);
                    }
                    ByteCode::Decr => {
                        *self.stack.last_mut().unwrap() -= Value::Int(1);
                    }
                    ByteCode::Greater => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        let ok = b.unwrap() > a.unwrap();
                        self.stack.push(Value::Bool(ok));
                    }
                    ByteCode::Less => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        let ok = b.unwrap() < a.unwrap();
                        self.stack.push(Value::Bool(ok));
                    }
                    ByteCode::EqualEqual => {
                        let (a, b) = (self.stack.pop(), self.stack.pop());
                        let b = a.unwrap() == b.unwrap();
                        self.stack.push(Value::Bool(b));
                    }
                    ByteCode::Jump(p) => ip += *p,
                    ByteCode::GetLocal(i) => {
                        // locals from stack
                        println!("stack: {:?}, arg_count: {}, i: {}", self.stack, arg_count, i);
                        let value = self.stack.get(self.stack.len() - arg_count + *i).unwrap();
                        self.stack.push(value.clone());
                    }
                    ByteCode::SetLocal(_i) => todo!(),
                    ByteCode::Print => {
                        let val = self.stack.pop().unwrap();
                        print!("{}", val);
                    }
                    ByteCode::Call(n) => {
                        // Save current frame,
                        let current_frame = Frame::new(self.stack.len(), ip, self.current);
                        self.frames.push(current_frame);

                        // Create new frame
                        println!("call stack: {:?}", self.stack);
                        let mut args = vec![];
                        for _ in 0..*n {
                            let arg = self.stack.pop().unwrap();
                            args.push(arg);
                        }
                        let func = self.stack.pop().unwrap();
                        let closure = func.as_closure().unwrap();
                        let func_name = constant.get(*closure.0).unwrap();
                        let next_func_index = funcs
                            .iter()
                            .position(|f| f.name.as_str() == func_name.as_string().unwrap())
                            .unwrap();
                        self.current = next_func_index;
                        arg_count = *n;
                        chunk = funcs.get(next_func_index).unwrap().chunk();
                        code = &chunk.codes;
                        constant = &chunk.constants;
                        for ele in args {
                            self.stack.push(ele);
                        }
                        ip = 0;
                    }
                    ByteCode::Ret => {
                        let val = self.stack.pop().unwrap();
                        let frame = self.frames.pop();
                        if let Some(frame) = frame {
                            ip = frame.ip;
                            let current = frame.current;
                            chunk = funcs.get(current).unwrap().chunk();
                            code = &chunk.codes;
                            constant = &chunk.constants;

                            self.stack.truncate(frame.sp);
                            self.stack.push(val.clone());
                        } else {
                            ret = val;
                        }
                    }
                    ByteCode::JumpIfFalse(p) => ip += *p,
                    ByteCode::Closure(i) => {
                        let value = constant.get(*i).unwrap();
                        self.stack.push(value.clone());
                    }
                    ByteCode::Equal => todo!(),
                    ByteCode::DefineGlabal(i) => {
                        let val = self.stack.pop().unwrap();
                        let name = constant.get(*i).unwrap();
                        self.globals.insert(name.as_string().unwrap().clone(), val);
                    }
                    ByteCode::GetGlobal(i) => {
                        let name = constant.get(*i).unwrap();
                        let val = self.globals.get(name.as_string().unwrap()).unwrap();
                        self.stack.push(val.clone());
                    }
                    ByteCode::SetGlobal(_) => todo!(),
                    ByteCode::Constant(i) => {
                        let val = constant.get(*i).unwrap();
                        self.stack.push(val.clone());
                    }
                    ByteCode::Nil => {
                        self.stack.push(Value::Nil);
                    }
                }
            }

            ret
        } else {
            Value::Nil
        }
    }

    fn current_frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::ByteCode;
    use crate::debug::debug_all;
    use crate::emitter::{Emitter, Function};
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::value::Value;

    use super::VM;

    #[test]
    fn test_variable_declare() {
        let mut func = Function::new("main".to_string());
        let mut chunk = func.chunk_mut();
        let mut index = chunk.add_constant(Value::Int(1));
        chunk.add_bytecode(ByteCode::Constant(index));
        index = chunk.add_constant(Value::Int(2));
        chunk.add_bytecode(ByteCode::Constant(index));
        chunk.add_bytecode(ByteCode::Add);
        index = chunk.add_constant(Value::Int(34));
        chunk.add_bytecode(ByteCode::Constant(index));
        chunk.add_bytecode(ByteCode::Add);

        index = chunk.add_constant(Value::String("a".to_string()));
        chunk.add_bytecode(ByteCode::DefineGlabal(index));
        index = chunk.add_constant(Value::String("a".to_string()));
        chunk.add_bytecode(ByteCode::GetGlobal(index));

        chunk.add_bytecode(ByteCode::Print);
        chunk.add_bytecode(ByteCode::Nil);
        chunk.add_bytecode(ByteCode::Ret);

        let mut vm = VM::default();
        let funcs = vec![func];
        let ret = vm.interpret(&funcs);
        assert_eq!(ret, Value::Nil);
    }

    #[test]
    fn test_eval_variable_declare() {
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
        let chunk = emitter.emit_all(&result).unwrap();
        assert_eq!(chunk.len(), 1);
        debug_all(chunk);

        let mut vm = VM::default();
        let ret = vm.interpret(chunk);
        assert_eq!(ret, Value::Nil);
    }

    #[test]
    fn test_func_call() {
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
        let funcs = emitter.emit_all(&result).unwrap();
        assert_eq!(funcs.len(), 2);
        debug_all(funcs);

        let mut vm = VM::new();
        let ret = vm.interpret(funcs);
        assert_eq!(ret, Value::Nil);
    }

    #[test]
    fn test_func_eval() {
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
        // println!("{:#?}", tokens.as_ref().unwrap());
        assert_eq!(tokens.as_ref().unwrap().len(), 49);

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse().unwrap();
        // println!("{:#?}", result);
        assert_eq!(result.len(), 2);

        let mut emitter = Emitter::default();
        let funcs = emitter.emit_all(&result).unwrap();
        assert_eq!(funcs.len(), 2);
        debug_all(funcs);

        let mut vm = VM::new();
        let ret = vm.interpret(funcs);
        assert_eq!(ret, Value::Nil);
    }

    #[test]
    fn test_emit_mul() {
        let source = r#"
        function mul(n)
          local n1 = n + 1 * 4 / 2;
          return n1;
        end

        print(mul(4));
        "#;

        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        // println!("{:#?}", tokens.as_ref().unwrap());
        assert_eq!(tokens.as_ref().unwrap().len(), 29);

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        // println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 2);

        let mut emitter = Emitter::default();
        let funcs = emitter.emit_all(result.as_ref().unwrap()).unwrap();
        assert_eq!(funcs.len(), 2);
        debug_all(funcs);

        let mut vm = VM::default();
        let ret = vm.interpret(funcs);
        assert_eq!(ret, Value::Nil);
    }
}
