use std::{rc::Rc, time::SystemTime};

use super::{
    callable::Callable,
    environment::Environment,
    error_reporter::ErrorReporter,
    expr::{Expr, Literals},
    stmt::Stmt,
    token::Token,
    token_type::TokenType, printer::Print,
};
#[derive(Debug)]
pub struct RuntimeError<'a> {
    token: Rc<Token<'a>>,
    message: String,
}
impl<'a> RuntimeError<'a> {
    pub fn new(token: Rc<Token<'a>>, message: String) -> Self {
        Self { token, message }
    }
}

pub struct Interpreter<'p, T: Print> {
    pub environment: Environment,
    pub printer: &'p T
}

impl<'p, T: Print> Interpreter<'p, T> {
    pub fn new(printer: &'p T) -> Self {
        let mut interpreter = Self {
            environment: Environment::default(),
            printer
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
            ))),
        );

        // Define native to_string() function to convert any Lox datatype
        // into its string representation.
        interpreter.environment.define(
            "to_string".to_string(),
            Some(Literals::Function(Callable::new_native_fn(
                Rc::new(
                    |args| {
                        let literal = &args[0];
                        match literal {
                            Literals::String(_) => literal.clone(),
                            Literals::Number(n) => Literals::String(n.to_string()),
                            Literals::Bool(b) => Literals::String(b.to_string()),
                            Literals::Nil => Literals::String("Nil".to_string()),
                            Literals::Function(_) => Literals::String("<fn>".to_string()),
                        }
                    }
                ),
                1,
            ))),
        );

        interpreter
    }

    pub fn interpret<'a>(
        &mut self,
        statements: &Vec<Stmt<'a>>,
        err_reporter: &ErrorReporter<T>,
        declaration_refs: &mut Vec<Stmt<'a>>,
    ) -> i32 {
        for statement in statements {
            let result = self.execute(statement, declaration_refs);
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

    /// - statement: Stmt to be executed.
    /// - stmt_idx: Index of current statement to be executed in AST. This is needed to
    /// declare functions. stmt_idx for function declaration is stored in Literal::Function
    /// in Environment
    /// - ast: Complete AST. This is needed to evaluate non native functions call expr.
    /// stmt_idx stored in Literal::Function is used to fetch func declaration from "ast"
    /// and is then executed.
    fn execute<'b>(
        &mut self,
        statement: &Stmt<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Option<Literals>, RuntimeError<'b>> {
        match statement {
            Stmt::Expression(expr) => {
                self.evaluate(expr, declaration_refs)?;
                Ok(None)
            },

            Stmt::Print(expr) => {
                self.execute_print_stmt(expr, declaration_refs)?;
                Ok(None)
            }

            Stmt::Var(name, initializer) => {
                self.execute_var_declaration_stmt(
                    name.clone(),
                    initializer.as_ref(),
                    declaration_refs,
                )?;
                Ok(None)
            },

            Stmt::Block(stmts) => self.execute_block(stmts, declaration_refs),

            Stmt::If(condition, then_stmt, else_stmt) => {
                self.execute_if_stmt(condition, then_stmt, else_stmt, declaration_refs)
            }

            Stmt::While(condition, body) => {
                self.execute_while_statement(condition, body, declaration_refs)
            }

            Stmt::Function {
                name,
                params,
                body: _,
            } => {
                self.execute_fun_declaration_stmt(
                    name.clone(),
                    statement,
                    params.len(),
                    declaration_refs,
                );
                Ok(None)
            }
            Stmt::Return {
                return_keyword,
                expression,
            } => {
                Ok(Some(self.execute_return_stmt(return_keyword.clone(), expression, declaration_refs)?))
            }
        }
    }

    fn evaluate<'b>(
        &mut self,
        expr: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        match expr {
            Expr::Binary(left, op, right) => {
                self.interpret_binary(op.clone(), left, right, declaration_refs)
            }
            Expr::Grouping(grp) => self.interpret_group(grp, declaration_refs),
            // TODO: Avoid cloning especially for String
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Unary(op, right) => self.interpret_unary(op.clone(), right, declaration_refs),
            Expr::Variable(variable) => self.interpret_variable(variable.clone()),
            Expr::Assign(var_name, rvalue) => {
                self.execute_assign_expr(var_name.clone(), rvalue, declaration_refs)
            }
            Expr::Logical(left, op, right) => {
                self.interpret_logical(op.clone(), left, right, declaration_refs)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => self.interpret_call(callee, paren, arguments, declaration_refs),
        }
    }

    fn interpret_unary<'b>(
        &mut self,
        op: Rc<Token<'b>>,
        right: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let right = self.evaluate(right, declaration_refs)?;
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

    fn interpret_group<'b>(
        &mut self,
        expr: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        self.evaluate(expr, declaration_refs)
    }

    fn interpret_variable<'b>(&mut self, var: Rc<Token<'b>>) -> Result<Literals, RuntimeError<'b>> {
        self.environment.get(var)
    }

    fn interpret_binary<'b>(
        &mut self,
        op: Rc<Token<'b>>,
        left: &Expr<'b>,
        right: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let left = self.evaluate(left, declaration_refs)?;
        let right = self.evaluate(right, declaration_refs)?;

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

    fn interpret_logical<'b>(
        &mut self,
        op: Rc<Token<'b>>,
        left: &Expr<'b>,
        right: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let left = self.evaluate(left, declaration_refs)?;
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
        self.evaluate(right, declaration_refs)
    }

    fn interpret_call<'b>(
        &mut self,
        callee: &Expr<'b>,
        paren: &Rc<Token<'b>>,
        args: &[Expr<'b>],
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let callee = self.evaluate(callee, declaration_refs)?;
        let mut arguments = vec![];
        for arg in args {
            arguments.push(self.evaluate(arg, declaration_refs)?);
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
            match function.call(self, declaration_refs, arguments) {
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

    pub fn execute_block<'b>(
        &mut self,
        stmts: &[Stmt<'b>],
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Option<Literals>, RuntimeError<'b>> {
        let mut return_value = None;
        self.environment.create_new_scope();
        for stmt in stmts {
            // if let Stmt::Return {
            //     return_keyword, expression
            // } = stmt {
            //     return Ok(Some(
            //         self.evaluate(
            //             expression.as_ref().unwrap_or(&Literals::Nil.into()),
            //             declaration_refs
            //         )?
            //     ));
            // } else {
            // }
            return_value = self.execute(stmt, declaration_refs)?;
            if return_value.is_some() {
                break;
            }
        }
        self.environment.end_latest_scope();
        Ok(return_value)
    }

    fn execute_if_stmt<'b>(
        &mut self,
        condition: &Expr<'b>,
        then_stmt: &Stmt<'b>,
        else_statement: &Option<Stmt<'b>>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Option<Literals>, RuntimeError<'b>> {
        if Self::into_bool(&self.evaluate(condition, declaration_refs)?) {
            return self.execute(then_stmt, declaration_refs);
        } else if let Some(else_stmt) = else_statement {
            return self.execute(else_stmt, declaration_refs);
        }
        Ok(None)
    }

    fn execute_print_stmt<'b>(
        &mut self,
        expr: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<(), RuntimeError<'b>> {
        let value = self.evaluate(expr, declaration_refs)?;
        match value {
            Literals::Nil => self.printer.print(&"Nil"),
            Literals::String(s) => self.printer.print(&s),
            Literals::Number(n) => self.printer.print(&n),
            Literals::Bool(b) => self.printer.print(&b),
            Literals::Function(_) => self.printer.print(&"<fn>"),
        }
        Ok(())
    }

    fn execute_while_statement<'b>(
        &mut self,
        condition: &Expr<'b>,
        body: &Stmt<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Option<Literals>, RuntimeError<'b>> {
        while Self::into_bool(&self.evaluate(condition, declaration_refs)?) {
            let return_value = self.execute(body, declaration_refs)?;
            if return_value.is_some() {
                return Ok(return_value);
            }
        }
        Ok(None)
    }

    fn execute_var_declaration_stmt<'b>(
        &mut self,
        name: Rc<Token>,
        expr: Option<&Expr<'b>>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<(), RuntimeError<'b>> {
        let value = if expr.is_some() {
            Some(self.evaluate(expr.unwrap(), declaration_refs)?)
        } else {
            None
        };
        self.environment.define(name.lexeme.to_string(), value);
        Ok(())
    }

    fn execute_fun_declaration_stmt<'a>(
        &mut self,
        name: Rc<Token<'a>>,
        stmt: &Stmt<'a>,
        arity: usize,
        declaration_refs: &mut Vec<Stmt<'a>>,
    ) {
        declaration_refs.push(stmt.clone());
        self.environment.define(
            name.lexeme.to_string(),
            Some(Literals::Function(
                // len() - 1 is the index of function declaration just pushed
                // in declaration_refs. To be later used when evaluating call expr.
                Callable::new_foreign_fn(declaration_refs.len() - 1, arity),
            )),
        );
    }

    fn execute_return_stmt<'b>(
        &mut self,
        _return_keyword: Rc<Token<'b>>,
        expression: &Option<Expr<'b>>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        self.evaluate(
            expression.as_ref().unwrap_or(&Literals::Nil.into()),
            declaration_refs,
        )
    }

    fn execute_assign_expr<'b>(
        &mut self,
        name: Rc<Token<'b>>,
        expr: &Expr<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let value = self.evaluate(expr, declaration_refs)?;
        self.environment.assign(name, value)
    }
}

#[cfg(test)]
mod test {
    use crate::lox::{
        error_reporter::ErrorReporter, expr::Literals, parser::Parser, scanner::Scanner, stmt::Stmt, printer::TestPrinter,
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
        let mut declaration_refs: Vec<Stmt> = vec![];
        assert_eq!(
            interpreter.evaluate(ast, &mut declaration_refs).unwrap(),
            Literals::Number(2.0)
        );
    }
}
