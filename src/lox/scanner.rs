use phf::phf_map;

use std::{iter::Peekable, str::Chars, rc::Rc};

use crate::lox::token_type::TokenType;

use super::{error_reporter::ErrorReporter, token::Token, printer::Print};

const KEYWORDS: phf::Map<&str, TokenType> = phf_map! {
    "and" =>    TokenType::And,
    "class" =>  TokenType::Class,
    "else" =>   TokenType::Else,
    "false" =>  TokenType::False,
    "for" =>    TokenType::For,
    "fun" =>    TokenType::Fun,
    "if" =>     TokenType::If,
    "nil" =>    TokenType::Nil,
    "or" =>     TokenType::Or,
    "print" =>  TokenType::Print,
    "return" => TokenType::Return,
    "super" =>  TokenType::Super,
    "this" =>   TokenType::This,
    "true" =>   TokenType::True,
    "var" =>    TokenType::Var,
    "while" =>  TokenType::While,
};
pub struct Scanner<'a, 'p, T: Print> {
    source: &'a str,
    source_iter: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Rc<Token>>,
    err_reporter: &'a ErrorReporter<'a, 'p, T>,
}

impl<'a, 'p, T: Print> Scanner<'a, 'p, T> {
    pub fn new(source: &'a str, err_reporter: &'a ErrorReporter<'a, 'p, T>) -> Self {
        Self {
            source,
            source_iter: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            err_reporter,
        }
    }

