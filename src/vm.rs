use crate::bytecode::ByteCode;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct VM {}

#[derive(Debug)]
pub struct Frame {
    stack_offset: usize,
    ip: usize,
}

impl Frame {
    pub fn new(stack_offset: usize, ip: usize) -> Self {
        Self {
            stack_offset,
            ip,
        }
    }
}

impl VM {
    pub fn eval(&self, chunk: &Vec<ByteCode>) -> i32 {
        let mut stack: Vec<Value> = Vec::new();
        let mut frames: Vec<Frame> = Vec::new();
        let mut ip = 0;

        while let Some(op) = chunk.get(ip) {
            ip += 1;
            match op {
                ByteCode::Noop => {}
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
                    stack.push(Value::Int(ok as i32));
                }
                ByteCode::Less => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let ok = b.unwrap() < a.unwrap();
                    stack.push(Value::Int(ok as i32));
                }
                ByteCode::EqualEqual => {
                    let (a, b) = (stack.pop(), stack.pop());
                    let b = a.unwrap() == b.unwrap();
                    stack.push(Value::Int(b as i32));
                }
                ByteCode::Jump(p) => ip = *p,
                ByteCode::JE(p) => {
                    if *stack.last().unwrap() == Value::Int(0) {
                        stack.pop();
                        ip = *p;
                    }
                }
                ByteCode::JNE(p) => {
                    if *stack.last().unwrap() != Value::Int(0) {
                        stack.pop();
                        ip = *p;
                    }
                }
                ByteCode::JGT(p) => {
                    if *stack.last().unwrap() > Value::Int(0) {
                        stack.pop();
                        ip = *p;
                    }
                }
                ByteCode::JLT(p) => {
                    if *stack.last().unwrap() < Value::Int(0) {
                        stack.pop();
                        ip = *p;
                    }
                }
                ByteCode::JGE(p) => {
                    if *stack.last().unwrap() >= Value::Int(0) {
                        stack.pop();
                        ip = *p;
                    }
                }
                ByteCode::JLE(p) => {
                    if *stack.last().unwrap() <= Value::Int(0) {
                        stack.pop();
                        ip = *p;
                    }
                }
                ByteCode::Get(i) => {
                    let idx = frames.last().map_or(0, |s| s.stack_offset);
                    stack.push(stack.get(i + idx).unwrap().clone());
                }
                ByteCode::Set(i) => {
                    *stack
                        .get_mut(i + frames.last().map_or(0, |s| s.stack_offset))
                        .unwrap() = stack.last().unwrap().clone();
                }
                ByteCode::GetArg(i) => stack.push(
                    *stack
                        .get(frames.last().unwrap().stack_offset - 1 - i)
                        .unwrap(),
                ),
                ByteCode::SetArg(i) => {
                    let offset_i = frames.last().unwrap().stack_offset - 1 - i;
                    let new_val = stack.last().unwrap();
                    *stack.get_mut(offset_i).unwrap() = new_val.clone();
                }
                ByteCode::Print => {
                    print!("{}", stack.last().unwrap());
                }
                ByteCode::Call(p) => {
                    frames.push(Frame {
                        stack_offset: stack.len(),
                        ip: ip - 1,
                    });
                    ip = *p;
                }
                ByteCode::Ret => ip = frames.pop().unwrap().ip,
            }
        }

        return 0;
    }
}

mod tests {
    use crate::bytecode::ByteCode;
    use crate::debug::debug;
    use crate::emitter::Emitter;
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::value::Value;

    use super::VM;

    #[test]
    fn test_func_call() {
        let mut chunk = Vec::new();
        chunk.push(ByteCode::Push(Value::Int(3)));
        chunk.push(ByteCode::Push(Value::Int(2)));
        chunk.push(ByteCode::Push(Value::Int(1)));

        chunk.push(ByteCode::Jump(7));
        let add_mul_ip = chunk.len();
        chunk.push(ByteCode::Add);
        chunk.push(ByteCode::Mul);
        chunk.push(ByteCode::Ret);

        chunk.push(ByteCode::Jump(11));
        let square_ip = chunk.len();
        chunk.push(ByteCode::GetArg(0));
        chunk.push(ByteCode::Mul);
        chunk.push(ByteCode::Ret);

        chunk.push(ByteCode::Call(add_mul_ip));
        chunk.push(ByteCode::Call(square_ip));
        chunk.push(ByteCode::Print);

        let vm = VM::default();
        vm.eval(chunk.as_ref());
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
        let r = emitter.emit(result.as_ref().unwrap());
        assert_eq!(r.is_err(), false);
        assert_eq!(r.as_ref().unwrap().len(), 23);
        println!("{:#?}", r.as_ref().unwrap());
        debug(r.as_ref().unwrap());

        let chunk = r.unwrap();
        let vm = VM::default();
        vm.eval(chunk.as_ref());
    }
}
