use super::expr::{Expr, Visitor};
use crate::lexer::token::Token;
use crate::lexer::types::TokenType;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_unary_expr(&self, op: &Token, expr: &Box<Expr>) -> String {
        format!("({} {})", op.lexeme, expr.accept(self))
    }

    fn visit_binary_expr(&self, expr1: &Box<Expr>, op: &Token, expr2: &Box<Expr>) -> String {
        format!(
            "({} {} {})",
            op.lexeme,
            expr1.accept(self),
            expr2.accept(self)
        )
    }

    fn visit_literal_expr(&self, token: &Token) -> String {
        match &token.token_type {
            TokenType::String(str) => str.to_string(),
            TokenType::Number(num) => num.to_string(),
            _ => "not implemented".to_string(),
        }
    }

    fn visit_grouping_expr(&self, expr: &Box<Expr>) -> String {
        format!("(group {})", expr.accept(self))
    }
}

impl AstPrinter {
    pub fn print(expr: Expr) -> String {
        expr.accept(&Self)
    }
}

#[test]
fn test_print() {
    struct TestCase<'a> {
        input: Expr,
        expected: &'a str,
    }

    let testcases = vec![
        TestCase {
            input: Expr::new_binary_expr(
                Expr::new_number_literal(12.0),
                Token::new_default(TokenType::Plus, "+"),
                Expr::new_number_literal(34.0),
            ),
            expected: "(+ 12 34)",
        },
        TestCase {
            input: Expr::new_binary_expr(
                Expr::new_number_literal(12.0),
                Token::new_default(TokenType::Plus, "+"),
                Expr::new_grouping_expr(Expr::new_number_literal(34.0)),
            ),
            expected: "(+ 12 (group 34))",
        },
        TestCase {
            input: Expr::new_binary_expr(
                Expr::new_unary_expr(
                    Token::new_default(TokenType::Minus, "-"),
                    Expr::new_number_literal(123.0),
                ),
                Token::new_default(TokenType::Star, "*"),
                Expr::new_grouping_expr(Expr::new_number_literal(45.67)),
            ),
            expected: "(* (- 123) (group 45.67))",
        },
    ];

    for t in testcases {
        println!("Testing: {:?}", t.input);
        assert_eq!(AstPrinter::print(t.input), t.expected);
    }
}
