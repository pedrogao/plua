use crate::{
    bytecode::ByteCode,
    emitter::{Chunk, Function},
};

// 输出字节码详细信息
pub fn debug(chunk: &Chunk) {
    let codes = &chunk.codes;
    let constants = &chunk.constants;
    let mut offset = 0;
    while offset < codes.len() {
        print!("{:04} ", offset);
        match &codes[offset] {
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
            ByteCode::GetLocal(i) => {
                print!("{:16} '{}", "GetLocal", i);
                print!("'\n");
            }
            ByteCode::SetLocal(i) => {
                print!("{:16} '{}", "SetLocal", i);
                print!("'\n");
            }
            ByteCode::Print => {
                print!("{:16}", "Print");
                print!("\n");
            }
            ByteCode::Call(c) => {
                print!("{:16} '{}", "Call", c);
                print!("'\n");
            }
            ByteCode::Ret => {
                print!("{:16}", "Ret");
                print!("\n");
            }
            ByteCode::Equal => {
                print!("{:16}", "Equal");
                print!("\n");
            }
            ByteCode::JumpIfFalse(i) => {
                print!("{:16} '{:04}", "JumpIfFalse", i);
                print!("'\n");
            }
            ByteCode::Closure(i) => {
                print!("{:16} {} '{}", "Closure", i, constants[*i]);
                print!("'\n");
            }
            ByteCode::DefineGlabal(i) => {
                print!("{:16} {} '{}", "DefineGlabal", i, constants[*i]);
                print!("'\n");
            }
            ByteCode::GetGlobal(i) => {
                print!("{:16} {} '{}", "GetGlobal", i, constants[*i]);
                print!("'\n");
            }
            ByteCode::SetGlobal(i) => {
                print!("{:16} {} '{}", "SetGlobal", i, constants[*i]);
                print!("'\n");
            }
            ByteCode::Constant(i) => {
                print!("{:16} {} '{}", "Constant", i, constants[*i]);
                print!("'\n");
            }
            ByteCode::Nil => {
                print!("{:16}", "Nil");
                print!("\n");
            }
        }
        offset += 1;
    }
}

pub fn debug_all(funcs: &Vec<Function>) {
    for func in funcs {
        print!(
            "== {} arity: {}  value_count: {} ==\n",
            func.name.as_str(),
            func.arity,
            func.value_count
        );
        debug(&func.chunk());
    }
}
