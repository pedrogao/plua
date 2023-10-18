use crate::value::Value;

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

#[derive(Debug, Clone)]
pub enum ByteCode {
    Push(Value),
    Pop,
    Add,
    Sub,
    Incr,
    Decr,
    Mul,
    Div,
    Equal,
    EqualEqual,
    Less,
    Greater,
    Jump(usize),
    JumpIfFalse(usize),
    
    //
    Closure(usize),
    Call(usize),
    DefineGlabal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    GetLocal(usize),
    SetLocal(usize),
    Constant(usize),
    Nil,
    Print,
    Ret,

    // TODO:
    // Negtive,
    // Bang,
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
