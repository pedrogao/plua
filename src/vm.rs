use std::borrow::Borrow;
use std::collections::HashMap;
use crate::bytecode::ByteCode;

use crate::compile::{Program, Value};

#[derive(Debug, Default)]
pub struct VM {
    ip: i32,
    stack: Vec<Value>,
    frame: Vec<Frame>,
}

#[derive(Debug)]
pub struct Frame {
    return_address: i32,
    slots: HashMap<String, Value>,
}

impl Frame {
    pub fn new(ra: i32) -> Self {
        Self {
            return_address: ra,
            slots: HashMap::new(),
        }
    }
}

impl VM {
    pub fn eval(&mut self, prog: Program) -> i32 {
        let mut ip = 0; // ip
        let mut bp = 0; // 栈(帧)底
        let mut sp = 0; // 栈(帧)顶
        let mut stack: Vec<Value> = vec![]; // 栈

        while ip < prog.chunk.len() {
            let op = ByteCode::Return;
            match op {
                ByteCode::Constant(_) => {
                    // TODO
                    let constant = prog.read_byte(ip + 1);
                    print!("{}", prog.read_constant(constant as usize));
                    ip += 2;
                }
                ByteCode::Call(_)  => {
                    // TODO 注意：在函数调用前，就必须已经把函数参数 push 入栈
                    let index = prog.read_byte(ip + 1);
                    let func_name = prog.read_label(index as usize); // 得到函数名
                    let sym = prog.syms[func_name].borrow();
                    let args_count = sym.narguments;
                    let mut locals_count = sym.nlocals;
                    // 1. 存储 ip, bp
                    stack.push(Value::Int((ip + 1) as i32));
                    stack.push(Value::Int(bp as i32));
                    bp = stack.len();
                    // 2. 参数，在call之前已经push到了栈中，所以将参数个数推入栈中
                    stack.push(Value::Int(args_count as i32));
                    // 3. 局部变量预分配
                    while locals_count > 0 {
                        stack.push(Value::Nil); // nil
                        locals_count -= 1;
                    }
                    // 4. 设置 bp, sp的值
                    sp = stack.len();
                    ip += 2;
                }
                ByteCode::Jump(_)  => {
                    let index = prog.read_byte(ip + 1);
                    ip += 2;
                }
                ByteCode::Add => {}
                ByteCode::Sub => {}
                ByteCode::Equal => {}
                ByteCode::Greater => {}
                ByteCode::Less => {}
                ByteCode::Print => {}
                ByteCode::Pop => {}
                ByteCode::GetParameter(_)  => {}
                ByteCode::GetLocal(_)  => {}
                ByteCode::SetLocal(_)  => {}
                ByteCode::JumpIfFalse => {}
                ByteCode::Return => {}
                ByteCode::Nop => {
                    ip += 1;
                }
            }
        }

        return 0;
    }
}

mod tests {
    use crate::debug::debug;
    use crate::compile::{Program, Value};
    use crate::bytecode::ByteCode;

    #[test]
    fn test_func_declare() {
        let mut prog = Program::default();
        // function gen declare
        // 1. 跳转到函数末尾，目前无需调用
        let func_end = prog.write_label("func_end".to_string());
        prog.write_byte(ByteCode::Jump.into());
        prog.write_byte(func_end as u8);

        // 2. 处理函数参数, 根据bp来访问param
        prog.write_byte(ByteCode::GetParameter.into());
        prog.write_byte(0); // 第一个参数

        // 3. 处理函数语句
        let constant_index = prog.write_constant(Value::Int(2));
        prog.write_byte(ByteCode::Constant.into());
        prog.write_byte(constant_index as u8);
        // 比较
        prog.write_byte(ByteCode::Less.into());
        prog.write_byte(ByteCode::JumpIfFalse.into());
        // 不满足则跳到return后面
        prog.write_byte((prog.chunk.len() + 1) as u8); // 跳过 return
        // 满足比较，即返回，记得将结果入栈
        prog.write_byte(ByteCode::Return.into());

        // 局部变量 TODO 本质是语句赋值
        let n1_index = prog.write_label("n1".to_string());
        prog.write_byte(ByteCode::SetLocal.into());
        prog.write_byte(n1_index as u8);
        // 将 n 和 1 入栈
        prog.write_byte(ByteCode::Add.into());

        let n2_index = prog.write_label("n2".to_string());
        prog.write_byte(ByteCode::SetLocal.into());
        prog.write_byte(n2_index as u8);

        prog.write_byte(ByteCode::Add.into());

        prog.write_byte(ByteCode::Return.into());

        debug(&prog);
    }
}