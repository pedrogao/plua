#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BfIR {
    AddVal(u8), // +
    SubVal(u8), // -
    AddPtr(u8), // >
    SubPtr(u8), // <
    GetByte,    // ,
    PutByte,    // .
    Jz,         // [
    Jnz,        // ]
}
