use std::rc::Rc;
use std::cell::Cell;

use super::{token::Token, callable::Callable};

thread_local!{ 
    pub static EXPR_ID: Cell<usize> = Cell::new(1);
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub id: usize,
    pub expr_type: ExprType
}
impl Expr {
    fn new(expr_type: ExprType) -> Self {
        Self { id: Self::get_inc_expr_id(), expr_type }
    }

    fn get_inc_expr_id() -> usize {
        // Assign new incrementing id to every new expr.
        let mut id: usize = 0;
        EXPR_ID.with(|expr_id| {
            id = expr_id.get();
            expr_id.set(id + 1);
        });
        id
    }
}
#[derive(Debug, Clone)]
pub enum ExprType {
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
        // Self::Literal(value)
        Self::new(ExprType::Literal(value))
    }
}

impl From<ExprType> for Expr {
    fn from(value: ExprType) -> Self {
        Self::new(value)
    }
}