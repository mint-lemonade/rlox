use std::{cell::Cell, rc::Rc};

use super::{expr::Expr, token::Token, token_type::TokenType, error_reporter::ErrorReporter};

pub struct Parser<'a> {
    tokens: &'a Vec<Rc<Token<'a>>>,
    current: Cell<usize>,
    err_reporter: &'a ErrorReporter<'a>
}

impl<'a> Parser<'a> {

    pub fn new(tokens: &'a Vec<Rc<Token>>, err_reporter: &'a ErrorReporter<'a>) -> Self {
        Self {
            tokens,
            current: Cell::new(0),
            err_reporter
        }
    }

    pub fn parse(&self) -> Option<Expr<'a>> {
        self.expression()
    }

    fn expression(&self) -> Option<Expr<'a>> {
        self.equality()
    }

    fn equality(&self) -> Option<Expr<'a>> {
        let mut expr = self.comparison();
        while self.r#match([TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                op,
                Box::new(right.unwrap()),
            ));
        }
        expr
    }

    fn comparison(&self) -> Option<Expr<'a>> {
        let mut expr = self.term();
        while self.r#match([TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous();
            let right = self.term();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                op,
                Box::new(right.unwrap()),
            ));
        }
        expr
    }

    fn term(&self) -> Option<Expr<'a>> {
        let mut expr = self.factor();
        while self.r#match([TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                op,
                Box::new(right.unwrap()),
            ));
        }
        expr
    }

    fn factor(&self) -> Option<Expr<'a>> {
        let mut expr = self.unary();
        while self.r#match([TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                op,
                Box::new(right.unwrap()),
            ));
        }
        expr
    }

    fn unary(&self) -> Option<Expr<'a>> {
        if self.r#match([TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Some(Expr::Unary(op, Box::new(right.unwrap())));
        }
        Some(self.primary().unwrap())
    }

    fn primary(&self) -> Option<Expr<'a>> {

        if self.r#match([TokenType::False]) {
            return Some(Expr::BoolLiteral(false));
        }
        if self.r#match([TokenType::True]) {
            return Some(Expr::BoolLiteral(true));
        }
        if self.r#match([TokenType::Nil]) {
            return Some(Expr::NilLiteral)
        }

        if self.r#match([TokenType::Number(0.)]) {
            let TokenType::Number(n) = self.previous().token_type else {unreachable!()};
            return Some(Expr::NumberLiteral(n));
        }
        if self.r#match([TokenType::String("".to_string())]) {
            let TokenType::String(s) = self.previous().token_type.clone() else {unreachable!()};
            return Some(Expr::StringLiteral(s));
        }

        if self.r#match([TokenType::LeftParen]) {
            let expr = self.expression();
            // ignnore Result for now. Handle later when implementing error recovery.
            let _ =self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Some(Expr::Grouping(Box::new(expr.unwrap())));
        }
        unimplemented!("Incorerect syntax or currently not implemented")
    }

    fn r#match<const N: usize>(&self, token_types: [TokenType; N]) -> bool {
        let matched = token_types.iter().any(|tkn_type| self.check(tkn_type));
        if matched {
            self.advance();
        }
        matched
    }

    fn consume<'b>(&self, tkn_type: TokenType, mssg: &'b str) -> Result<(), &'b str>  {
        if self.check(&tkn_type) {
            self.advance();
            // return ();
            return Ok(());
        }
        self.err_reporter.error_token(self.previous(), mssg);
        Err(mssg)
    }

    fn check(&self, tkn_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(tkn_type)
    }

    fn advance(&self) -> Rc<Token<'a>> {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current.get()].token_type == TokenType::Eof
    }

    fn peek(&self) -> Rc<Token<'a>> {
        self.tokens[self.current.get()].clone()
    }

    fn previous(&self) -> Rc<Token<'a>> {
        self.tokens[self.current.get() - 1].clone()
    }
}

#[cfg(test)]
mod test {
    use crate::lox::{error_reporter::ErrorReporter, scanner::Scanner, ast_printer::{pretty_print, pretty_to_string}};

    use super::Parser;

    #[test]
    fn parsed_ast_print(){
        let source = "(5 - (3.7 - 1)) + -1.2";
        let error_reporter = ErrorReporter::new(
            source, false
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        assert_eq!(pretty_to_string(ast.as_ref().unwrap()), "(+ (group (- 5 (group (- 3.7 1)))) (- 1.2))");
    }
}