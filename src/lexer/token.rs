use super::types::TokenType;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub character: usize,
}

impl Token {
    /// Creates a new token with the given token type, lexeme, line, and character values.
    pub fn new(token_type: TokenType, lexeme: &str, line: usize, character: usize) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line,
            character,
        }
    }

    /// Creates a new token with the default line and character values of 0.
    /// This is intended to be only used for testing purposes.
    pub fn new_default(token_type: TokenType, lexeme: &str) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: 0,
            character: 0,
        }
    }

    /// Returns the value of the token as a string.
    pub fn get_value(&self) -> String {
        match &self.token_type {
            TokenType::Number(val) => {
                // If the number is an integer, display it with one decimal place
                if val.fract() == 0.0 {
                    format!("{:.1}", val)
                } else {
                    format!("{}", val)
                }
            },
            TokenType::String(val) => val.to_string(),
            _ => "null".to_string(),
        }
    }

    /// Returns the name of the token type as a string.
    pub fn get_name(&self) -> String {
        match &self.token_type {
            TokenType::EOF => "EOF".to_string(),

            TokenType::LeftParen => "LEFT_PAREN".to_string(),
            TokenType::RightParen => "RIGHT_PAREN".to_string(),
            TokenType::LeftBrace => "LEFT_BRACE".to_string(),
            TokenType::RightBrace => "RIGHT_BRACE".to_string(),

            TokenType::Star => "STAR".to_string(),
            TokenType::Dot => "DOT".to_string(),
            TokenType::Comma => "COMMA".to_string(),
            TokenType::Semicolon => "SEMICOLON".to_string(),
            TokenType::Plus => "PLUS".to_string(),
            TokenType::Minus => "MINUS".to_string(),
            TokenType::Slash => "SLASH".to_string(),

            TokenType::Bang => "BANG".to_string(),
            TokenType::Equal => "EQUAL".to_string(),
            TokenType::EqualEqual => "EQUAL_EQUAL".to_string(),
            TokenType::BangEqual => "BANG_EQUAL".to_string(),

            TokenType::Greater => "GREATER".to_string(),
            TokenType::GreaterEqual => "GREATER_EQUAL".to_string(),
            TokenType::Less => "LESS".to_string(),
            TokenType::LessEqual => "LESS_EQUAL".to_string(),

            TokenType::String(_) => "STRING".to_string(),
            TokenType::Number(_) => "NUMBER".to_string(),
            TokenType::Identifier(_) => "IDENTIFIER".to_string(),

            TokenType::AND => "AND".to_string(),    
            TokenType::CLASS => "CLASS".to_string(),
            TokenType::ELSE => "ELSE".to_string(),
            TokenType::FALSE => "FALSE".to_string(),
            TokenType::FOR => "FOR".to_string(),
            TokenType::FUN => "FUN".to_string(),
            TokenType::IF => "IF".to_string(),
            TokenType::NIL => "NIL".to_string(),
            TokenType::OR => "OR".to_string(),
            TokenType::PRINT => "PRINT".to_string(),
            TokenType::RETURN => "RETURN".to_string(),
            TokenType::SUPER => "SUPER".to_string(),
            TokenType::THIS => "THIS".to_string(),
            TokenType::TRUE => "TRUE".to_string(),
            TokenType::VAR => "VAR".to_string(),
            TokenType::WHILE => "WHILE".to_string(),

            TokenType::UnterminatedString(_) => "UnterminatedString".to_string(),
            TokenType::Unknown => "Unknown".to_string(),
        }
    }

    /// Returns a string representation of the token in the format:
    /// <token_type> <lexeme> <literal>
    /// This is used for the "tokenize" command.
    pub fn tokenized_string(&self) -> String {
        match &self.token_type {
            // Errors
            TokenType::UnterminatedString(_) => {
                format!("[line {}] Error: Unterminated string.", self.line)
            }
            TokenType::Unknown => {
                format!(
                    "[line {}] Error: Unexpected character: {}",
                    self.line, self.lexeme
                )
            }

            _ => {
                format!(
                    "{} {} {}",
                    self.get_name(),
                    self.lexeme,
                    self.get_value()
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

/// Implements the Debug trait for the Token struct.
/// We only want to display the token type when debugging, and not the entire struct
/// with fields like lexeme, line, and character.
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}
