use std::{rc::Rc, time::SystemTime, usize::MAX};

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
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            environment: Environment::new(),
        };

        // Define native function "clock()" to return current time in secs
        interpreter.environment.define("clock".to_string(), Some(
            Literals::Function(Callable::new_native_fn(Rc::new(|_args| {
                match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(d) => Literals::Number(d.as_secs_f64()),
                    Err(_) => Literals::Nil,
                }
            }), 0))
        ));

        interpreter
    }

    pub fn interpret<'a>(
        &mut self, statements: &Vec<Stmt<'a>>, 
        err_reporter: &ErrorReporter, declaration_refs: &mut Vec<Stmt<'a>>
    ) {
        for (stmt_idx, statement) in statements.iter().enumerate() {
            let result = self.execute(
                statement, declaration_refs
            );
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

    /// - statement: Stmt to be executed.
    /// - stmt_idx: Index of current statement to be executed in AST. This is needed to 
    /// declare functions. stmt_idx for function declaration is stored in Literal::Function
    /// in Environment
    /// - ast: Complete AST. This is needed to evaluate non native functions call expr.
    /// stmt_idx stored in Literal::Function is used to fetch func declaration from "ast"
    /// and is then executed.
    fn execute<'b>(
        &mut self, statement: &Stmt<'b>, declaration_refs: &mut Vec<Stmt<'b>>
    ) -> Result<(), RuntimeError<'b>> {
        match statement {
            Stmt::Expression(expr) => {
                self.evaluate(expr, Some(declaration_refs))?;
                Ok(())
            }

            Stmt::Print(expr) => self.execute_print_stmt(expr),

            Stmt::Var(name, initializer) => {
                self.execute_var_declaration_stmt(name.clone(), initializer.as_ref())
            }

            Stmt::Block(stmts) => {
                self.execute_block(stmts, declaration_refs)?;
                Ok(())
            },

            Stmt::If(
                condition, then_stmt, else_stmt
            ) => self.execute_if_stmt(condition, then_stmt, else_stmt, declaration_refs),
            
            Stmt::While(condition, body) => self.execute_while_statement(condition, body, declaration_refs),
            
            Stmt::Function { name, params, body: _ } => {
                self.execute_fun_declaration_stmt(
                    name.clone(), statement, params.len(), declaration_refs
                );
                Ok(())
            },
        }
    }

    fn evaluate<'b>(&mut self, expr: &Expr<'b>, declaration_refs: Option<&mut Vec<Stmt<'b>>>) -> Result<Literals, RuntimeError<'b>> {
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
            } => self.interpret_call(callee, paren, arguments, declaration_refs),
        }
    }

    fn interpret_unary<'b>(
        &mut self,
        op: Rc<Token<'b>>,
        right: &Expr<'b>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let right = self.evaluate(right, None)?;
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
        self.evaluate(expr, None)
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
        let left = self.evaluate(left, None)?;
        let right = self.evaluate(right, None)?;

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
        let left = self.evaluate(left, None)?;
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
        self.evaluate(right, None)
    }

    fn interpret_call<'b>(
        &mut self,
        callee: &Expr<'b>,
        paren: &Rc<Token<'b>>,
        args: &[Expr<'b>],
        declaration_refs: Option<&mut Vec<Stmt<'b>>>
    ) -> Result<Literals, RuntimeError<'b>> {
        let callee = self.evaluate(callee, None)?;
        let mut arguments = vec![];
        for arg in args {
            arguments.push(self.evaluate(arg, None)?);
        }
        if let Literals::Function(Callable::Native(function)) = callee {
            if arguments.len() != function.arity {
                return Err(RuntimeError::new(paren.clone(), format!("Expected {} arguments, recieved {}", function.arity, arguments.len())));
            }
            Ok((function.call)(arguments))
        } else if let Literals::Function(Callable::Foreign(function)) = callee {
            match function.call(self, declaration_refs.unwrap(), arguments) {
                Ok(return_val) => Ok(return_val),
                Err(err) => Err(RuntimeError::new(paren.clone(), err.message)),
            }
        } else {
            Err(RuntimeError::new(paren.clone(), "Can only call functions and classes".to_string()))
        }
    }

    pub fn execute_block<'b>(&mut self, stmts: &Vec<Stmt<'b>>, declaration_refs: &mut Vec<Stmt<'b>>) -> Result<Option<Literals>, RuntimeError<'b>> {
        self.environment.create_new_scope();
        for (stmt_idx, stmt) in stmts.iter().enumerate() {
            if let Stmt::Return { 
                return_keyword, expression 
            } = stmt {
                return Ok(Some(
                    self.evaluate(
                        expression.as_ref().unwrap_or(&Literals::Nil.into()), 
                        Some(declaration_refs)
                    )?
                ));
            } else {
                self.execute(stmt, declaration_refs)?;
            }
        }
        self.environment.end_latest_scope();
        Ok(None)
    }

    fn execute_if_stmt<'b>(
        &mut self, condition: &Expr<'b>, 
        then_stmt: &Stmt<'b>, else_statement: &Option<Stmt<'b>>,
        declaration_refs: &mut Vec<Stmt<'b>>
    ) -> Result<(), RuntimeError<'b>> {
        if Self::into_bool(&self.evaluate(condition, None)?) {
            self.execute(then_stmt, declaration_refs)?;
        } else if let Some(else_stmt) = else_statement {
            self.execute(else_stmt, declaration_refs)?;
        }
        Ok(())
    }

    fn execute_print_stmt<'b>(&mut self, expr: &Expr<'b>) -> Result<(), RuntimeError<'b>> {
        let value = self.evaluate(expr, None)?;
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
        &mut self, condition: &Expr<'b>, body: &Stmt<'b>,
        declaration_refs: &mut Vec<Stmt<'b>>
    ) -> Result<(), RuntimeError<'b>> {
        while Self::into_bool(&self.evaluate(condition,  None)?) {
            self.execute(body, declaration_refs)?;
        }
        Ok(())
    }

    fn execute_var_declaration_stmt<'b>(
        &mut self,
        name: Rc<Token>,
        expr: Option<&Expr<'b>>,
    ) -> Result<(), RuntimeError<'b>> {
        let value = if expr.is_some() {
            Some(self.evaluate(expr.unwrap(), None)?)
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
        declaration_refs: &mut Vec<Stmt<'a>>
    ) {
        declaration_refs.push(stmt.clone());
        self.environment.define(
            name.lexeme.to_string(),
            Some(Literals::Function(
                // len() - 1 is the index of function declaration just pushed
                // in declaration_refs. To be later used when evaluating call expr.
                Callable::new_foreign_fn(declaration_refs.len() - 1, arity)
            ))
        );
    }

    fn execute_assign_expr<'b>(
        &mut self,
        name: Rc<Token<'b>>,
        expr: &Expr<'b>,
    ) -> Result<Literals, RuntimeError<'b>> {
        let value = self.evaluate(expr, None)?;
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
        assert_eq!(interpreter.evaluate(ast, None).unwrap(), Literals::Number(2.0));
    }
}
