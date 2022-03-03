// Stmt 语句 trait
pub trait Stmt {
    fn visit_stmt(&self) -> String;
}