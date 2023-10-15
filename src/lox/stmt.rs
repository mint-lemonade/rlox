use std::rc::Rc;

use super::{expr::Expr, token::Token};

pub enum Stmt<'a> {
    Expression(Expr<'a>),
    If(Expr<'a>, Box<Stmt<'a>>, Box<Option<Stmt<'a>>>),
    Print(Expr<'a>),
    Var(Rc<Token<'a>>, Option<Expr<'a>>),
    Block(Vec<Stmt<'a>>),
    While(Expr<'a>, Box<Stmt<'a>>)
}