use super::types::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub character: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: u32, character: u32) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line,
            character,
        }
    }

    /// Returns a string representation of the token in the format:
    /// <token_type> <lexeme> <literal>
    /// This is used for the "tokenize" command.
    pub fn tokenized_string(&self) -> String {
        match &self.token_type {
            TokenType::EOF => format!("EOF  null"),

            // Braces and Parentheses
            TokenType::LeftParen => format!("LEFT_PAREN {} null", self.lexeme),
            TokenType::RightParen => format!("RIGHT_PAREN {} null", self.lexeme),
            TokenType::LeftBrace => format!("LEFT_BRACE {} null", self.lexeme),
            TokenType::RightBrace => format!("RIGHT_BRACE {} null", self.lexeme),

            // Operators
            TokenType::Star => format!("STAR {} null", self.lexeme),
            TokenType::Dot => format!("DOT {} null", self.lexeme),
            TokenType::Comma => format!("COMMA {} null", self.lexeme),
            TokenType::Semicolon => format!("SEMICOLON {} null", self.lexeme),
            TokenType::Plus => format!("PLUS {} null", self.lexeme),
            TokenType::Minus => format!("MINUS {} null", self.lexeme),
            TokenType::Slash => format!("SLASH {} null", self.lexeme),

            // Equality and Negation
            TokenType::Bang => format!("BANG {} null", self.lexeme),
            TokenType::Equal => format!("EQUAL {} null", self.lexeme),
            TokenType::EqualEqual => format!("EQUAL_EQUAL {} null", self.lexeme),
            TokenType::BangEqual => format!("BANG_EQUAL {} null", self.lexeme),

            // Relational Operators
            TokenType::Greater => format!("GREATER {} null", self.lexeme),
            TokenType::GreaterEqual => format!("GREATER_EQUAL {} null", self.lexeme),
            TokenType::Less => format!("LESS {} null", self.lexeme),
            TokenType::LessEqual => format!("LESS_EQUAL {} null", self.lexeme),

            // Literals
            TokenType::String(val) => format!("STRING {} {}", self.lexeme, val),

            // Error
            TokenType::UnterminatedString(_) => {
                format!("[line {}] Error: Unterminated string.", self.line)
            }
            TokenType::Unknown => {
                format!(
                    "[line {}] Error: Unexpected character: {}",
                    self.line, self.lexeme
                )
            }
        }
    }

    pub fn is_error(&self) -> bool {
        match self.token_type {
            TokenType::Unknown | TokenType::UnterminatedString(_) => true,
            _ => false,
        }
    }
}
