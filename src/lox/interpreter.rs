use std::{collections::HashMap, rc::Rc, time::SystemTime};

use super::{
    callable::Callable,
    environment::Environment,
    error_reporter::ErrorReporter,
    expr::{Expr, ExprType, Literals},
    printer::Print,
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
};
#[derive(Debug)]
pub struct RuntimeError {
    token: Rc<Token>,
    message: String,
}
impl RuntimeError {
    pub fn new(token: Rc<Token>, message: String) -> Self {
        Self { token, message }
    }
}

pub struct Interpreter<'p, T: Print> {
    pub environment: Environment,
    pub printer: &'p T,
    /// locals: HashMap<Expr.id, depth>
    /// where "depth" is scope to which Expr is resolved.
    locals: HashMap<usize, usize>,
}

impl<'p, T: Print> Interpreter<'p, T> {
    pub fn new(printer: &'p T) -> Self {
        let interpreter = Self {
            environment: Environment::default(),
            printer,
            locals: HashMap::new(),
        };

        // Define native function "clock()" to return current time in secs
        interpreter.environment.define(
            "clock".to_string(),
            Some(Literals::Function(Callable::new_native_fn(
                Rc::new(
                    |_args| match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(d) => Literals::Number(d.as_secs_f64()),
                        Err(_) => Literals::Nil,
                    },
                ),
                0,
                "clock".to_string(),
            ))),
        );

        // Define native to_string() function to convert any Lox datatype
        // into its string representation.
        interpreter.environment.define(
            "to_string".to_string(),
            Some(Literals::Function(Callable::new_native_fn(
                Rc::new(|args| {
                    let literal = &args[0];
                    match literal {
                        Literals::String(_) => literal.clone(),
                        Literals::Number(n) => Literals::String(n.to_string()),
                        Literals::Bool(b) => Literals::String(b.to_string()),
                        Literals::Nil => Literals::String("Nil".to_string()),
                        Literals::Function(f) => match f {
                            Callable::Native(native) => {
                                Literals::String(format!("<native-fn {}()>", native.name))
                            }
                            Callable::Foreign(foreign) => {
                                Literals::String(format!("<fn {}()>", foreign.name))
                            }
                        },
                    }
                }),
                1,
                "to_string".to_string(),
            ))),
        );

