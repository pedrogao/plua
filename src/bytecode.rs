// 字节码
#[derive(Debug)]
pub enum OpCode {
    // store local in stack
    // 局部变量push进栈
    DupPlusFP(i32),
    // It copies arguments from behind the frame pointer to in front of the frame pointer.
    // 将参数从之前的frame拷贝到当前的frame
    GetParameter(usize, i32),
    // copies a value from the stack (offset the frame pointer) onto the top of the stack.
    // 从栈中拷贝一个值，放在当前的栈顶
    MovePlusFP(usize),
    // 写内存
    Store(i32),
    // 返回
    Return,
    // if != 0, jump
    JumpIfNotZero(String),
    // jump
    Jump(String),
    // call, function name, args length
    Call(String, usize),
    // add
    Add,
    // sub
    Subtract,
    // <=
    LessThan,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ByteCode {
    // push constant [op, index]
    Constant(usize),
    // + [op]
    Add,
    // - [op]
    Sub,
    // = [op]
    Equal,
    // > [op]
    Greater,
    // < [op]
    Less,
    // native func print [op]
    Print,
    // pop stack [op]
    Pop,
    // get local variable [op, index]
    GetLocal(usize),
    // set local variable [op, index]
    SetLocal(usize),
    // get function param [op, index]
    GetParameter(usize),
    // jump [op, index]
    Jump(String),
    // jump if top of stack is zero(false) [op, index]
    JumpIfFalse,
    // call function [op, index]
    Call(String),
    // return function or script [op]
    Return,
    // nop, default bytecode [op]
    Nop,
}

// 符号
#[derive(Debug)]
pub struct Symbol {
    // 位置
    pub location: i32,
    // 参数个数
    pub narguments: usize,
    // 局部变量个数
    pub nlocals: usize,
}
