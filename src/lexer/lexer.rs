use super::{token::Token, types::TokenType};

use std::string::String;
use TokenType::*;

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

    /// Skips any whitespace characters in the source code.
    /// Whitespace characters include spaces, tabs, carriage returns, and newlines.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.consume();
                }
                _ => break,
            }
        }
    }

    /// Returns the next token in the source code. It consumes the source code
    /// character by character and returns a token for each character.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.consume() {
            None => self.new_token(EOF, ""),
            Some(ch) => match ch {
                // Braces and Parentheses
                '(' => self.new_token(LeftParen, "("),
                ')' => self.new_token(RightParen, ")"),
                '{' => self.new_token(LeftBrace, "{"),
                '}' => self.new_token(RightBrace, "}"),

                // Operators
                '*' => self.new_token(Star, "*"),
                '.' => self.new_token(Dot, "."),
                ',' => self.new_token(Comma, ","),
                ';' => self.new_token(Semicolon, ";"),
                '+' => self.new_token(Plus, "+"),
                '-' => self.new_token(Minus, "-"),
                '/' => {
                    if self.peek() == Some('/') {
                        // The following characters are a comment
                        while self.peek() != Some('\n') && self.peek() != None {
                            self.consume();
                        }
                        self.next_token()
                    } else {
                        self.new_token(Slash, "/")
                    }
                }

                // Equality and Negation
                '=' => match self.peek() {
                    Some('=') => {
                        self.consume();
                        self.new_token(EqualEqual, "==")
                    }
                    _ => self.new_token(Equal, "="),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.consume();
                        self.new_token(BangEqual, "!=")
                    }
                    _ => self.new_token(Bang, "!"),
                },

                // Relational Operators
                '>' => match self.peek() {
                    Some('=') => {
                        self.consume();
                        self.new_token(GreaterEqual, ">=")
                    }
                    _ => self.new_token(Greater, ">"),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.consume();
                        self.new_token(LessEqual, "<=")
                    }
                    _ => self.new_token(Less, "<"),
                },

                // String Literals
                '"' => {
                    let mut literal = String::new();
                    let mut lexeme = String::from(ch);

                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            // Arrived at a newline character before the string was terminated. Example values:
                            // Lexeme: "abc
                            // Literal: abc
                            return self.new_token(UnterminatedString(literal), lexeme.as_str());
                        }

                        lexeme.push(ch);
                        self.consume();
                        if ch == '"' {
                            return self.new_token(String(literal), lexeme.as_str());
                        }

                        literal.push(ch);
                    }

                    self.new_token(UnterminatedString(literal), lexeme.as_str())
                }

                _ => self.new_token(Unknown, String::from(ch).as_str()),
            },
        }
    }

    /// Returns a vector of tokens from the source code.
    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut to_break = false;
        while !to_break {
            let token = self.next_token();
            if token.token_type == EOF {
                to_break = true;
            }
            tokens.push(token);
        }

        tokens
    }
}
