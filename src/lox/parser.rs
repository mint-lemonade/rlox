use std::{cell::Cell, rc::Rc, vec};

use crate::lox::expr::Literals;

use super::{expr::Expr, token::Token, token_type::TokenType, error_reporter::ErrorReporter, stmt::Stmt, printer::Print};

struct LoxParseError;
pub struct Parser<'a, T: Print> {
    tokens: &'a Vec<Rc<Token>>,
    current: Cell<usize>,
    err_reporter: &'a ErrorReporter<'a, T>
}

impl<'a, T: Print> Parser<'a, T> {

    pub fn new(tokens: &'a Vec<Rc<Token>>, err_reporter: &'a ErrorReporter<'a, T>) -> Self {
        Self {
            tokens,
            current: Cell::new(0),
            err_reporter
        }
    }

    pub fn parse(&self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt)
            }
        }
        statements
    }

    fn declaration(&self) -> Option<Stmt> {
        let stmt = if self.r#match([TokenType::Fun]) {
            self.function_declaration("function")
        } else if self.r#match([TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        
        if stmt.is_err() {
            self.synchronize();
        }
        stmt.ok()
    }

    fn var_declaration(&self) -> Result<Stmt, LoxParseError> {
        let name = self.consume(
            TokenType::Identifier, "Expected variable name."
        )?;
        let mut initilizer= None;
        if self.r#match([TokenType::Equal]) {
            // match self.expression() {
            //     Ok(expr) => initilizer = Some(expr),
            //     Err(e) => return Err(e),
            // };
            initilizer = Some(self.expression()?)
        }
        self.consume(
            TokenType::SemiColon, "Expected ';' after variable declaration"
        )?;
        // Err(LoxParseError)
        Ok(Stmt::Var(name, initilizer))

    }

    fn statement(&self) -> Result<Stmt, LoxParseError> {
        if self.r#match([TokenType::For]) {
            return self.for_statement();
        }
        if self.r#match([TokenType::If]) {
            return self.if_statement();
        }
        if self.r#match([TokenType::Print]) {
            return self.print_statement();
        }
        if self.r#match([TokenType::Return]) {
            return self.return_statement();
        }
        if self.r#match([TokenType::While]) {
            return self.while_statement();
        }
        if self.r#match([TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expression_statement()
    }

    fn for_statement(&self) -> Result<Stmt, LoxParseError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'")?;

        let initializer;
        if self.r#match([TokenType::SemiColon]) {
            initializer = None;
        } else if self.r#match([TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(&TokenType::SemiColon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::SemiColon, "Expected ';' after loop condition")?;

        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expected ')' after for clause")?;

        let mut body = self.statement()?;

        // 
        // Desugar into While statement
        // 
        if let Some(increment) =  increment {
            body = Stmt::Block(vec![
                body,
                Stmt::Expression(increment)
            ])
        }

        let condition = condition.unwrap_or(Expr::Literal(Literals::Bool(true)));
        body = Stmt::While(condition, Box::new(body));

        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![
                initializer,
                body
            ])
        }

        Ok(body)
    }

    fn if_statement(&self) -> Result<Stmt, LoxParseError> {
        self.consume(TokenType::LeftParen, "Expected '(' after If")?;
        let condtion = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after If")?;
        let then_stmt = self.statement()?;
        let mut else_statement = Box::new(None);
        if self.r#match([TokenType::Else]) {
            else_statement = Box::new(Some(self.statement()?));
        }
        Ok(Stmt::If(condtion, Box::new(then_stmt), else_statement))
    }

    fn print_statement(&self) -> Result<Stmt, LoxParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value")?;
        Ok(Stmt::Print(expr))
    }

    fn return_statement(&self) -> Result<Stmt, LoxParseError> {
        let keyword = self.previous();
        let mut expr = None;
        if !self.check(&TokenType::SemiColon) {
            expr = Some(self.expression()?);
        }
        self.consume(TokenType::SemiColon, "Expect ';' after value")?;
        Ok(Stmt::Return { return_keyword: keyword, expression: expr })
    }

    fn while_statement(&self) -> Result<Stmt, LoxParseError> {
        self.consume(TokenType::LeftParen, "Expected '(' after While")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after While")?;
        let body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn block(&self) -> Result<Vec<Stmt>, LoxParseError> {
        let mut statements = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_at_end()  {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    fn expression_statement(&self) -> Result<Stmt, LoxParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value")?;
        Ok(Stmt::Expression(expr))
    }

    fn function_declaration(&self, kind: &str) -> Result<Stmt, LoxParseError> {
        let name = self.consume(TokenType::Identifier, format!("Expected {kind} name").as_str())?;
        self.consume(TokenType::LeftParen, format!("Expected '(' after {kind} name").as_str())?;
        let mut params = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() > 255 {
                    self.err_reporter.error_token(self.peek(), "Can't have more than 255 parameters");
                    return Err(LoxParseError);
                }
                params.push(self.consume(TokenType::Identifier, "Expected parameter name")?);
                if !self.r#match([TokenType::Comma])  { 
                    break 
                };
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenType::LeftBrace, format!("Expected '{{' before {kind} body").as_str())?;
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn expression(&self) -> Result<Expr, LoxParseError> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr, LoxParseError> {
        let expr = self.or()?;
        if self.r#match([TokenType::Equal]) {
            let equals = self.previous();
            let value = self.or()?;
            if let Expr::Variable(var_name) = expr {
                return Ok(Expr::Assign(var_name, Box::new(value)));
            } else {
                self.err_reporter.error_token(equals, "Invalid assignment target")
            }
        }
        Ok(expr)
    }

    fn or(&self) -> Result<Expr, LoxParseError> {
        let left = self.and()?;
        if self.r#match([TokenType::Or]) {
            let op = self.previous();
            let right = self.equality()?;
            return Ok(Expr::Logical(Box::new(left), op, Box::new(right)));
        }
        Ok(left)
    }

    fn and(&self) ->  Result<Expr, LoxParseError> {
        let left = self.equality()?;
        if self.r#match([TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            return Ok(Expr::Logical(Box::new(left), op, Box::new(right)));
        }
        Ok(left)
    }


    fn equality(&self) -> Result<Expr, LoxParseError> {
        let mut expr = self.comparison()?;
        while self.r#match([TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }
        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, LoxParseError> {
        let mut expr = self.term()?;
        while self.r#match([TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }
        Ok(expr)
    }

    fn term(&self) -> Result<Expr, LoxParseError> {
        let mut expr = self.factor()?;
        while self.r#match([TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }
        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, LoxParseError> {
        let mut expr = self.unary()?;
        while self.r#match([TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(
                Box::new(expr),
                op,
                Box::new(right),
            );
        }
        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, LoxParseError> {
        if self.r#match([TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }
        self.call()
    }

    fn call(&self) -> Result<Expr, LoxParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.r#match([TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// This function takes parsed 'callee' expr and parses argument list and closing
    /// paranthesis after the args list and returns function expression (Expr::Call)
    /// containing  callee and args list.
    fn finish_call(&'a self, callee: Expr) -> Result<Expr, LoxParseError> {
        let mut arguments = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if !self.r#match([TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments list")?;
        Ok(Expr::Call{ callee: Box::new(callee), paren, arguments })

    }

    fn primary(&self) -> Result<Expr, LoxParseError> {

        if self.r#match([TokenType::False]) {
            return Ok(Expr::Literal(Literals::Bool(false)));
        }
        if self.r#match([TokenType::True]) {
            return Ok(Expr::Literal(Literals::Bool(true)));
        }
        if self.r#match([TokenType::Nil]) {
            return Ok(Expr::Literal(Literals::Nil))
        }

        if self.r#match([TokenType::Number(0.)]) {
            let TokenType::Number(n) = self.previous().token_type else {unreachable!()};
            return Ok(Expr::Literal(Literals::Number(n)));
        }
        if self.r#match([TokenType::String("".to_string())]) {
            let TokenType::String(s) = self.previous().token_type.clone() else {unreachable!()};
            return Ok(Expr::Literal(Literals::String(s)));
        }

        if self.r#match([TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()))
        }

        if self.r#match([TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        // unimplemented!("Incorerect syntax or currently not implemented")
        self.err_reporter.error_token(self.previous(), "Expected Expression");
        Err(LoxParseError)
    }

    fn r#match<const N: usize>(&self, token_types: [TokenType; N]) -> bool {
        let matched = token_types.iter().any(|tkn_type| self.check(tkn_type));
        if matched {
            self.advance();
        }
        matched
    }

    fn consume(
        &self, tkn_type: TokenType, mssg: &str
    ) -> Result<Rc<Token>, LoxParseError>  {
        if self.check(&tkn_type) {
            return Ok(self.advance());
        }
        self.err_reporter.error_token(self.previous(), mssg);
        Err(LoxParseError)
    }

    fn check(&self, tkn_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(tkn_type)
    }

    fn advance(&self) -> Rc<Token> {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current.get()].token_type == TokenType::Eof
    }

    fn peek(&self) -> Rc<Token> {
        self.tokens[self.current.get()].clone()
    }

    fn previous(&self) -> Rc<Token> {
        self.tokens[self.current.get() - 1].clone()
    }

    fn synchronize(&self) {
        self.advance();
    
        while !self.is_at_end() {
          if self.previous().token_type == TokenType::SemiColon { return ; };
    
          match self.peek().token_type {
            TokenType::Class | TokenType::Fun | TokenType::Var |
            TokenType::For | TokenType::If | TokenType::While |
            TokenType::Print | TokenType::Return => { return ; },
            _ => ()
          };
    
          self.advance();
        }
      }
}

#[cfg(test)]
mod test {
    use crate::lox::{error_reporter::ErrorReporter, scanner::Scanner, printer::{pretty_to_string, TestPrinter}, stmt::Stmt};

    use super::Parser;

    #[test]
    fn parsed_ast_print(){
        let source = "(5 - (3.7 - 1)) + -1.2;";
        let printer = TestPrinter::default();
        let error_reporter = ErrorReporter::new(
            source, false, &printer
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let Stmt::Expression(ast) = &parser.parse()[0] else {panic!()};
        assert_eq!(pretty_to_string(ast), "(+ (group (- 5 (group (- 3.7 1)))) (- 1.2))");
    }
}