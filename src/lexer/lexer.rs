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

    /// Consumes the next character in the source code and checks if it matches the expected character.
    fn match_next(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.consume();
            true
        } else {
            false
        }
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

    /// Returns if the provided digit is a 0-9 digit
    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    /// Returns if the provided character is an identifier character
    fn is_identifier(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    /// Returns if the provided character is an alphanumeric character
    fn is_aplhanumeric(c: char) -> bool {
        return Self::is_digit(c) || Self::is_identifier(c);
    }

    /// Parses an identifier from the input. It assumes that it has already been
    /// checked that the first character is an identifier character.
    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.peek() {
            if !Self::is_aplhanumeric(ch) {
                break;
            }

            self.consume();
            identifier.push(ch);
        }

        identifier
    }

    /// Parses an integer from the input. Returns the parsed integer
    /// and the number of characters consumed
    fn parse_integer(&mut self) -> (u32, usize) {
        let mut res = 0;
        let mut consumed = 0;

        while let Some(ch) = self.peek() {
            if !Self::is_digit(ch) {
                break;
            }

            self.consume();
            consumed += 1;
            let digit = ch as u32 - ('0' as u32);
            res = res * 10 + digit;
        }

        (res, consumed)
    }

    /// Parses a floating-point or integer number from the source code.
    fn parse_number(&mut self) -> f32 {
        let mut num = self.parse_integer().0 as f32;

        if self.match_next('.') {
            if let Some(ch) = self.peek() {
                if Self::is_digit(ch) {
                    let (fr, len) = self.parse_integer();
                    num += fr as f32 / 10_f32.powi(len as i32);
                }
            }
        }

        num
    }

    /// Parses a string token from the source code.
    fn parse_string_token(&mut self) -> Token {
        let mut literal = String::new();
        let mut lexeme = String::from('"');

        while let Some(ch) = self.peek() {
            lexeme.push(ch);

            // Reached the end of the line before the string was terminated
            if ch == '\n' {
                return self.new_token(UnterminatedString(literal), lexeme.as_str());
            }

            self.consume();

            if ch == '"' {
                return self.new_token(String(literal), lexeme.as_str());
            }

            literal.push(ch);
        }

        // Reached the end of the source code before the string was terminated
        self.new_token(UnterminatedString(literal), lexeme.as_str())
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
                '=' => match self.match_next('=') {
                    true => self.new_token(EqualEqual, "=="),
                    false => self.new_token(Equal, "="),
                },
                '!' => match self.match_next('=') {
                    true => self.new_token(BangEqual, "!="),
                    false => self.new_token(Bang, "!"),
                },

                // Relational Operators
                '>' => match self.match_next('=') {
                    true => self.new_token(GreaterEqual, ">="),
                    false => self.new_token(Greater, ">"),
                },
                '<' => match self.match_next('=') {
                    true => self.new_token(LessEqual, "<="),
                    false => self.new_token(Less, "<"),
                },

                // Literals
                '"' => self.parse_string_token(),

                _ => {
                    if Self::is_digit(ch) {
                        let num = self.parse_number();
                        return self.new_token(Number(num), num.to_string().as_str());
                    }

                    if Self::is_identifier(ch) {
                        let identifier = self.parse_identifier();

                        if let Some(keyword) = TokenType::check_keyword(identifier.as_str()) {
                            return self.new_token(keyword, identifier.as_str());
                        }

                        return self.new_token(Identifier(identifier.clone()), identifier.as_str());
                    }

                    self.new_token(Unknown, String::from(ch).as_str())
                }
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
