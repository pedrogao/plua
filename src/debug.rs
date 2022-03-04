use crate::bytecode::ByteCode;
use crate::compile::Program;

// 输出字节码详细信息
pub fn debug(prog: &Program) {
    let mut offset = 0;
    while offset < prog.chunk.len() {
        print!("{:04} ", offset);
        match prog.chunk[offset] {
            ByteCode::Push(d) => {
                print!("{:16} '{}", "Push", d);
                print!("'\n");
            }
            ByteCode::Pop => {
                print!("{:16}", "Pop");
                print!("\n");
            }
            ByteCode::Add => {
                print!("{:16}", "Add");
                print!("\n");
            }
            ByteCode::Sub => {
                print!("{:16}", "Sub");
                print!("\n");
            }
            ByteCode::Incr => {
                print!("{:16}", "Incr");
                print!("\n");
            }
            ByteCode::Decr => {
                print!("{:16}", "Decr");
                print!("\n");
            }
            ByteCode::Mul => {
                print!("{:16}", "Mul");
                print!("\n");
            }
            ByteCode::Div => {
                print!("{:16}", "Div");
                print!("\n");
            }
            ByteCode::Greater => {
                print!("{:16}", "Greater");
                print!("\n");
            }
            ByteCode::Less => {
                print!("{:16}", "Less");
                print!("\n");
            }
            ByteCode::EqualEqual => {
                print!("{:16}", "Equal");
                print!("\n");
            }
            ByteCode::Jump(i) => {
                print!("{:16} '{:04}", "Jump", i);
                print!("'\n");
            }
            ByteCode::JE(i) => {
                print!("{:16} '{:04}", "JE", i);
                print!("'\n");
            }
            ByteCode::JNE(i) => {
                print!("{:16} '{:04}", "JNE", i);
                print!("'\n");
            }
            ByteCode::JGT(i) => {
                print!("{:16} '{:04}", "JGT", i);
                print!("'\n");
            }
            ByteCode::JLT(i) => {
                print!("{:16} '{:04}", "JLT", i);
                print!("'\n");
            }
            ByteCode::JGE(i) => {
                print!("{:16} '{:04}", "JGE", i);
                print!("'\n");
            }
            ByteCode::JLE(i) => {
                print!("{:16} '{:04}", "JLE", i);
                print!("'\n");
            }
            ByteCode::Get(i) => {
                print!("{:16} '{}", "Get", i);
                print!("'\n");
            }
            ByteCode::Set(i) => {
                print!("{:16} '{}", "Set", i);
                print!("'\n");
            }
            ByteCode::GetArg(i) => {
                print!("{:16} '{}", "GetArg", i);
                print!("'\n");
            }
            ByteCode::SetArg(i) => {
                print!("{:16} '{}", "SetArg", i);
                print!("'\n");
            }
            ByteCode::Noop => {
                print!("{:16}", "Noop");
                print!("\n");
            }
            ByteCode::Print => {
                print!("{:16}", "Print");
                print!("\n");
            }
            ByteCode::Call(i) => {
                print!("{:16} '{:04}", "Call", i);
                print!("'\n");
            }
            ByteCode::Ret => {
                print!("{:16}", "Ret");
                print!("\n");
            }
        }
        offset += 1;
    }
}