        interpreter
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>, err_reporter: &ErrorReporter<T>) -> i32 {
        for statement in statements {
            let result = self.execute(statement);
            match result {
                Ok(_) => (),
                Err(e) => {
                    err_reporter.runtime_error(e.token, e.message);
                    return 70; // 70: An internal software error has been detected
                }
            }
        }
        0
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Option<Literals>, RuntimeError> {
        match statement {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(None)
            }

            Stmt::Print(expr) => {
                self.execute_print_stmt(expr)?;
                Ok(None)
            }

            Stmt::Var(name, initializer) => {
                self.execute_var_declaration_stmt(name.clone(), initializer.as_ref())?;
                Ok(None)
            }

            Stmt::Block(stmts) => self.execute_block(stmts, true),

            Stmt::If(condition, then_stmt, else_stmt) => {
                self.execute_if_stmt(condition, then_stmt, else_stmt)
            }

            Stmt::While(condition, body) => self.execute_while_statement(condition, body),

            Stmt::Function {
                name,
                params,
                body: _,
            } => {
                self.execute_fun_declaration_stmt(name.clone(), statement, params.len());
                Ok(None)
            }
            Stmt::Return {
                return_keyword,
                expression,
            } => Ok(Some(
                self.execute_return_stmt(return_keyword.clone(), expression)?,
            )),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Literals, RuntimeError> {
        match &expr.expr_type {
            ExprType::Binary(left, op, right) => self.interpret_binary(op.clone(), left, right),
            ExprType::Grouping(grp) => self.interpret_group(grp),
            // TODO: Avoid cloning especially for String
            ExprType::Literal(literal) => Ok(literal.clone()),
            ExprType::Unary(op, right) => self.interpret_unary(op.clone(), right),
            ExprType::Variable(variable) => self.interpret_variable(variable.clone(), expr),
            ExprType::Assign(var_name, rvalue) => {
                self.execute_assign_expr(var_name.clone(), rvalue, expr)
            }
            ExprType::Logical(left, op, right) => self.interpret_logical(op.clone(), left, right),
            ExprType::Call {
                callee,
                paren,
                arguments,
            } => self.interpret_call(callee, paren, arguments),
        }
    }

    fn interpret_unary(&mut self, op: Rc<Token>, right: &Expr) -> Result<Literals, RuntimeError> {
        let right = self.evaluate(right)?;
        match op.token_type {
            TokenType::Minus => match right {
                Literals::Number(n) => Ok(Literals::Number(-n)),
                _ => Err(RuntimeError::new(op, "Operand must be number".into())),
            },
            TokenType::Bang => {
                // match right {
                //     Literals::Bool(b) => Ok(Literals::Bool(!b)),
                //     // Nil is falsey => !Nil is truthy
                //     Literals::Nil => Ok(Literals::Bool(true)),
                //     // everthing_else is truthy => !everything_else is falsey
                //     _ => Ok(Literals::Bool(false)),
                // }
                Ok(Literals::Bool(!Self::into_bool(&right)))
            }
            _ => unreachable!(),
        }
    }

    fn into_bool(literal: &Literals) -> bool {
        match literal {
            Literals::Bool(b) => *b,
            Literals::Nil => false,
            _ => true,
        }
    }

    fn interpret_group(&mut self, expr: &Expr) -> Result<Literals, RuntimeError> {
        self.evaluate(expr)
    }

    fn interpret_variable(
        &mut self,
        var_name: Rc<Token>,
        var_expr: &Expr,
    ) -> Result<Literals, RuntimeError> {
        self.lookup_variable(var_name, var_expr)
        // self.environment.get(var_name)
    }

    fn lookup_variable(&self, var_name: Rc<Token>, expr: &Expr) -> Result<Literals, RuntimeError> {
        if let Some(distance) = self.locals.get(&expr.id) {
            self.environment.get_at(*distance, var_name)
        } else {
            self.environment.get_global(var_name)
        }
    }

    fn interpret_binary(
        &mut self,
        op: Rc<Token>,
        left: &Expr,
        right: &Expr,
    ) -> Result<Literals, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match op.token_type {
            TokenType::Plus => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a + b));
                    }
                }
                if let Literals::String(a) = left {
                    if let Literals::String(b) = right {
                        // a.push_str(&b);
                        return Ok(Literals::String(format!("{}{}", a, b)));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Both Operands must be either number or string.".to_string(),
                ))
            }
            TokenType::Minus => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a - b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::Slash => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a / b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::Star => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a * b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::Greater => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a > b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::GreaterEqual => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a >= b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::Less => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a < b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::LessEqual => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a <= b));
                    }
                }
                Err(RuntimeError::new(
                    op.clone(),
                    "Operands must be number.".to_string(),
                ))
            }
            TokenType::BangEqual => Ok(Literals::Bool(left != right)),
            TokenType::EqualEqual => Ok(Literals::Bool(left == right)),
            _ => unreachable!(),
        }
    }

    fn interpret_logical(
        &mut self,
        op: Rc<Token>,
        left: &Expr,
        right: &Expr,
    ) -> Result<Literals, RuntimeError> {
        let left = self.evaluate(left)?;
        match op.token_type {
            TokenType::Or => {
                if Self::into_bool(&left) {
                    return Ok(left);
                }
            }
            TokenType::And => {
                if !Self::into_bool(&left) {
                    return Ok(left);
                }
            }
            _ => unreachable!(),
        }
        self.evaluate(right)
    }

    fn interpret_call(
        &mut self,
        callee: &Expr,
        paren: &Rc<Token>,
        args: &[Expr],
    ) -> Result<Literals, RuntimeError> {
        let callee = self.evaluate(callee)?;
        let mut arguments = vec![];
        for arg in args {
            arguments.push(self.evaluate(arg)?);
        }
        if let Literals::Function(Callable::Native(function)) = callee {
            if arguments.len() != function.arity {
                return Err(RuntimeError::new(
                    paren.clone(),
                    format!(
                        "Expected {} arguments, recieved {}",
                        function.arity,
                        arguments.len()
                    ),
                ));
            }
            Ok((function.call)(arguments))
        } else if let Literals::Function(Callable::Foreign(function)) = callee {
            match function.call(self, arguments) {
                Ok(return_val) => Ok(return_val),
                Err(err) => Err(RuntimeError::new(paren.clone(), err.message)),
            }
        } else {
            Err(RuntimeError::new(
                paren.clone(),
                "Can only call functions and classes".to_string(),
            ))
        }
    }

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],

        with_new_scope: bool,
    ) -> Result<Option<Literals>, RuntimeError> {
        if with_new_scope {
            self.environment.create_new_scope();
        }
        let mut return_value = None;
        for stmt in stmts {
            return_value = self.execute(stmt)?;
            if return_value.is_some() {
                break;
            }
        }
        if with_new_scope {
            self.environment.end_latest_scope();
        }
        Ok(return_value)
    }

    fn execute_if_stmt(
        &mut self,
        condition: &Expr,
        then_stmt: &Stmt,
        else_statement: &Option<Stmt>,
    ) -> Result<Option<Literals>, RuntimeError> {
        if Self::into_bool(&self.evaluate(condition)?) {
            return self.execute(then_stmt);
        } else if let Some(else_stmt) = else_statement {
            return self.execute(else_stmt);
        }
        Ok(None)
    }

    fn execute_print_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        match value {
            Literals::Nil => self.printer.print(&"Nil"),
            Literals::String(s) => self.printer.print(&s),
            Literals::Number(n) => self.printer.print(&n),
            Literals::Bool(b) => self.printer.print(&b),
            Literals::Function(_) => self.printer.print(&"<fn>"),
        }
        Ok(())
    }

    fn execute_while_statement(
        &mut self,
        condition: &Expr,
        body: &Stmt,
    ) -> Result<Option<Literals>, RuntimeError> {
        while Self::into_bool(&self.evaluate(condition)?) {
            let return_value = self.execute(body)?;
            if return_value.is_some() {
                return Ok(return_value);
            }
        }
        Ok(None)
    }

    fn execute_var_declaration_stmt(
        &mut self,
        name: Rc<Token>,
        expr: Option<&Expr>,
    ) -> Result<(), RuntimeError> {
        let value = if expr.is_some() {
            Some(self.evaluate(expr.unwrap())?)
        } else {
            None
        };
        self.environment.define(name.lexeme.to_string(), value);
        Ok(())
    }

    fn execute_fun_declaration_stmt(&mut self, name: Rc<Token>, stmt: &Stmt, arity: usize) {
        self.environment.define(
            name.lexeme.to_string(),
            Some(Literals::Function(
                // len() - 1 is the index of function declaration just pushed
                // in declaration_refs. To be later used when evaluating call expr.
                Callable::new_foreign_fn(
                    Rc::new(stmt.clone()),
                    name.lexeme.to_string(),
                    arity,
                    self.environment.scope.clone(),
                ),
            )),
        );
    }

    fn execute_return_stmt(
        &mut self,
        _return_keyword: Rc<Token>,
        expression: &Option<Expr>,
    ) -> Result<Literals, RuntimeError> {
        self.evaluate(expression.as_ref().unwrap_or(&Literals::Nil.into()))
    }

    fn execute_assign_expr(
        &mut self,
        name: Rc<Token>,
        rvalue: &Expr,
        expr: &Expr,
    ) -> Result<Literals, RuntimeError> {
        let value = self.evaluate(rvalue)?;
        // self.environment.assign(name, value)
        if let Some(distance) = self.locals.get(&expr.id) {
            self.environment.assign_at(*distance, name, value)
        } else {
            self.environment.assign_global(name, value)
        }
    }

    pub fn resolve(&mut self, expr_id: usize, depth: usize) {
        self.locals.insert(expr_id, depth);
    }
}

#[cfg(test)]
mod test {
    use crate::lox::{
        error_reporter::ErrorReporter, expr::Literals, parser::Parser, printer::TestPrinter,
        scanner::Scanner, stmt::Stmt,
    };

    use super::Interpreter;

    #[test]
    fn direct_expression_evaluation() {
        let source = "(5 - (3 - 1)) + -1;";
        let printer = TestPrinter::default();
        let error_reporter = ErrorReporter::new(source, false, &printer);

        let mut scanner = Scanner::new(source, &error_reporter);

        scanner.scan_tokens();
        if error_reporter.had_error.get() {
            panic!("Error while scanning.");
        }

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let Stmt::Expression(ast) = &parser.parse()[0] else {panic!()};
        if error_reporter.had_error.get() {
            panic!("Error while parsing.");
        }
        let mut interpreter = Interpreter::new(&printer);
        assert_eq!(interpreter.evaluate(ast).unwrap(), Literals::Number(2.0));
    }
}
