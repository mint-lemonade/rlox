use std::rc::Rc;

use super::token::Token;

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Rc<Token<'a>>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(Literals),
    Unary(Rc<Token<'a>>, Box<Expr<'a>>),
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