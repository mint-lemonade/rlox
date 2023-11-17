use std::{fmt::Debug, rc::Rc, cell::Cell};

use super::{expr::{Literals, Expr}, interpreter::{RuntimeError, Interpreter}, stmt::Stmt};

thread_local!{ 
    pub static FUNCTION_ID: Cell<usize> = Cell::new(1);
}
#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Native(NativeFn),
    Foreign(ForeignFn)
}

impl Callable {
    pub fn new_native_fn(func: Rc<FuncCall>, arity: usize) -> Self {
        Self::Native(NativeFn::new(func, arity, Self::get_inc_func_id()))
    }

    pub fn new_foreign_fn(declaration_idx: usize, arity: usize) -> Self {
        Self::Foreign(ForeignFn::new(declaration_idx, arity, Self::get_inc_func_id()))
    }

    fn get_inc_func_id() -> usize {
        // Assign new incrementing id to every new function.
        let mut id: usize = 0;
        FUNCTION_ID.with(|func_id| {
            id = func_id.get();
            func_id.set(id + 1);
        });
        id
    }

    // pub fn call(&self)
}
// type FuncCall = dyn  Fn(Vec<Literals>) -> Result<Literals, RuntimeError<'_>>;
// type FuncCall = dyn  Fn(Option<Vec<Literals>>) -> Literals;
type FuncCall = dyn Fn(Vec<Literals>) -> Literals;
// type FuncCall = fn(Vec<Literals>) -> Literals;

pub struct NativeFn {
    id: usize,
    pub arity: usize,
    pub call: Rc<FuncCall>
}

impl NativeFn {
    pub fn new(func: Rc<FuncCall>, arity: usize, id: usize) -> Self {
        Self {
            id,
            arity,
            call: func
        }
    }
}


impl Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callable")
        .field("call", &"<native-function>")
        .field("id", &self.id)
        .finish()
    }
}

impl Clone for NativeFn {
    fn clone(&self) -> Self {
        Self { call: self.call.clone(), id: self.id, arity: self.arity }
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
pub struct ForeignFn {
    id: usize,
    pub arity: usize,
    pub declaration_stmt_index: usize
    // pub declaration_idx: Box<Stmt<'a>>
}

impl ForeignFn {
    pub fn new(declaration_idx: usize, arity: usize, id: usize) -> Self {
        Self {
            id,
            arity,
            declaration_stmt_index: declaration_idx
        }
    }

    pub fn call<'a>(&self, intrprtr: &mut Interpreter, ast: &[Stmt<'a>], args: Vec<Literals>) -> Result<Literals, RuntimeError<'a>>{
        // Fetch function declaration
        let declaration = &ast[self.declaration_stmt_index];
        let Stmt::Function { 
            name, params, body 
        } = declaration else { 
            panic!("Non Callable object being called.")
        };
        if self.arity != args.len() {
            return Err(RuntimeError::new(name.clone(), format!("Expected {} arguments, received {}", self.arity, args.len())));
        }

        intrprtr.environment.create_new_scope();
        // Assign arguments to variables in current environment.
        for (value, param) in args.into_iter().zip(params) {
            intrprtr.environment.define(param.lexeme.to_string(), Some(value))
        }

        // Execute function body.
        intrprtr.execute_block(body)?;
        Ok(Literals::Nil)
    }
}

impl PartialEq for ForeignFn {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}