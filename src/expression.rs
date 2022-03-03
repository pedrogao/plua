// Expr 表达式 trait
pub trait Expr {
    fn visit_expr(&self) -> String;
}