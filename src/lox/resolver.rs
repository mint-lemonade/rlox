use std::{collections::HashMap, rc::Rc};

use super::{
    error_reporter::ErrorReporter,
    expr::{Expr, ExprType},
    interpreter::Interpreter,
    printer::Print,
    stmt::Stmt,
    token::Token,
};

pub struct Resolver<'a, 'p, T: Print> {
    // pub environment: Environment,
    err_reporter: &'a ErrorReporter<'a, 'p, T>,
    interpreter: &'a mut Interpreter<'p, T>,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a, 'p, T: Print> Resolver<'a, 'p, T> {
    pub fn new(
        err_reporter: &'a ErrorReporter<'a, 'p, T>,
        interpreter: &'a mut Interpreter<'p, T>,
    ) -> Self {
        Self {
            err_reporter,
            interpreter,
            scopes: vec![],
        }
    }

    pub fn resolve(&mut self, stmts: &'a Vec<Stmt>) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &'a Stmt) {
        match stmt {
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::Function { name, params, body } => {
                self.resolve_function_declaration(name, params, body)
            }
            Stmt::If(condtion, then_stmt, else_stmt) => {
                self.resolve_if_stmt(condtion, then_stmt, else_stmt)
            }
            Stmt::Print(expr) => self.resolve_print_stmt(expr),
            Stmt::Var(name, expr) => self.resolve_var_stmt(name, expr),
            Stmt::Block(stmts) => self.resolve_block_stmt(stmts),
            Stmt::While(condition, body) => self.resolve_while_stmt(condition, body),
            Stmt::Return {
                return_keyword: _,
                expression,
            } => self.resolve_return_stmt(expression),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Rc<Token>) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.to_string(), false);
    }

    fn define(&mut self, name: &Rc<Token>) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.to_string(), true);
    }

    fn resolve_block_stmt(&mut self, stmts: &'a Vec<Stmt>) {
        self.begin_scope();
        self.resolve(stmts);
        self.end_scope();
    }

    fn resolve_var_stmt(&mut self, name: &Rc<Token>, expr: &Option<Expr>) {
        if self
            .scopes
            .last()
            .is_some_and(|scope| scope.contains_key(&name.lexeme))
        {
            self.err_reporter.error_token(name.clone(), "Already a variable with this name in this scope")
        }
        self.declare(name);
        if let Some(expr) = expr {
            self.resolve_expr(expr);
        }
        self.define(name);
    }

    fn resolve_function_declaration(
        &mut self,
        name: &Rc<Token>,
        params: &Vec<Rc<Token>>,
        body: &'a Vec<Stmt>,
    ) {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body);
    }

    fn resolve_function(&mut self, params: &Vec<Rc<Token>>, body: &'a Vec<Stmt>) {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(body);
        self.end_scope()
    }

    fn resolve_if_stmt(
        &mut self,
        condition: &Expr,
        then_stmt: &'a Stmt,
        else_stmt: &'a Option<Stmt>,
    ) {
        self.resolve_expr(condition);
        self.resolve_stmt(then_stmt);
        if let Some(else_stmt) = else_stmt {
            self.resolve_stmt(else_stmt);
        }
    }

    fn resolve_print_stmt(&mut self, expr: &Expr) {
        self.resolve_expr(expr);
    }

    fn resolve_return_stmt(&mut self, expr: &Option<Expr>) {
        if let Some(expr) = expr {
            self.resolve_expr(expr)
        }
    }

    fn resolve_while_stmt(&mut self, condition: &Expr, body: &'a Stmt) {
        self.resolve_expr(condition);
        self.resolve_stmt(body);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match &expr.expr_type {
            ExprType::Assign(var_name, value_expr) => {
                self.resolve_assign_expr(var_name, value_expr, expr)
            }
            ExprType::Binary(left, _op, right) => self.resolve_binary_expr(left, right),
            ExprType::Grouping(expr) => self.resolve_expr(expr),
            ExprType::Literal(_) => (),
            ExprType::Logical(left, _op, right) => self.resolve_logical_expr(left, right),
            ExprType::Unary(_op, expr) => self.resolve_expr(expr),
            ExprType::Variable(name) => self.resolve_var_expr(name, expr),
            ExprType::Call {
                callee,
                paren: _,
                arguments,
            } => self.resolve_call_expr(callee, arguments),
        }
    }

    fn resolve_var_expr(&mut self, name: &Rc<Token>, expr: &Expr) {
        if !self.scopes.is_empty()
            && self
                .scopes
                .last()
                .unwrap()
                .get(&name.lexeme)
                .is_some_and(|defined| !(*defined))
        {
            self.err_reporter.error_token(
                name.clone(),
                "Can't read local variables in its own declaration.",
            )
        }
        self.resolve_local(name, expr.id);
    }

    fn resolve_assign_expr(&mut self, name: &Rc<Token>, value_expr: &Expr, assign_expr: &Expr) {
        self.resolve_expr(value_expr);
        self.resolve_local(name, assign_expr.id);
    }

    fn resolve_binary_expr(&mut self, left: &Expr, right: &Expr) {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn resolve_call_expr(&mut self, callee: &Expr, args: &Vec<Expr>) {
        self.resolve_expr(callee);
        for arg in args {
            self.resolve_expr(arg);
        }
    }

    fn resolve_logical_expr(&mut self, left: &Expr, right: &Expr) {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn resolve_local(&mut self, name: &Rc<Token>, expr_id: usize) {
        let mut i = self.scopes.len();
        loop {
            if i == 0 {
                break;
            }
            i -= 1;
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter.resolve(expr_id, self.scopes.len() - 1 - i);
                return;
            }
        }
    }
}
