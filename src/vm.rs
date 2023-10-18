use std::collections::HashMap;

use crate::bytecode::ByteCode;
use crate::emitter::{Chunk, Function};
use crate::value::Value;

#[derive(Debug, Default)]
pub struct VM {
    globals: HashMap<String, Value>,
    frames: Vec<Frame>,
    current_frame: Option<usize>,
    funcs: Vec<Function>,
}

#[derive(Debug)]
pub struct Frame {
    sp: usize,
    ip: usize,
    locals: Vec<Value>,
}

impl Frame {
    pub fn new(sp: usize, ip: usize) -> Self {
        Self {
            sp,
            ip,
            locals: Vec::new(),
        }
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            frames: Vec::new(),
            current_frame: None,
            funcs: Vec::new(),
        }
    }

    pub fn new_with_funcs(funcs: Vec<Function>) -> Self {
        Self {
            globals: HashMap::new(),
            frames: Vec::new(),
            current_frame: None,
            funcs,
        }
    }

    pub fn eval_all(&mut self) -> Value {
        let chunk = self.funcs.first().map(|func| func.chunk()).cloned();
        if let Some(chunk) = chunk.as_ref() {
            self.eval(chunk)
        } else {
            Value::Nil
        }
    }

    pub fn eval(&mut self, chunk: &Chunk) -> Value {
        let mut stack: Vec<Value> = Vec::new();
        let mut ip = 0;
        let code = &chunk.codes;
        let constant = &chunk.constants;
        let mut ret = Value::Nil;

        while let Some(op) = code.get(ip) {
            ip += 1;
            match op {
                ByteCode::Push(d) => stack.push(d.clone()),
                ByteCode::Pop => {
                    stack.pop();
                }
                ByteCode::Add => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(a.unwrap() + b.unwrap())
                }
                ByteCode::Sub => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(b.unwrap() - a.unwrap())
                }
                ByteCode::Mul => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(a.unwrap() * b.unwrap())
                }
                ByteCode::Div => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(b.unwrap() / a.unwrap())
                }
                ByteCode::Incr => {
                    *stack.last_mut().unwrap() += Value::Int(1);
                }
                ByteCode::Decr => {
                    *stack.last_mut().unwrap() -= Value::Int(1);
                }
                ByteCode::Greater => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let ok = b.unwrap() > a.unwrap();
                    stack.push(Value::Bool(ok));
                }
                ByteCode::Less => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let ok = b.unwrap() < a.unwrap();
                    stack.push(Value::Bool(ok));
                }
                ByteCode::EqualEqual => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let b = a.unwrap() == b.unwrap();
                    stack.push(Value::Bool(b));
                }
                ByteCode::Jump(p) => ip = *p,
                ByteCode::GetLocal(i) => {
                    let name = constant.get(*i).unwrap();
                    // let val = self
                    //     .current_frame()
                    //     .locals
                    //     .get(name.as_string().unwrap())
                    //     .unwrap();
                    // stack.push(val.clone());
                }
                ByteCode::SetLocal(_i) => todo!(),
                ByteCode::Print => {
                    let val = stack.pop().unwrap();
                    print!("{}", val);
                }
                ByteCode::Call(arg_count) => {
                    // Save current frame,
                    let current_frame = Frame::new(stack.len(), ip - 1);
                    self.frames.push(current_frame);

                    // Create new frame
                    for _ in 0..*arg_count {
                        let arg = stack.pop().unwrap();
                    }
                    let func = stack.pop().unwrap();

                    let closure = func.as_closure().unwrap();
                    let func_name = constant.get(*closure.0).unwrap();
                    // let param_names = constant.get(*closure.1).unwrap();
                    let next_func = self
                        .funcs
                        .iter()
                        .find(|f| f.name.as_str() == func_name.as_string().unwrap())
                        .unwrap();
                    println!("next_func: {:?}", next_func);
                    // let ret = self.eval_func(next_func, *arg_count);
                    // stack.push(ret);
                    // let frame = Frame::new(stack.len() - *arg_count, 0);
                }
                ByteCode::Ret => {
                    let val = stack.pop().unwrap();
                    ret = val;
                }
                ByteCode::JumpIfFalse(_) => todo!(),
                ByteCode::Closure(i) => {
                    let value = constant.get(*i).unwrap();
                    stack.push(value.clone());
                }
                ByteCode::Equal => todo!(),
                ByteCode::DefineGlabal(i) => {
                    let val = stack.pop().unwrap();
                    let name = constant.get(*i).unwrap();
                    self.globals.insert(name.as_string().unwrap().clone(), val);
                }
                ByteCode::GetGlobal(i) => {
                    let name = constant.get(*i).unwrap();
                    let val = self.globals.get(name.as_string().unwrap()).unwrap();
                    stack.push(val.clone());
                }
                ByteCode::SetGlobal(_) => todo!(),
                ByteCode::Constant(i) => {
                    let val = constant.get(*i).unwrap();
                    stack.push(val.clone());
                }
                ByteCode::Nil => {
                    stack.push(Value::Nil);
                }
            }
        }

        ret
    }

    fn eval_func(&mut self, func: &Function, arg_count: usize) -> Value {
        // if func.arity != arg_count {
        // }
        let mut stack: Vec<Value> = Vec::new();
        let chunk = func.chunk();
        let mut ip = 0;
        let code = &chunk.codes;
        let constant = &chunk.constants;
        let mut ret = Value::Nil;

        while let Some(op) = code.get(ip) {
            ip += 1;
            match op {
                ByteCode::Push(d) => stack.push(d.clone()),
                ByteCode::Pop => {
                    stack.pop();
                }
                ByteCode::Add => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(a.unwrap() + b.unwrap())
                }
                ByteCode::Sub => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(b.unwrap() - a.unwrap())
                }
                ByteCode::Mul => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(a.unwrap() * b.unwrap())
                }
                ByteCode::Div => {
                    let (a, b) = (stack.pop(), stack.pop());
                    stack.push(b.unwrap() / a.unwrap())
                }
                ByteCode::Incr => {
                    *stack.last_mut().unwrap() += Value::Int(1);
                }
                ByteCode::Decr => {
                    *stack.last_mut().unwrap() -= Value::Int(1);
                }
                ByteCode::Greater => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let ok = b.unwrap() > a.unwrap();
                    stack.push(Value::Bool(ok));
                }
                ByteCode::Less => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let ok = b.unwrap() < a.unwrap();
                    stack.push(Value::Bool(ok));
                }
                ByteCode::EqualEqual => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let b = a.unwrap() == b.unwrap();
                    stack.push(Value::Bool(b));
                }
                ByteCode::Jump(p) => ip = *p,
                ByteCode::GetLocal(i) => {
                    let name = constant.get(*i).unwrap();
                    // let val = self
                    //     .current_frame()
                    //     .locals
                    //     .get(name.as_string().unwrap())
                    //     .unwrap();
                    // stack.push(val.clone());
                }
                ByteCode::SetLocal(_i) => todo!(),
                ByteCode::Print => {
                    let val = stack.pop().unwrap();
                    print!("{}", val);
                }
                ByteCode::Call(arg_count) => {
                    // Save current frame,
                    let current_frame = Frame::new(stack.len(), ip - 1);
                    self.frames.push(current_frame);

                    // Create new frame
                    let func = stack.pop().unwrap();
                    let closure = func.as_closure().unwrap();

                    let func_name = constant.get(*closure.0).unwrap();
                    // let param_names = constant.get(*closure.1).unwrap();
                    let next_func = self
                        .funcs
                        .iter()
                        .find(|f| f.name.as_str() == func_name.as_string().unwrap())
                        .unwrap();
                    // let ret = self.eval_func(next_func, *arg_count);
                    // stack.push(ret);
                    // let frame = Frame::new(stack.len() - *arg_count, 0);
                }
                ByteCode::Ret => {
                    let val = stack.pop().unwrap();
                    ret = val;
                }
                ByteCode::JumpIfFalse(_) => todo!(),
                ByteCode::Closure(i) => {
                    let value = constant.get(*i).unwrap();
                    stack.push(value.clone());
                }
                ByteCode::Equal => todo!(),
                ByteCode::DefineGlabal(i) => {
                    let val = stack.pop().unwrap();
                    let name = constant.get(*i).unwrap();
                    self.globals.insert(name.as_string().unwrap().clone(), val);
                }
                ByteCode::GetGlobal(i) => {
                    let name = constant.get(*i).unwrap();
                    let val = self.globals.get(name.as_string().unwrap()).unwrap();
                    stack.push(val.clone());
                }
                ByteCode::SetGlobal(_) => todo!(),
                ByteCode::Constant(i) => {
                    let val = constant.get(*i).unwrap();
                    stack.push(val.clone());
                }
                ByteCode::Nil => {
                    stack.push(Value::Nil);
                }
            }
        }

        ret
    }

    fn current_frame(&mut self) -> &mut Frame {
        let idx = self.current_frame.unwrap();
        self.frames.get_mut(idx).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::ByteCode;
    use crate::debug::{debug, debug_all};
    use crate::emitter::{Chunk, Emitter};
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::value::Value;

    use super::VM;

    #[test]
    fn test_variable_declare() {
        let mut chunk = Chunk::new();
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
        let ret = vm.eval(&chunk);
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
        let chunk = emitter.emit(&result).unwrap();
        assert_eq!(chunk.codes.len(), 10);
        debug(chunk);

        let mut vm = VM::default();
        let ret = vm.eval(&chunk);
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

        let mut vm = VM::new_with_funcs(funcs.clone());
        let ret = vm.eval_all();
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
        println!("{:#?}", tokens.as_ref().unwrap());
        assert_eq!(tokens.as_ref().unwrap().len(), 49);

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 2);

        let mut emitter = Emitter::default();
        let r = emitter.emit(result.as_ref().unwrap()).unwrap();
        assert_eq!(r.codes.len(), 23);
        debug(r);

        let mut vm = VM::default();
        vm.eval(r);
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
        println!("{:#?}", tokens.as_ref().unwrap());
        assert_eq!(tokens.as_ref().unwrap().len(), 29);

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 2);

        let mut emitter = Emitter::default();
        let chunk = emitter.emit(result.as_ref().unwrap()).unwrap();
        assert_eq!(chunk.codes.len(), 12);
        debug(chunk);

        let mut vm = VM::default();
        vm.eval(chunk);
    }
}
