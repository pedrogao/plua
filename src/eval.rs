use crate::compile::Program;
use crate::opcode::OpCode;

// 基于虚拟机(栈机)执行字节码
pub fn eval(prg: Program) -> i32 {
    let mut ip: i32 = 0; // ip 寄存器
    let mut sp: i32 = 0; // stack pointer 寄存器
    let mut stack: Vec<i32> = vec![]; // 栈

    while ip < prg.instructions.len() as i32 {
        match &prg.instructions[ip as usize] {
            OpCode::DupPlusFP(i) => {
                stack.push(stack[(sp + i) as usize]);
                ip += 1;
            }
            OpCode::MoveMinusFP(local_offset, fp_offset) => {
                stack[sp as usize + local_offset] = stack[(sp - (fp_offset + 4)) as usize];
                ip += 1;
            }
            OpCode::MovePlusFP(i) => {
                let val = stack.pop().unwrap();
                let index = sp as usize + *i;
                // Accounts for top-level locals
                while index >= stack.len() {
                    stack.push(0);
                }
                stack[index] = val;
                ip += 1;
            }
            OpCode::JumpIfNotZero(label) => {
                let top = stack.pop().unwrap();
                if top == 0 {
                    ip = prg.syms[label].location;
                }
                ip += 1;
            }
            OpCode::Jump(label) => {
                ip = prg.syms[label].location;
            }
            OpCode::Return => {
                let ret = stack.pop().unwrap();

                // Clean up the local stack
                while sp < stack.len() as i32 {
                    stack.pop();
                }

                // Restore pc and fp
                let mut narguments = stack.pop().unwrap();
                ip = stack.pop().unwrap();
                sp = stack.pop().unwrap();

                // Clean up arguments
                while narguments > 0 {
                    stack.pop();
                    narguments -= 1;
                }

                // Add back return value
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

                stack.push(sp);
                stack.push(ip + 1); // 记住下一个指令的地址
                stack.push(prg.syms[label].narguments as i32); // 函数参数个数
                ip = prg.syms[label].location;
                sp = stack.len() as i32;

                // Set up space for all arguments/locals
                let mut nlocals = prg.syms[label].nlocals; // 预分配局部变量
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
