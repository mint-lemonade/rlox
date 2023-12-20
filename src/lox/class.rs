use std::rc::Rc;

use super::{callable::{Callable, ForeignFn}, stmt::Stmt, token::Token};

#[derive(Debug)]
pub struct Class {
    pub name: Rc<Token>,
    // methods: Vec<ForeignFn>
}

impl Class {
    pub fn new(name: Rc<Token>) -> Self {
    // pub fn new(name: String, methods: Vec<ForeignFn>) -> Self {
        Self {
            name,
            // methods
        }
    }
}