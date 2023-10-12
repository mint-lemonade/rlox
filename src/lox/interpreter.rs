use std::rc::Rc;

use super::{expr::{Expr, Literals}, error_reporter::ErrorReporter, token::Token, token_type::TokenType, stmt::Stmt};
#[derive(Debug)]
pub struct RuntimeError<'a> { token: Rc<Token<'a>>, message: &'static str }
impl<'a> RuntimeError<'a> {
    fn new(token: Rc<Token<'a>>, message: &'static str) -> Self {
        Self {
            token,
            message
        }
    }
}

pub struct Interpreter;

impl<'b> Interpreter {
    pub fn interpret(&self, statements: &Vec<Stmt>, err_reporter: &ErrorReporter) {
        for statement in statements {
            let result = self.execute(statement); 
            match result {
                Ok(_) => (),
                Err(e) => err_reporter.runtime_error(e.token, e.message),
            }
        }
    }

    fn execute(&'b self, statement: &'b Stmt) -> Result<(), RuntimeError> {
        match statement {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            },
            Stmt::Print(expr) => self.execute_print_stmt(expr),
        }
    }

    fn evaluate(&'b self, expr: &'b Expr) -> Result<Literals, RuntimeError> {
        match expr {
            Expr::Binary(
                left, op, right
            ) => self.interpret_binary(op.clone(), left, right),
            Expr::Grouping(grp) => self.interpret_group(grp),
            // TODO: Avoid cloning especially for String
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Unary(
                op, right
            ) => self.interpret_unary(op.clone(), right),
        }
    }

    fn interpret_unary(
        &'b self, 
        op: Rc<Token<'b>>, 
        right: &'b Expr
    ) -> Result<Literals, RuntimeError> {
        let right = self.evaluate(right)?;
        match op.token_type {
            TokenType::Minus => {
                match right {
                    Literals::Number(n) => Ok(Literals::Number(-n)),
                    _ => Err(RuntimeError::new(op, "Operand must be number"))
                }
            },
            TokenType::Bang => {
                match right {
                    Literals::Bool(b) => Ok(Literals::Bool(!b)),
                    // Nil is falsey => !Nil is truthy
                    Literals::Nil => Ok(Literals::Bool(true)),
                    // everthing_else is truthy => !everything_else is falsey
                    _ => Ok(Literals::Bool(false))
                }
            }
            _ => unreachable!()
        }
    }

    fn interpret_group(
        &'b self,
        expr: &'b Expr
    ) -> Result<Literals, RuntimeError>{
        self.evaluate(expr)
    }

    fn interpret_binary(
        &'b self,
        op: Rc<Token<'b>>,
        left: &'b Expr,
        right: &'b Expr
    ) -> Result<Literals, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match op.token_type {
            TokenType::Plus => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a+b));
                    }
                }
                if let Literals::String(a) = left {
                    if let Literals::String(b) = right {
                        // a.push_str(&b);
                        return Ok(Literals::String(format!("{}{}", a, b)));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Both Operands must be either number or string."))
            },
            TokenType::Minus => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a-b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::Slash => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a/b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::Star => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Number(a*b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::Greater => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a>b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::GreaterEqual => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a>=b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::Less => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a<b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::LessEqual => {
                if let Literals::Number(a) = left {
                    if let Literals::Number(b) = right {
                        return Ok(Literals::Bool(a<=b));
                    }
                }
                Err(RuntimeError::new(op.clone(), "Operands must be number."))
            },
            TokenType::BangEqual => Ok(Literals::Bool(left != right)),
            TokenType::EqualEqual => Ok(Literals::Bool(left == right)),
            _ => unreachable!()
        }
    }

    fn execute_print_stmt(&'b self, expr: &'b Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        match value {
            Literals::Nil => println!("Nil"),
            Literals::String(s) => println!("{}", s),
            Literals::Number(n) => println!("{}", n),
            Literals::Bool(b) => println!("{}", b),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::lox::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser, expr::Literals, stmt::Stmt};

    use super::Interpreter;

    #[test]
    fn direct_expression_evaluation() {
        let source = "(5 - (3 - 1)) + -1;";
        let error_reporter = ErrorReporter::new(
            source, false
        );

        let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        if error_reporter.had_error.get() { panic!("Error while scanning.") ; }     

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let Stmt::Expression(ast) = &parser.parse()[0] else {panic!()};
        if error_reporter.had_error.get() { panic!("Error while parsing.") ; }
        let interpreter = Interpreter;
        assert_eq!(interpreter.evaluate(ast).unwrap(), Literals::Number(2.0));
    }
}