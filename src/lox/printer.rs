use std::{fmt::Display, cell::RefCell, any::Any};
use super::expr::{Expr, Literals};

pub trait Print: Any {
    fn print(&self, subject: &dyn Display);
}

pub struct CliPrinter;
impl Default for CliPrinter {
    fn default() -> Self {
        Self
    }
}
impl Print for CliPrinter {
    fn print(&self, subject: &dyn Display) {
        println!("{}", subject);
    }
}

pub struct TestPrinter {
    pub result: RefCell<Vec<String>>
}
impl Default for TestPrinter {
    fn default() -> Self {
        Self {
            result: RefCell::new(vec![])
        }
    }
}
impl Print for TestPrinter {
    fn print(&self, subject: &dyn Display) {
        self.result.borrow_mut().push(subject.to_string())
    }
}

#[allow(dead_code)]
pub fn pretty_print(expression: &Expr) {
    println!("{}", parenthesize(expression));
}

#[allow(dead_code)]
pub fn pretty_to_string(expression: &Expr) -> String{
    parenthesize(expression)
}

fn parenthesize(expression: &Expr) -> String {
    match expression {
        Expr::Binary(left, op, right) | Expr::Logical(left, op, right) => {
            format!(
                "({} {} {})",
                op.lexeme,
                parenthesize(left),
                parenthesize(right)
            )
        }
        Expr::Grouping(expression) => {
            format!("(group {})", parenthesize(expression))
        }
        // Exp
        Expr::Literal(Literals::String(s)) => s.to_string(),
        Expr::Literal(Literals::Number(n)) => n.to_string(),
        Expr::Literal(Literals::Bool(b)) => b.to_string(),
        Expr::Literal(Literals::Nil) => "NIL".to_string(),
        Expr::Literal(Literals::Function(_)) => "<Function>".to_string(),
        Expr::Unary(op, right) => {
            format!("({} {})", op.lexeme, parenthesize(right))
        }
        Expr::Variable(var_name) => format!("(Var {})", var_name.lexeme),
        Expr::Assign(op, expr) => format!("({} {})", op.lexeme, parenthesize(expr)),
        Expr::Call { callee: _, paren: _, arguments: _ } => todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::lox::{printer::parenthesize, expr::{Expr, Literals}, token::Token, token_type::TokenType};

    #[test]
    fn pretty_print_expression() {
        let t_minus = Rc::new(Token::new(TokenType::Minus, "-", 1));
        let t_star = Rc::new(Token::new(TokenType::Star, "*", 1));
        let ex = Expr::Binary(
            Box::new(Expr::Unary(t_minus, Box::new(Literals::Number(123.0).into()))),
            t_star,
            Box::new(Expr::Grouping(Box::new(Literals::Number(45.67).into()))),
        );
        assert_eq!("(* (- 123) (group 45.67))", parenthesize(&ex));
    }
}
