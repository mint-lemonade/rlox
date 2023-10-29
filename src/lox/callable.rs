use std::{fmt::Debug, rc::Rc, cell::Cell};

use super::{expr::{Literals, Expr}, interpreter::RuntimeError};

thread_local!{ 
    pub static FUNCTION_ID: Cell<usize> = Cell::new(1);
}

// type FuncCall = dyn  Fn(Vec<Literals>) -> Result<Literals, RuntimeError<'_>>;
// type FuncCall = dyn  Fn(Option<Vec<Literals>>) -> Literals;
type FuncCall = dyn  Fn(Vec<Literals>) -> Literals;
pub struct Callable {
    id: usize,
    pub call: Rc<FuncCall>
}

impl Callable {
    pub fn new(func: Rc<FuncCall>) -> Self {

        // Assign new incrementing id to every new function.
        let mut id: usize = 0;
        FUNCTION_ID.with(|func_id| {
            id = func_id.get();
            func_id.set(id + 1);
        });

        Self {
            id,
            call: func
        }
    }
}


impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callable").field("func", &"<function>").finish()
    }
}

impl Clone for Callable {
    fn clone(&self) -> Self {
        Self { call: self.call.clone(), id: self.id }
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}