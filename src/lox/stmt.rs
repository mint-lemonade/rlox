use super::expr::Expr;

pub enum Stmt<'a> {
    Expression(Expr<'a>),
    Print(Expr<'a>)
}