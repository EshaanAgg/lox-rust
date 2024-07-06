#[derive(Debug, PartialEq, Clone, Copy)]
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
}
