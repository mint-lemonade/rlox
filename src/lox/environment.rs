use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{expr::Literals, interpreter::RuntimeError, token::Token};

// type Scope = usize;
#[derive(Debug)]
pub struct Scope {
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
    pub scope: Rc<RefCell<Scope>>,
    pub globals: Rc<RefCell<Scope>>,
}

impl Environment {
    pub fn create_new_scope(&mut self) {
        let new_scope = Scope::new(Some(self.scope.clone()));
        self.scope = Rc::new(RefCell::new(new_scope));
    }

    pub fn end_latest_scope(&mut self) -> Rc<RefCell<Scope>> {
        let enclosing_scope = self.scope.borrow().enclosing.clone();
        let latest_scope = self.scope.clone();
        self.scope = enclosing_scope.expect("Expected Scope!");
        latest_scope
    }

    pub fn define(&self, name: String, value: Option<Literals>) {
        self.scope.borrow_mut().values.insert(name, value);
    }

    pub fn get_at(&self, distance: usize, name: Rc<Token>) -> Result<Literals, RuntimeError> {
        let resolved_scope = self.get_ancestor(distance);
        Ok(resolved_scope
            .clone()
            .borrow()
            .values
            .get(&name.lexeme)
            .unwrap()
            .clone()
            .unwrap_or(Literals::Nil))
    }

    pub fn assign_at(&self, distance: usize, name: Rc<Token>, value: Literals) -> Result<Literals, RuntimeError> {
        let resolved_scope = self.get_ancestor(distance);
        resolved_scope
            .borrow_mut()
            .values
            .insert(name.lexeme.clone(), Some(value.clone()));
        Ok(value)
    }

    fn get_ancestor(&self, mut distance: usize) -> Rc<RefCell<Scope>> {
        let mut resolved_scope = self.scope.clone();
        while distance > 0 {
            resolved_scope = resolved_scope
                .clone()
                .borrow()
                .enclosing
                .as_ref()
                .unwrap_or_else(|| panic!("Scope expected at depth {distance}"))
                .clone();
            distance -= 1;
        }
        resolved_scope
    }

    pub fn get_global(&self, name: Rc<Token>) -> Result<Literals, RuntimeError> {
        if self.globals.borrow().values.contains_key(&name.lexeme) {
            return Ok(self.globals.borrow().values.get(&name.lexeme).unwrap().clone().unwrap_or(Literals::Nil));
        }
        let err_mssg = format!("Undefined variable '{}'", name.lexeme);
        Err(RuntimeError::new(name, err_mssg))
    }

    pub fn assign_global(&self, name: Rc<Token>, value: Literals) -> Result<Literals, RuntimeError> {
        if self.globals.borrow().values.contains_key(&name.lexeme) {
            self.globals.borrow_mut().values.insert(name.lexeme.clone(), value.clone().into());
            return Ok(value);
        }
        let err_mssg = format!("Undefined variable '{}'", name.lexeme);
        Err(RuntimeError::new(name, err_mssg))
    }
}

impl Default for Environment {
    fn default() -> Self {
        let global = Rc::new(RefCell::new(Scope::new(None)));
        Self {
            scope: global.clone(),
            globals: global,
        }
    }
}
