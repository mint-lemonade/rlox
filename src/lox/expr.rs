use std::rc::Rc;

use super::token::Token;

#[derive(Debug)]
pub enum Expr<'a> {
    /// Assign(var_name, rvalue)
    Assign(Rc<Token<'a>>, Box<Expr<'a>>),
    /// Binary(left, operation, right)
    Binary(Box<Expr<'a>>, Rc<Token<'a>>, Box<Expr<'a>>),
    /// Grouping(expr)
    Grouping(Box<Expr<'a>>),
    /// Literal(literal)
    Literal(Literals),
    /// Logical(left, operation, right)
    Logical(Box<Expr<'a>>, Rc<Token<'a>>, Box<Expr<'a>>),
    /// Unary(operation, expr)
    Unary(Rc<Token<'a>>, Box<Expr<'a>>),
    /// Variable(var_name)
    Variable(Rc<Token<'a>>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl From<Literals> for Expr<'_> {
    fn from(value: Literals) -> Self {
        Self::Literal(value)
    }
}