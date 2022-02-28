use crate::bytecode::ByteCode;
use crate::compile::Program;

// 输出字节码详细信息
pub fn debug(prog: &Program) {
    let mut offset = 0;
    while offset < prog.chunk.len() {
        print!("{:04} ", offset);
        let op = ByteCode::Return;
        match op {
            ByteCode::Constant(_) => {
                let constant = prog.read_byte(offset + 1);
                print!("{:16} {} '", "Constant", constant);
                print!("{}", prog.read_constant(constant as usize));
                print!("'\n");
                offset += 2;
            }
            ByteCode::Call(_) => {
                let index = prog.read_byte(offset + 1);
                print!("{:16} {} '", "Call", index);
                print!("{}", prog.read_label(index as usize));
                print!("'\n");
                offset += 2;
            }
            ByteCode::Jump(_) => {
                let index = prog.read_byte(offset + 1);
                print!("{:16} {} \n", "Jump", index);
                offset += 2;
            }
            _ => {}
        }
    }
}
