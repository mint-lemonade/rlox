use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{expr::Literals, interpreter::RuntimeError, token::Token};

// type Scope = usize;
#[derive(Debug)]
struct Scope {
    pub values: HashMap<String, Option<Literals>>,
    pub enclosing: Option<Rc<RefCell<Scope>>>,
}
impl Scope {
    pub fn new(enclosing: Option<Rc<RefCell<Scope>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }
}
pub struct Environment {
    scope: Rc<RefCell<Scope>>,
}

impl Environment {
    pub fn create_new_scope(&mut self) {
        let new_scope = Scope::new(Some(self.scope.clone()));
        self.scope = Rc::new(RefCell::new(new_scope));
    }

    pub fn end_latest_scope(&mut self) {
        let old_scope = self.scope.borrow().enclosing.clone();
        self.scope = old_scope.expect("Expected Scope!");
    }

    pub fn define(&self, name: String, value: Option<Literals>) {
        self.scope.borrow_mut().values.insert(name, value);
    }

    pub fn get(&self, name: Rc<Token>) -> Result<Literals, RuntimeError> {
        let mut curr_scope = self.scope.clone();
        loop {
            if curr_scope.borrow().values.contains_key(&name.lexeme) {
                let value = curr_scope
                    .borrow()
                    .values
                    .get(&name.lexeme)
                    .unwrap()
                    .clone();
                return Ok(value.unwrap_or(Literals::Nil));
            }
            if curr_scope.borrow().enclosing.is_none() {
                break;
            }

            curr_scope = curr_scope
                .clone()
                .borrow()
                .enclosing
                .as_ref()
                .unwrap()
                .clone();
        }

        let err_mssg = format!("Undefined variable '{}'", name.lexeme);
        Err(RuntimeError::new(name, err_mssg))
    }

    pub fn assign(
        &mut self,
        var_name: Rc<Token>,
        value: Literals,
    ) -> Result<Literals, RuntimeError> {
        let mut curr_scope = self.scope.clone();
        loop {
            if curr_scope.borrow().values.contains_key(&var_name.lexeme) {
                curr_scope
                    .borrow_mut()
                    .values
                    .insert(var_name.lexeme.to_string(), Some(value.clone()));
                return Ok(value);
            }
            if curr_scope.borrow().enclosing.is_none() {
                break;
            }
            curr_scope = curr_scope
                .clone()
                .borrow()
                .enclosing
                .as_ref()
                .unwrap()
                .clone();
        }

        let err_mssg = format!("Undefined variable '{}'", var_name.lexeme);
        Err(RuntimeError::new(var_name, err_mssg))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scope: Rc::new(RefCell::new(Scope::new(None))),
        }
    }
}
