#[derive(Debug, thiserror::Error)]
pub enum Error {
    // 词法分析错误
    #[error("Scan error: {0}")]
    ScanError(String),
    // 词法错误
    #[error("Lex error: {0}")]
    LexError(String),
    // 语法错误
    #[error("Parse error: {0}")]
    ParseError(String),
    // 语义错误
    #[error("Resolve error: {0}")]
    ResolveError(String),
    // 解释运行时错误
    #[error("Intercept error: {0}")]
    InterceptError(String),
    // 生成字节码错误
    #[error("Emit error: {0}")]
    EmitError(String),
    // 未知错误
    #[error("Unknown error")]
    UnknownError,
}
