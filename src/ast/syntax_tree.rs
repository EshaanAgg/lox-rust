use crate::lexer::{token::Token, types::TokenType};

use super::{expr::Expr, printer::AstPrinter};

#[derive(Debug)]
pub struct SyntaxTree {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, PartialEq)]
pub struct ParserError {
    message: String,
    line: usize,
    character: usize,
}

impl ParserError {
    fn new(token: Option<&Token>, message: &str) -> Self {
        ParserError {
            message: message.to_string(),
            line: token.map_or_else(|| 0, |t| t.line),
            character: token.map_or_else(|| 0, |t| t.character),
        }
    }
}

impl SyntaxTree {
    fn new(tokens: Vec<Token>) -> Self {
        SyntaxTree { tokens, current: 0 }
    }

    /// Returns the next token in the source code without consuming it.
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).map(|t| t.clone())
    }

    /// Consumes the next token in the source code and returns it.
    fn consume(&mut self) -> Option<Token> {
        let tok = self.peek();
        self.current += 1;
        tok
    }

    /// Consumes the next token in the source code and checks if it matches the expected token type.
    /// Does not work with literal types like String, Number, or Identifier as their lexeme is not known at compile time.
    fn expect(&mut self, expected_types: &[TokenType]) -> Result<(), ParserError> {
        match self.consume() {
            None => Err(ParserError::new(
                None,
                "Expected a ) to close the expression, but arrived at EOF",
            )),
            Some(token) => match expected_types.contains(&token.token_type) {
                true => Ok(()),
                false => Err(ParserError::new(
                    Some(&token),
                    format!(
                        "Expected one of types {:?}, but got {:?}",
                        expected_types, token.token_type
                    )
                    .as_str(),
                )),
            },
        }
    }

    /// Checks if the next token matches with the provided types. It is does, then the token is consumed,
    /// otherwise the state is left as is.
    /// Does not work with literal types like String, Number, or Identifier as their lexeme is not known at compile time.
    fn matches(&mut self, expected_types: &[TokenType]) -> Option<Token> {
        match self.peek() {
            None => None,
            Some(token) => match expected_types.contains(&token.token_type) {
                true => {
                    self.consume();
                    Some(token)
                }
                false => None,
            },
        }
    }

    /// Parses an expression.
    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    /// Parses an equality expression.
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparision()?;

        while let Some(tok) = self.matches(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let right = self.comparision()?;
            expr = Expr::new_binary_expr(expr, tok, right);
        }

        Ok(expr)
    }

    /// Parses a comparision expression.
    fn comparision(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while let Some(tok) = self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let right = self.term()?;
            expr = Expr::new_binary_expr(expr, tok, right);
        }

        Ok(expr)
    }

    /// Parses a term expression.
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while let Some(tok) = self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let right = self.factor()?;
            expr = Expr::new_binary_expr(expr, tok, right);
        }

        Ok(expr)
    }

    /// Parses a factor expression.
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while let Some(tok) = self.matches(&[TokenType::Star, TokenType::Slash]) {
            let right = self.unary()?;
            expr = Expr::new_binary_expr(expr, tok, right);
        }

        Ok(expr)
    }

    /// Parses a unary expression.
    fn unary(&mut self) -> Result<Expr, ParserError> {
        match self.matches(&[TokenType::Bang, TokenType::Minus]) {
            None => self.primary(),
            Some(tok) => Ok(Expr::new_unary_expr(tok, self.unary()?)),
        }
    }

    /// Parses a primary expression.
    fn primary(&mut self) -> Result<Expr, ParserError> {
        match self.consume() {
            None => Err(ParserError::new(None, "Unexpected end of file")),
            Some(token) => match token.token_type {
                TokenType::String(_)
                | TokenType::Number(_)
                | TokenType::Identifier(_)
                | TokenType::TRUE
                | TokenType::FALSE
                | TokenType::NIL => Ok(Expr::Literal(token)),

                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    let _ = self.expect(&[TokenType::RightParen])?;
                    Ok(Expr::new_grouping_expr(expr))
                }

                _ => Err(ParserError::new(
                    Some(&token),
                    format!(
                        "Unexpected type of token, expected a literal but got {:?}",
                        token.token_type
                    )
                    .as_str(),
                )),
            },
        }
    }

    /// Prints the syntax tree generated from the source code.
    /// Makes use of the AstPrinter to generate the string representation of the syntax tree.
    fn print(&mut self) -> String {
        match self.expression() {
            Ok(expr) => AstPrinter::print(expr),
            Err(err) => format!(
                "Error at line {}:{}: {}",
                err.line, err.character, err.message
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token;
    use crate::lexer::types::TokenType;

    #[test]
    fn test_expect() {
        let tokens = vec![
            Token::new_default(TokenType::LeftParen, "("),
            Token::new_default(TokenType::RightParen, ")"),
            Token::new_default(TokenType::NIL, "nil"),
            Token::new_default(TokenType::AND, "and"),
        ];

        let mut syntax_tree = SyntaxTree::new(tokens);

        assert!(syntax_tree.expect(&[TokenType::LeftParen]).is_ok());
        assert!(syntax_tree.expect(&[TokenType::NIL]).is_err());
        assert!(syntax_tree.expect(&[TokenType::NIL]).is_ok());
        assert!(syntax_tree.expect(&[TokenType::AND]).is_ok());
    }

    #[test]
    fn test_print_valid_expr() {
        struct TestCase<'a> {
            name: &'a str,
            input: &'a str,
            expected: &'a str,
        }

        let test_cases = vec![
            TestCase {
                name: "Simple addition",
                input: "12 + 34",
                expected: "(+ 12 34)",
            },
            TestCase {
                name: "Simple subtraction",
                input: "12 - 34",
                expected: "(- 12 34)",
            },
            TestCase {
                name: "Simple multiplication",
                input: "12 * 34",
                expected: "(* 12 34)",
            },
            TestCase {
                name: "Simple division",
                input: "12 / 34",
                expected: "(/ 12 34)",
            },
            TestCase {
                name: "Simple grouping",
                input: "(12 + 34)",
                expected: "(group (+ 12 34))",
            },
            TestCase {
                name: "Simple grouping with multiple operators",
                input: "(12 + 34) * 56",
                expected: "(* (group (+ 12 34)) 56)",
            },
            TestCase {
                name: "Precedence: Grouping",
                input: "12 + (34 * 56)",
                expected: "(+ 12 (group (* 34 56)))",
            },
            TestCase {
                name: "Precedence: Primary > Unary",
                input: "-12 + 34",
                expected: "(+ (- 12) 34)",
            },
            TestCase {
                name: "Precedence: Unary > Factor",
                input: "-12 * 34",
                expected: "(* (- 12) 34)",
            },
            TestCase {
                name: "Precedence: Factor > Term",
                input: "-12 * 34 / 56",
                expected: "(/ (* (- 12) 34) 56)",
            },
            TestCase {
                name: "Precedence: Term > Comparison",
                input: "-12 * 34 / 56 > 78",
                expected: "(> (/ (* (- 12) 34) 56) 78)",
            },
            TestCase {
                name: "Precedence: Comparison > Equality",
                input: "-12 * 34 / 56 > 78 == 90",
                expected: "(== (> (/ (* (- 12) 34) 56) 78) 90)",
            },
            TestCase {
                name: "Left Associativity: Equality",
                input: "12 == 34 == 56",
                expected: "(== (== 12 34) 56)",
            },
            TestCase {
                name: "Left Associativity: Comparison",
                input: "12 > 34 > 56",
                expected: "(> (> 12 34) 56)",
            },
            TestCase {
                name: "Left Associativity: Term",
                input: "12 * 34 * 56",
                expected: "(* (* 12 34) 56)",
            },
            TestCase {
                name: "Left Associativity: Factor",
                input: "12 / 34 / 56",
                expected: "(/ (/ 12 34) 56)",
            },
            TestCase {
                name: "Right Associativity: Unary",
                input: "!-12",
                expected: "(! (- 12))",
            },
        ];

        for test_case in test_cases {
            let tokens = crate::lexer::lexer::Lexer::new(test_case.input).get_tokens();
            let mut syntax_tree = SyntaxTree::new(tokens);

            assert_eq!(
                syntax_tree.print(),
                test_case.expected,
                "{}",
                test_case.name
            );
        }
    }
}
