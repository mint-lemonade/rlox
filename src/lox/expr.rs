use std::rc::Rc;

use super::token::Token;

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Rc<Token<'a>>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    StringLiteral(String),
    NumberLiteral(f64),
    BoolLiteral(bool),
    NilLiteral,
    Unary(Rc<Token<'a>>, Box<Expr<'a>>)
}