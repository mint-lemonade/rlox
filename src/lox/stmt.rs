use std::rc::Rc;

use super::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    /// Expression( expr )
    Expression(Expr),
    /// Function
    Function { name: Rc<Token>, params: Vec<Rc<Token>>, body: Vec<Stmt>},
    /// If( condition, then_statmenet, else_statement )
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    /// Print( expr )
    Print(Expr),
    /// Var( var_name, initializer )
    Var(Rc<Token>, Option<Expr>),
    /// Block( statements )
    Block(Vec<Stmt>),
    /// While( condition, body )
    While(Expr, Box<Stmt>),

    Return { return_keyword: Rc<Token>, expression: Option<Expr> },
    
    Class { name: Rc<Token>, methods: Vec<Stmt> }
}