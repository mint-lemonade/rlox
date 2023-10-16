use std::rc::Rc;

use super::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt<'a> {
    /// Expression( expr )
    Expression(Expr<'a>),
    /// If( condition, then_statmenet, else_statement )
    If(Expr<'a>, Box<Stmt<'a>>, Box<Option<Stmt<'a>>>),
    /// Print( expr )
    Print(Expr<'a>),
    /// Var( var_name, initializer )
    Var(Rc<Token<'a>>, Option<Expr<'a>>),
    /// Block( statements )
    Block(Vec<Stmt<'a>>),
    /// While( condition, body )
    While(Expr<'a>, Box<Stmt<'a>>)
}