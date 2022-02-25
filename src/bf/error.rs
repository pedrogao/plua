use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum CompileErrorKind {
    #[error("Unclosed left bracket")]
    UnclosedLeftBracket,
    #[error("Unexpected right bracket")]
    UnexcpectedRightBracket,
}

#[derive(Debug)]
pub struct CompileError {
    pub line: u32,
    pub col: u32,
    pub kind: CompileErrorKind,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Pointer overflow")]
    PointerOverflow,
}

#[derive(Debug, thiserror::Error)]
pub enum VMError {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Compile: {0}")]
    Compile(#[from] CompileError),

    #[error("Runtime: {0}")]
    Runtime(#[from] RuntimeError),
}

pub type Result<T> = std::result::Result<T, VMError>;
