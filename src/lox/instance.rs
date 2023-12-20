use std::collections::HashMap;
use std::rc::Rc;
use std::cell::Cell;

use super::class::Class;
use super::expr::Literals;
use super::interpreter::RuntimeError;
use super::token::Token;

thread_local!{ 
    pub static Instance_ID: Cell<usize> = Cell::new(1);
}

#[derive(Debug, Clone)]
pub struct Instance {
    id: usize,
    // pub name: String,
    // methods: Vec<ForeignFn>
    pub class: Rc<Class>,
    fields: HashMap<String, Literals>
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self {
            id: Self::get_inc_expr_id(),
            class,
            fields: HashMap::new()
        }
    }

    fn get_inc_expr_id() -> usize {
        // Assign new incrementing id to every new instance.
        let mut id: usize = 0;
        Instance_ID.with(|expr_id| {
            id = expr_id.get();
            expr_id.set(id + 1);
        });
        id
    }

    pub fn get(&self, property: Rc<Token>) -> Result<Literals, RuntimeError> {
        dbg!(&self.fields);

        self.fields.get(&property.lexeme)
        .cloned()
        .ok_or_else(|| RuntimeError::new(
            property.clone(), 
            format!("Undefined property '{}'", property.lexeme)
        ))
    }

    pub fn set(&mut self, property: Rc<Token>, value: Literals) {
        self.fields.insert(property.lexeme.to_string(), value);
        dbg!(&self.fields);
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}