    pub fn scan_tokens(&mut self) {
        while self.source_iter.peek().is_some() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Rc::new(Token::new(TokenType::Eof, "".to_string(), self.line)));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            //
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            //
            '!' => {
                if self.r#match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.r#match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.r#match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.r#match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            //
            '/' => {
                if self.r#match('/') {
                    while self.source_iter.peek().is_some_and(|c| *c != '\n') {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }

            '\"' => self.string(),

            //
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,

            // 
            _ => {
                if Self::is_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    self.err_reporter.error(
                        self.line,
                        self.start,
                        self.current - self.start,
                        format!("Unexpected character \"{}\"", c.escape_default()).as_str(),
                    )
                }
            }
        }
    }

    fn number(&mut self) {
        while self.source_iter.peek().is_some_and(|&c| Self::is_digit(c)) {
            self.advance();
        }
        if self.source_iter.peek().is_some_and(|&c| c == '.')
            && self.peek_next().is_some_and(Self::is_digit)
        {
            self.advance();
            while self.source_iter.peek().is_some_and(|&c| Self::is_digit(c)) {
                self.advance();
            }
        }
        self.add_token(TokenType::Number(
            self.source[self.start..self.current].parse().unwrap(),
        ))
    }

    fn string(&mut self) {
        // read string until file ends or string ends with ending quote: \"
        while self.source_iter.peek().is_some_and(|c| *c != '\"') {
            if *self.source_iter.peek().unwrap() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // Error for unterminated string
        if self.source_iter.peek().is_none() {
            self.err_reporter.error(
                self.line,
                self.start,
                self.current - self.start,
                "Unterminated string",
            );
        } else {
            // Close string by consuming final \"
            self.advance();
    
            // Get string literal by removing surrounding quotes.
            let value = &self.source[self.start + 1..self.current - 1];
            self.add_token(TokenType::String(value.to_string()));
        }
    }

    fn identifier(&mut self) {
        while self
            .source_iter
            .peek()
            .is_some_and(|&c| Self::is_alphanumeric(c))
        {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        if let Some(token_type) = KEYWORDS.get(text).cloned() {
            self.add_token(token_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool where 'p: 'a {
        Self::is_digit(c) || Self::is_alpha(c)
    }

    /// Peek into the further next element that is returned by peek() method on source_iter.
    /// Since current is already advanced. Ths method returns current+1th char.
    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        self.source[self.current + 1..self.current + 2]
            .chars()
            .next()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_iter.next().unwrap()
    }

    /// advances iterator and increment current only if next char matches expected.
    fn r#match(&mut self, expected: char) -> bool {
        if self.source_iter.next_if_eq(&expected).is_some() {
            self.current += 1;
            return true;
        }
        false
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Rc::new(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            self.line,
        )));
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::lox::{error_reporter::ErrorReporter, token_type::TokenType, printer::TestPrinter};

    use super::Scanner;

    #[test]
    fn identifiers() {
        let source = "andy formless fo _ _123 _abc ab123
        abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";
        let printer = TestPrinter::default();
        let expected = vec![
            (TokenType::Identifier, "andy", 1),
            (TokenType::Identifier, "formless", 1),
            (TokenType::Identifier, "fo", 1),
            (TokenType::Identifier, "_", 1),
            (TokenType::Identifier, "_123", 1),
            (TokenType::Identifier, "_abc", 1),
            (TokenType::Identifier, "ab123", 1),
            (TokenType::Identifier, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_", 2),
            (TokenType::Eof, "", 2)
        ]; // (tken_type, lexeme, line)
        let error_reporter = ErrorReporter::new(
            source, false, &printer
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        for (i, tkn) in scanner.tokens.iter().enumerate() {
            assert_eq!(tkn.token_type, expected[i].0);
            assert_eq!(tkn.lexeme, expected[i].1);
            assert_eq!(tkn.line, expected[i].2);
        }
    }

    #[test]
    fn keywords() {
        let source = "and class else false for fun if nil or return super this true var while";
        let expected = vec![
            (TokenType::And, "and", 1),
            (TokenType::Class, "class", 1),
            (TokenType::Else, "else", 1),
            (TokenType::False, "false", 1),
            (TokenType::For, "for", 1),
            (TokenType::Fun, "fun", 1),
            (TokenType::If, "if", 1),
            (TokenType::Nil, "nil", 1),
            (TokenType::Or, "or", 1),
            (TokenType::Return, "return", 1),
            (TokenType::Super, "super", 1),
            (TokenType::This, "this", 1),
            (TokenType::True, "true", 1),
            (TokenType::Var, "var", 1),
            (TokenType::While, "while", 1),
            (TokenType::Eof, "", 1)
        ]; // (tken_type, lexeme, line)
        let printer = TestPrinter::default();
        let error_reporter = ErrorReporter::new(
            source, false, &printer
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        for (i, tkn) in scanner.tokens.iter().enumerate() {
            assert_eq!(tkn.token_type, expected[i].0);
            assert_eq!(tkn.lexeme, expected[i].1);
            assert_eq!(tkn.line, expected[i].2);
        }
    }

    #[test]
    fn numbers() {
        let source = "123
        123.456
        .456
        123.";
        let expected = vec![
            (TokenType::Number(123.0), "123", 1),
            (TokenType::Number(123.456), "123.456", 2),
            (TokenType::Dot, ".", 3),
            (TokenType::Number(456.0), "456", 3),
            (TokenType::Number(123.0), "123", 4),
            (TokenType::Dot, ".", 4),
            (TokenType::Eof, "", 4)
        ]; // (tken_type, lexeme, line)
        let printer = TestPrinter::default();
        let error_reporter = ErrorReporter::new(
            source, false, &printer
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        for (i, tkn) in scanner.tokens.iter().enumerate() {
            assert_eq!(tkn.token_type, expected[i].0);
            assert_eq!(tkn.lexeme, expected[i].1);
            assert_eq!(tkn.line, expected[i].2);
        }
    }

    #[test]
    fn strings() {
        let source = "\"\"
        \"string\"";
        let expected = vec![
            (TokenType::String("".to_string()), "\"\"", 1),
            (TokenType::String("string".to_string()), "\"string\"", 2),
            (TokenType::Eof, "", 2)
        ]; // (tken_type, lexeme, line)
        let printer = TestPrinter::default();
        let error_reporter = ErrorReporter::new(
            source, false, &printer
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        for (i, tkn) in scanner.tokens.iter().enumerate() {
            assert_eq!(tkn.token_type, expected[i].0);
            assert_eq!(tkn.lexeme, expected[i].1);
            assert_eq!(tkn.line, expected[i].2);
        }
    }

    #[test]
    fn whitespace() {
        let source = "space    tabs				newlines




        end";
        let expected = vec![
            (TokenType::Identifier, "space", 1),
            (TokenType::Identifier, "tabs", 1),
            (TokenType::Identifier, "newlines", 1),
            (TokenType::Identifier, "end", 6),
            (TokenType::Eof, "", 6)
        ]; // (tken_type, lexeme, line)
        let printer = TestPrinter::default();
        let error_reporter = ErrorReporter::new(
            source, false, &printer
        );
        let mut scanner = Scanner::new(source,  &error_reporter);
        scanner.scan_tokens();
        for (i, tkn) in scanner.tokens.iter().enumerate() {
            assert_eq!(tkn.token_type, expected[i].0);
            assert_eq!(tkn.lexeme, expected[i].1);
            assert_eq!(tkn.line, expected[i].2);
        }
    }
}
