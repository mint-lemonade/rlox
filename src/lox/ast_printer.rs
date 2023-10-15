use super::expr::{Expr, Literals};

pub fn pretty_print(expression: &Expr) {
    println!("{}", parenthesize(expression));
}

pub fn pretty_to_string(expression: &Expr) -> String{
    parenthesize(expression)
}

fn parenthesize(expression: &Expr) -> String {
    match expression {
        Expr::Binary(left, op, right) => {
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
        Expr::Unary(op, right) => {
            format!("({} {})", op.lexeme, parenthesize(right))
        }
        Expr::Variable(_) => todo!(),
        Expr::Assign(_, _) => todo!(),
        Expr::Logical(_, _, _) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::lox::{ast_printer::parenthesize, expr::{Expr, Literals}, token::Token, token_type::TokenType};

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
