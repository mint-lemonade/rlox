use std::{collections::HashMap, rc::Rc};

use super::{expr::Literals, interpreter::RuntimeError, token::Token};

pub struct Environment {
    values: Vec<HashMap<String, Option<Literals>>>,
    current_env: usize
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: vec![HashMap::new()],
            current_env: 0
        }
    }

    pub fn create_new_scope(&mut self) {
        self.values.push(HashMap::new());
        self.current_env += 1;
    }

    pub fn end_latest_scope(&mut self) {
        self.values.pop();
        self.current_env -= 1;
    }

    pub fn define(&mut self, name: String, value: Option<Literals>) {
        self.values[self.current_env].insert(name, value);
    }

    pub fn get<'a>(
        &self, name: Rc<Token<'a>>
    ) -> Result<Literals, RuntimeError<'a>> {
        let mut curr_scope = self.current_env;
        loop {
            if self.values[curr_scope].contains_key(name.lexeme) {
                let value = self.values[curr_scope].get(name.lexeme).unwrap().clone();
                return Ok(value.unwrap_or(Literals::Nil));
                
            }
            if curr_scope == 0 {
                break;
            }
            curr_scope -= 1;
        }

        let err_mssg = format!("Undefined variable '{}'", name.lexeme);
        Err(RuntimeError::new(name, err_mssg))
    }

    pub fn assign<'a>(
        &mut self, var_name: Rc<Token<'a>>, value: Literals
    ) -> Result<Literals, RuntimeError<'a>> {
        let mut curr_scope = self.current_env;
        loop { 
            if self.values[curr_scope].contains_key(var_name.lexeme) {
                self.values[curr_scope].insert(var_name.lexeme.to_string(), Some(value.clone()));
                return Ok(value);         
            }
            if curr_scope == 0 {
                break;
            }
            curr_scope -= 1;
        }

        let err_mssg = format!("Undefined variable '{}'", var_name.lexeme);
        Err(RuntimeError::new(var_name, err_mssg))
    }
}