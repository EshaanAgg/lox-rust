#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    Unknown,

    // Braces and Parentheses
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Operators
    Star,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Slash,

    // Equality and Negation
    Bang,
    Equal,
    EqualEqual,
    BangEqual,

    // Relational Operators
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    String(String),
    Identifier(String),
    UnterminatedString(String),
    Number(f32),

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

impl TokenType {
    pub fn check_keyword(str: &str) -> Option<Self> {
        match str {
            "and" => Some(Self::AND),
            "class" => Some(Self::CLASS),
            "else" => Some(Self::ELSE),
            "false" => Some(Self::FALSE),
            "for" => Some(Self::FOR),
            "fun" => Some(Self::FUN),
            "if" => Some(Self::IF),
            "nil" => Some(Self::NIL),
            "or" => Some(Self::OR),
            "print" => Some(Self::PRINT),
            "return" => Some(Self::RETURN),
            "super" => Some(Self::SUPER),
            "this" => Some(Self::THIS),
            "true" => Some(Self::TRUE),
            "var" => Some(Self::VAR),
            "while" => Some(Self::WHILE),
            _ => None,
        }
    }
}
