use crate::bytecode::ByteCode;
use crate::compile::{Program};
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
    pub fn eval(&self, prog: Program) -> i32 {
        let mut stack: Vec<Value> = Vec::new();
        let mut frames: Vec<Frame> = Vec::new();
        let mut ip = 0;

        while let Some(op) = prog.chunk.get(ip) {
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
                        ip,
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
    use crate::compile::{Program};
    use crate::debug::debug;
    use crate::value::Value;

    use super::VM;

    #[test]
    fn test_func_call() {
        let mut prog = Program::default();
        prog.write_byte(ByteCode::Push(Value::Int(3)));
        prog.write_byte(ByteCode::Push(Value::Int(2)));
        prog.write_byte(ByteCode::Push(Value::Int(1)));

        prog.write_byte(ByteCode::Jump(7));
        let add_mul_ip = prog.chunk.len();
        prog.write_byte(ByteCode::Add);
        prog.write_byte(ByteCode::Mul);
        prog.write_byte(ByteCode::Ret);

        prog.write_byte(ByteCode::Jump(11));
        let square_ip = prog.chunk.len();
        prog.write_byte(ByteCode::GetArg(0));
        prog.write_byte(ByteCode::Mul);
        prog.write_byte(ByteCode::Ret);

        prog.write_byte(ByteCode::Call(add_mul_ip));
        prog.write_byte(ByteCode::Call(square_ip));
        prog.write_byte(ByteCode::Print);

        debug(&prog);

        let vm = VM::default();
        vm.eval(prog);
    }
}
