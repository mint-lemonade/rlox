use super::expr::Expr;

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
        Expr::StringLiteral(s) => s.to_string(),
        Expr::NumberLiteral(n) => n.to_string(),
        Expr::BoolLiteral(b) => b.to_string(),
        Expr::NilLiteral => "NIL".to_string(),
        Expr::Unary(op, right) => {
            format!("({} {})", op.lexeme, parenthesize(right))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::lox::{ast_printer::parenthesize, expr::Expr, token::Token, token_type::TokenType};

    #[test]
    fn pretty_print_expression() {
        let t_minus = Rc::new(Token::new(TokenType::Minus, "-", 1));
        let t_star = Rc::new(Token::new(TokenType::Star, "*", 1));
        let ex = Expr::Binary(
            Box::new(Expr::Unary(t_minus, Box::new(Expr::NumberLiteral(123.0)))),
            t_star,
            Box::new(Expr::Grouping(Box::new(Expr::NumberLiteral(45.67)))),
        );
        assert_eq!("(* (- 123) (group 45.67))", parenthesize(&ex));
    }
}
