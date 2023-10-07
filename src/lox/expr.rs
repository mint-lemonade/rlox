use super::token::Token;

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    StringLiteral(String),
    NumberLiteral(f64),
    Unary(Token<'a>, Box<Expr<'a>>)
}