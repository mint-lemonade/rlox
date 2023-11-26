use std::rc::Rc;

use super::{token::Token, callable::Callable};

#[derive(Debug, Clone)]
pub enum Expr {
    /// Assign(var_name, rvalue)
    Assign(Rc<Token>, Box<Expr>),
    /// Binary(left, operation, right)
    Binary(Box<Expr>, Rc<Token>, Box<Expr>),
    /// Grouping(expr)
    Grouping(Box<Expr>),
    /// Literal(literal)
    Literal(Literals),
    /// Logical(left, operation, right)
    Logical(Box<Expr>, Rc<Token>, Box<Expr>),
    /// Unary(operation, expr)
    Unary(Rc<Token>, Box<Expr>),
    /// Variable(var_name)
    Variable(Rc<Token>),
    /// Call( callee, paren, arguments )
    Call{ callee: Box<Expr>, paren: Rc<Token>, arguments: Vec<Expr> }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    Function(Callable)
}

impl From<Literals> for Expr {
    fn from(value: Literals) -> Self {
        Self::Literal(value)
    }
}