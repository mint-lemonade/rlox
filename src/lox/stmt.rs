use std::rc::Rc;

use super::{expr::Expr, token::Token};

pub enum Stmt<'a> {
    Expression(Expr<'a>),
    Print(Expr<'a>),
    Var(Rc<Token<'a>>, Option<Expr<'a>>),
    Block(Vec<Stmt<'a>>)

}