use super::{token::Token, types::TokenType};
use TokenType::{
    Comma, Dot, LeftBrace, LeftParen, Minus, Plus, RightBrace, RightParen, Semicolon, Star,
    Unknown, EOF,
};

#[derive(Debug)]
pub struct Lexer {
    characters: Vec<char>,

    current: usize,
    line: u32,
    character: u32,
}

impl Lexer {
    /// Creates a new lexer with the given source code.
    pub fn new(source: &str) -> Self {
        Self {
            characters: source.chars().collect(),
            current: 0,
            line: 1,
            character: 1,
        }
    }

    /// Returns the next character in the source code without consuming it.
    fn peek(&self) -> Option<char> {
        self.characters.get(self.current).copied()
    }

    /// Consumes the next character in the source code and returns it.
    fn consume(&mut self) -> Option<char> {
        let next = self.peek();
        if let Some(ch) = next {
            if ch == '\n' {
                self.line += 1;
                self.character = 1;
            }
        }
        self.current += 1;

        next
    }

    /// Creates a new token with the given token type and lexeme.
    fn new_token(&self, token_type: TokenType, lexeme: &str) -> Token {
        Token::new(token_type, lexeme, self.line, self.character)
    }

    /// Returns the next token in the source code. It consumes the source code
    /// character by character and returns a token for each character.
    pub fn next_token(&mut self) -> Token {
        match self.consume() {
            None => self.new_token(EOF, ""),
            Some(ch) => match ch {
                '(' => self.new_token(LeftParen, "("),
                ')' => self.new_token(RightParen, ")"),
                '{' => self.new_token(LeftBrace, "{"),
                '}' => self.new_token(RightBrace, "}"),

                '*' => self.new_token(Star, "*"),
                '.' => self.new_token(Dot, "."),
                ',' => self.new_token(Comma, ","),
                ';' => self.new_token(Semicolon, ";"),
                '+' => self.new_token(Plus, "+"),
                '-' => self.new_token(Minus, "-"),

                _ => self.new_token(Unknown, String::from(ch).as_str()),
            },
        }
    }

    /// Returns a tuple of two vectors containing the tokens from the source code.
    /// The first vector contains the tokens that were successfully parsed, and the
    /// second vector contains the tokens that were not successfully parsed (Unknown tokens)
    /// Both the vectors contain the tokens in the order they appear in the source code.
    pub fn get_tokens(&mut self) -> (Vec<Token>, Vec<Token>) {
        let mut tokens = Vec::new();
        let mut unknown_tokens = Vec::new();

        let mut to_break = false;
        while !to_break {
            let token = self.next_token();

            if token.token_type == EOF {
                to_break = true;
            }

            if token.token_type == Unknown {
                unknown_tokens.push(token);
            } else {
                tokens.push(token);
            }
        }

        (tokens, unknown_tokens)
    }
}
