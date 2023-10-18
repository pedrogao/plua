use crate::value::Value;

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
    GetGlobal(usize), // get global variable from global table, `usize` is the index of constant pool, then read value from constant pool
    SetGlobal(usize),
    GetLocal(usize), // get local variable from local slots, `usize` is the index of local slots
    SetLocal(usize),
    Constant(usize),
    Nil,
    Print,
    Ret,
    // TODO:
    // Negtive,
    // Bang,
}
