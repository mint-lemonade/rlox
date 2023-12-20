use std::{fmt::Debug, rc::Rc, cell::{Cell, RefCell}};

use super::{expr::Literals, interpreter::{RuntimeError, Interpreter, self}, stmt::Stmt, printer::Print, environment::Scope, class::Class, instance::Instance};

thread_local!{ 
    pub static FUNCTION_ID: Cell<usize> = Cell::new(1);
}
#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Native(NativeFn),
    Foreign(ForeignFn),
    Class(ClassInitializer)
}

impl Callable {
    pub fn new_native_fn(func: Rc<FuncCall>, arity: usize, name: String) -> Self {
        Self::Native(NativeFn::new(func, arity, name, Self::get_inc_func_id()))
    }

    pub fn new_foreign_fn(declaration_stmt: Rc<Stmt>, name: String, arity: usize, closure: Rc<RefCell<Scope>>) -> Self {
        Self::Foreign(ForeignFn::new( declaration_stmt, name, arity, Self::get_inc_func_id(), closure))
    }

    pub fn new_class_initializer(class: Rc<Class>) -> Self {
        Self::Class(ClassInitializer::new(class, Self::get_inc_func_id()))
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

#[derive(Clone)]
pub struct NativeFn {
    id: usize,
    pub name: String,
    pub arity: usize,
    pub call: Rc<FuncCall>
}

impl NativeFn {
    pub fn new(func: Rc<FuncCall>, arity: usize, name: String, id: usize) -> Self {
        Self {
            id,
            name,
            arity,
            call: func
        }
    }
}

impl Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callable")
        .field("name", &self.name)
        .field("id", &self.id)
        .field("call", &"<native-fn>")
        .finish()
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
    pub name: String,
    pub arity: usize,
    declaration_stmt: Rc<Stmt>,
    closure: Rc<RefCell<Scope>>
}

impl ForeignFn {
    pub fn new(
        declaration_stmt: Rc<Stmt>,
        name: String, 
        arity: usize, id: usize,
        closure: Rc<RefCell<Scope>>
    ) -> Self {
        Self {
            id,
            name,
            arity,
            declaration_stmt,
            closure
        }
    }

    pub fn call<T: Print>(
        &self,
        intrprtr: &mut Interpreter<T>,
        args: Vec<Literals>
    ) -> Result<Literals, RuntimeError> {
        // Fetch function declaration
        // let declaration = &declaration_refs[self.declaration_stmt_index].clone();
        let declaration = self.declaration_stmt.as_ref();
        let Stmt::Function { 
            name, params, body 
        } = declaration else { 
            panic!("Non Callable object being called.")
        };
        if self.arity != args.len() {
            return Err(RuntimeError::new(
                name.clone(), 
                format!("Expected {} arguments, received {}", self.arity, args.len())
            ));
        }

        let back_to_scope = intrprtr.environment.scope.clone();
        intrprtr.environment.scope = self.closure.clone();
        intrprtr.environment.create_new_scope();

        // Assign arguments to variables in current environment.
        for (value, param) in args.into_iter().zip(params) {
            intrprtr.environment.define(param.lexeme.to_string(), Some(value))
        }
        // Execute function body.
        let result = intrprtr.execute_block(body, false)?;

        intrprtr.environment.end_latest_scope();
        intrprtr.environment.scope = back_to_scope;

        Ok(result.unwrap_or(Literals::Nil))
    }
}

impl PartialEq for ForeignFn {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
pub struct ClassInitializer {
    id: usize,
    arity: usize,
    pub class: Rc<Class>
}

impl PartialEq for ClassInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl ClassInitializer {
    pub fn new(class: Rc<Class>, id: usize) -> Self {
        Self {
            id,
            class,
            arity: 0
        }
    }

    pub fn call<T: Print>(
        &self,
        interpreter: &Interpreter<T>,
        args: Vec<Literals>
    ) -> Result<Literals, RuntimeError> {
        if self.arity != args.len() {
            return Err(RuntimeError::new(
                self.class.name.clone(), 
                format!("Expected {} arguments, received {}", self.arity, args.len())
            ));
        }
        let instance = Instance::new(self.class.clone());
        Ok(Literals::Instance(instance))
    }
}