use std::{rc::Rc, time::SystemTime};

use super::{
    environment::Environment,
    error_reporter::ErrorReporter,
    expr::{Expr, Literals},
    stmt::Stmt,
    token::Token,
    token_type::TokenType, callable::Callable,
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

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            environment: Environment::new(),
        };

        // Define native function "clock()" to return current time in secs
        interpreter.environment.define("clock".to_string(), Some(
            Literals::Function(Callable::new(Rc::new(|_args| {
                match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(d) => Literals::Number(d.as_secs_f64()),
                    Err(_) => Literals::Nil,
                }
            }), 0))
        ));

        interpreter
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>, err_reporter: &ErrorReporter) {
        for statement in statements {
            let result = self.execute(statement);
            match result {
                Ok(_) => (),
                Err(e) => {
                    err_reporter.runtime_error(e.token, e.message);
                    // TODO Do not panic. 
                    // Return error for exit to be handled gracefully in main.rs
                    panic!();
                },
            }
        }
    }

    fn execute<'b>(&mut self, statement: &Stmt<'b>) -> Result<(), RuntimeError<'b>> {
        match statement {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }

            Stmt::Print(expr) => self.execute_print_stmt(expr),

            Stmt::Var(name, initializer) => {
                self.execute_var_declaration_stmt(name.clone(), initializer.as_ref())
            }

            Stmt::Block(stmts) => self.execute_block(stmts),

            Stmt::If(
                condition, then_stmt, else_stmt
            ) => self.execute_if_stmt(condition, then_stmt, else_stmt),
            
            Stmt::While(condition, body) => self.execute_while_statement(condition, body),
            Stmt::Function { name, params, body } => {
                dbg!(name);
                dbg!(params);
                dbg!(body);
                todo!()
            },
        }
    }

    fn evaluate<'b>(&mut self, expr: &Expr<'b>) -> Result<Literals, RuntimeError<'b>> {
        match expr {
            Expr::Binary(left, op, right) => self.interpret_binary(op.clone(), left, right),
            Expr::Grouping(grp) => self.interpret_group(grp),
            // TODO: Avoid cloning especially for String
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Unary(op, right) => self.interpret_unary(op.clone(), right),
            Expr::Variable(variable) => self.interpret_variable(variable.clone()),
            Expr::Assign(var_name, rvalue) => self.execute_assign_expr(var_name.clone(), rvalue),
            Expr::Logical(left, op, right) => self.interpret_logical(op.clone(), left, right),
            Expr::Call { 
                callee, paren, arguments 
            } => self.interpret_call(callee, paren, arguments),
        }
    }

    fn interpret_unary<'b>(
        &mut self,
        op: Rc<Token<'b>>,
        right: &Expr<'b>,
    ) -> Result<Literals, RuntimeError<'b>> {
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

    fn interpret_group<'b>(&mut self, expr: &Expr<'b>) -> Result<Literals, RuntimeError<'b>> {
        self.evaluate(expr)
    }

    fn interpret_variable<'b>(&mut self, var: Rc<Token<'b>>) -> Result<Literals, RuntimeError<'b>> {
        self.environment.get(var)
    }
   
    fn interpret_binary<'b>(
        &mut self,
        op: Rc<Token<'b>>,
        left: &Expr<'b>,
        right: &Expr<'b>,
    ) -> Result<Literals, RuntimeError<'b>> {
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
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
            }
            TokenType::Slash => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a / b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
            }
            TokenType::Star => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a * b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
            }
            TokenType::Greater => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a > b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
            }
            TokenType::GreaterEqual => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a >= b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
            }
            TokenType::Less => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a < b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
            }
            TokenType::LessEqual => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a <= b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number.".to_string()))
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
    ) -> Result<Literals, RuntimeError<'b>> {
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
            _ => unreachable!()
        }
        self.evaluate(right)
    }

    fn interpret_call<'b>(
        &mut self,
        callee: &Expr<'b>,
        paren: &Rc<Token<'b>>,
        args: &[Expr<'b>],
    ) -> Result<Literals, RuntimeError<'b>> {
        let callee = self.evaluate(callee)?;
        let mut arguments = vec![];
        for arg in args {
            arguments.push(self.evaluate(arg)?);
        }
        if let Literals::Function(function) = callee {
            if arguments.len() != function.arity {
                return Err(RuntimeError::new(paren.clone(), format!("Expected {} arguments, recieved {}", function.arity, arguments.len())));
            }
            Ok((function.call)(arguments))
        } else {
            Err(RuntimeError::new(paren.clone(), "Can only call functions and classes".to_string()))
        }

        // todo!()
    }
    fn execute_block<'b>(&mut self, stmts: &Vec<Stmt<'b>>) -> Result<(), RuntimeError<'b>> {
        self.environment.create_new_scope();
        for stmt in stmts {
            self.execute(stmt)?;
        }
        self.environment.end_latest_scope();
        Ok(())
    }

    fn execute_if_stmt<'b>(
        &mut self, condition: &Expr<'b>, 
        then_stmt: &Stmt<'b>, else_statement: &Option<Stmt<'b>>
    ) -> Result<(), RuntimeError<'b>> {
        if Self::into_bool(&self.evaluate(condition)?) {
            self.execute(then_stmt)?;
        } else if let Some(else_stmt) = else_statement {
            self.execute(else_stmt)?;
        }
        Ok(())
    }

    fn execute_print_stmt<'b>(&mut self, expr: &Expr<'b>) -> Result<(), RuntimeError<'b>> {
        let value = self.evaluate(expr)?;
        match value {
            Literals::Nil => println!("Nil"),
            Literals::String(s) => println!("{}", s),
            Literals::Number(n) => println!("{}", n),
            Literals::Bool(b) => println!("{}", b),
            Literals::Function(f) => println!("<Function>"),
        }
        Ok(())
    }

    fn execute_while_statement<'b>(
        &mut self, condition: &Expr<'b>, body: &Stmt<'b>
    ) -> Result<(), RuntimeError<'b>> {
        while Self::into_bool(&self.evaluate(condition)?) {
            self.execute(body)?;
        }
        Ok(())
    }

    fn execute_var_declaration_stmt<'b>(
        &mut self,
        name: Rc<Token>,
        expr: Option<&Expr<'b>>,
    ) -> Result<(), RuntimeError<'b>> {
        let value = if expr.is_some() {
            Some(self.evaluate(expr.unwrap())?)
        } else {
            None
        };
        self.environment.define(name.lexeme.to_string(), value);
        Ok(())
    }

    fn execute_assign_expr<'b>(
        &mut self,
        name: Rc<Token<'b>>,
        expr: &Expr<'b>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let value = self.evaluate(expr)?;
        self.environment.assign(name, value)
    } 
}

#[cfg(test)]
mod test {
    use crate::lox::{
        error_reporter::ErrorReporter, expr::Literals, parser::Parser, scanner::Scanner, stmt::Stmt,
    };

    use super::Interpreter;

    #[test]
    fn direct_expression_evaluation() {
        let source = "(5 - (3 - 1)) + -1;";
        let error_reporter = ErrorReporter::new(source, false);

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
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.evaluate(ast).unwrap(), Literals::Number(2.0));
    }
}
