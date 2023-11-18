use std::rc::Rc;

use super::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    /// Expression( expr )
    Expression(Expr<'a>),
    /// Function
    Function { name: Rc<Token<'a>>, params: Vec<Rc<Token<'a>>>, body: Vec<Stmt<'a>>},
    /// If( condition, then_statmenet, else_statement )
    If(Expr<'a>, Box<Stmt<'a>>, Box<Option<Stmt<'a>>>),
    /// Print( expr )
    Print(Expr<'a>),
    /// Var( var_name, initializer )
    Var(Rc<Token<'a>>, Option<Expr<'a>>),
    /// Block( statements )
    Block(Vec<Stmt<'a>>),
    /// While( condition, body )
    While(Expr<'a>, Box<Stmt<'a>>),

    Return { return_keyword: Rc<Token<'a>>, expression: Option<Expr<'a>> }
}