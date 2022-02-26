// 字节码
#[derive(Debug)]
pub enum OpCode {
    // store local in stack, so fp+i
    DupPlusFP(i32),
    // It copies arguments from behind the frame pointer to in front of the frame pointer.
    MoveMinusFP(usize, i32),
    // copies a value from the stack (offset the frame pointer) onto the top of the stack.
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

// 符号
#[derive(Debug)]
pub struct Symbol {
    pub location: i32,
    pub narguments: usize,
    pub nlocals: usize,
}