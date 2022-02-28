use crate::compile::{Program};
use crate::bytecode::OpCode;

// 基于虚拟机(栈机)执行字节码
pub fn eval(prog: Program) -> i32 {
    // ip 寄存器
    let mut ip: i32 = 0;
    // The frame pointer points to the location on the data stack where each function can start storing its locals.
    let mut fp: i32 = 0;
    let mut stack: Vec<i32> = vec![]; // 栈

    println!("{:#?}", prog.instructions);
    println!("-----------------------------------");

    while ip < prog.instructions.len() as i32 {
        let op_code = &prog.instructions[ip as usize];
        println!("ip: {}, fp: {}, op_code: {:?}", ip, fp, op_code);
        match op_code {
            OpCode::DupPlusFP(i) => {
                stack.push(stack[(fp + i) as usize]); // 复制 fp
                ip += 1;
            }
            OpCode::GetParameter(local_offset, fp_offset) => {
                // 将函数参数拷贝到当前的frame
                stack[fp as usize + local_offset] = stack[(fp - (fp_offset + 4)) as usize];
                ip += 1;
            }
            OpCode::MovePlusFP(i) => {
                let val = stack.pop().unwrap();
                let index = fp as usize + *i; // 局部变量的序号
                // Accounts for top-level locals
                while index >= stack.len() {
                    stack.push(0);
                }
                stack[index] = val;
                ip += 1;
            }
            OpCode::JumpIfNotZero(label) => {
                let top = stack.pop().unwrap(); // compare 比较入栈后，对其进行判断
                if top == 0 {
                    ip = prog.syms[label].location;
                }
                ip += 1;
            }
            OpCode::Jump(label) => {
                ip = prog.syms[label].location;
            }
            OpCode::Return => {
                let ret = stack.pop().unwrap(); // 返回值先出栈

                // Clean up the local stack 函数返回需清理局部产生的数据
                // 通过 fp 来清理局部变量
                while fp < stack.len() as i32 {
                    stack.pop();
                }

                // Restore pc and fp
                // 通过 narguments 来清理参数
                let mut narguments = stack.pop().unwrap();
                ip = stack.pop().unwrap();
                fp = stack.pop().unwrap();

                // Clean up arguments 清理参数
                while narguments > 0 {
                    stack.pop();
                    narguments -= 1;
                }

                // Add back return value 然后再入栈
                stack.push(ret);
            }
            OpCode::Call(label, narguments) => {
                // handle native function
                if label == "print" {
                    for _ in 0..*narguments {
                        print!("{}", stack.pop().unwrap());
                        print!(" ");
                    }
                    println!();
                    ip += 1;
                    continue;
                }

                stack.push(fp);     // 保存当前fp值
                stack.push(ip + 1); // 记住下一个指令的地址
                stack.push(prog.syms[label].narguments as i32); // 函数参数个数，用来清理参数
                ip = prog.syms[label].location;
                fp = stack.len() as i32; // 局部变量在fp以上，所以 fp 就是函数局部变量与其它变量的分割线

                // Set up space for all arguments/locals
                let mut nlocals = prog.syms[label].nlocals; // 预分配局部变量
                while nlocals > 0 {
                    stack.push(0);
                    nlocals -= 1;
                }
            }
            OpCode::Add => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
                ip += 1;
            }
            OpCode::Subtract => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
                ip += 1;
            }
            OpCode::LessThan => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(if left < right { 1 } else { 0 });
                ip += 1;
            }
            OpCode::Store(n) => {
                stack.push(*n);
                ip += 1;
            }
        }
    }

    return stack.len() as i32;
}
