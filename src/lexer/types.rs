#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    EOF,
    Unknown,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Star,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
}
