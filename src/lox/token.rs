use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    // pub lexeme: &'a str,
    pub lexeme: String,
    pub line: usize
}

impl Token {
    pub fn new(
        token_type: TokenType, lexeme: String, line: usize
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}