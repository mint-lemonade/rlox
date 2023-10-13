use std::{collections::HashMap, rc::Rc};

use super::{expr::Literals, interpreter::RuntimeError, token::Token};

pub struct Environment {
    values: HashMap<String, Option<Literals>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: Option<Literals>) {
        self.values.insert(name, value);
    }

    pub fn get<'a>(
        &self, name: Rc<Token<'a>>
    ) -> Result<Literals, RuntimeError<'a>> {
        if self.values.contains_key(name.lexeme) {
            let value = self.values.get(name.lexeme).unwrap().clone();
            return Ok(value.unwrap_or(Literals::Nil));
            
        }
        Err(RuntimeError::new(name, "Undefined variable '{}'"))
    }

    pub fn assign<'a>(
        &mut self, var_name: Rc<Token<'a>>, value: Literals
    ) -> Result<Literals, RuntimeError<'a>> {
        if self.values.contains_key(var_name.lexeme) {
            self.values.insert(var_name.lexeme.to_string(), Some(value.clone()));
            return Ok(value);
            
        }
        Err(RuntimeError::new(var_name, "Undefined variable '{}'"))
    }
}