use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType, lexeme: &'a str, line: usize
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}