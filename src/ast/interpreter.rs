use anyhow::Error;

use super::expr::Expr;
use super::expr::Visitor;
use crate::lexer::{token::Token, types::TokenType};

pub struct Interpreter {}

#[derive(Debug, PartialEq)]
enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

impl Value {
    fn expect_number(&self) -> Result<f32, Error> {
        match self {
            Value::Number(num) => Ok(*num),
            _ => Err(Error::msg(format!("Expected number value, got {:?}", self))),
        }
    }

    fn expect_boolean(&self) -> Result<bool, Error> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(Error::msg(format!(
                "Expected boolean value, got {:?}",
                self
            ))),
        }
    }
}

impl Visitor<Result<Value, Error>> for Interpreter {
    fn visit_literal_expr(&self, value: &Token) -> Result<Value, Error> {
        match &value.token_type {
            TokenType::String(str) => Ok(Value::String(str.to_string())),
            TokenType::Number(num) => Ok(Value::Number(*num)),
            TokenType::TRUE => Ok(Value::Boolean(true)),
            TokenType::FALSE => Ok(Value::Boolean(false)),
            TokenType::NIL => Ok(Value::Nil),
            _ => panic!(
                "Unexpected token type: {:?} which should not be allowed by the AST parser.",
                value.token_type
            ),
        }
    }

    fn visit_binary_expr(
        &self,
        left: &Box<Expr>,
        op: &Token,
        right: &Box<Expr>,
    ) -> Result<Value, Error> {
        let left = left.accept(self)?;
        let right = right.accept(self)?;

        match op.token_type {
            TokenType::Plus => match (left, right) {
                // Adding two numbers
                (Value::Number(left_num), Value::Number(right_num)) => {
                    Ok(Value::Number(left_num + right_num))
                }

                // Concatenating two strings
                (Value::String(left_str), Value::String(right_str)) => {
                    Ok(Value::String(format!("{}{}", left_str, right_str)))
                }

                // Handling other cases
                (left, right) => Err(Error::msg(format!(
                    "Cannot add values of different types: {:?} and {:?}",
                    left, right
                ))),
            },

            TokenType::Minus => Ok(Value::Number(
                left.expect_number()? - right.expect_number()?,
            )),

            TokenType::Star => match (left, right) {
                // Multiplying two numbers
                (Value::Number(left_num), Value::Number(right_num)) => {
                    Ok(Value::Number(left_num * right_num))
                }

                // Multiplication with a string and a number
                (Value::String(str), Value::Number(num)) => {
                    Ok(Value::String(str.repeat(num as usize)))
                }

                // Handling other cases
                (left, right) => Err(Error::msg(format!(
                    "Cannot multiply values of different types: {:?} and {:?}",
                    left, right
                ))),
            },

            TokenType::Slash => Ok(Value::Number(
                left.expect_number()? / right.expect_number()?,
            )),

            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => match (left, right) {
                // Comparing two numbers
                (Value::Number(left_num), Value::Number(right_num)) => match op.token_type {
                    TokenType::Greater => Ok(Value::Boolean(left_num > right_num)),
                    TokenType::GreaterEqual => Ok(Value::Boolean(left_num >= right_num)),
                    TokenType::Less => Ok(Value::Boolean(left_num < right_num)),
                    TokenType::LessEqual => Ok(Value::Boolean(left_num <= right_num)),
                    _ => unreachable!(),
                },

                // Comparing two strings
                (Value::String(left_str), Value::String(right_str)) => match op.token_type {
                    TokenType::Greater => Ok(Value::Boolean(left_str > right_str)),
                    TokenType::GreaterEqual => Ok(Value::Boolean(left_str >= right_str)),
                    TokenType::Less => Ok(Value::Boolean(left_str < right_str)),
                    TokenType::LessEqual => Ok(Value::Boolean(left_str <= right_str)),
                    _ => unreachable!(),
                },

                (left, right) => Err(Error::msg(format!(
                    "Cannot compare values of different types: {:?} and {:?}",
                    left, right
                ))),
            },

            TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
            TokenType::BangEqual => Ok(Value::Boolean(left != right)),

            _ => panic!(
                "Unexpected operator {:?} which should not be allowed by the AST parser.",
                op.token_type
            ),
        }
    }

    fn visit_grouping_expr(&self, expr: &Box<Expr>) -> Result<Value, Error> {
        expr.accept(self)
    }

    fn visit_unary_expr(&self, op: &Token, expr: &Box<Expr>) -> Result<Value, Error> {
        let val = expr.accept(self)?;

        match op.token_type {
            TokenType::Minus => val.expect_number().map(|num| Value::Number(-num)),
            TokenType::Bang => val.expect_boolean().map(|b| Value::Boolean(!b)),

            _ => panic!(
                "Unexpected operator {:?} which should not be allowed by the AST parser.",
                op.token_type
            ),
        }
    }
}

#[test]
fn test_interpreter() {
    struct TestCase {
        description: &'static str,
        input: &'static str,
        expected: Value,
        should_err: bool,
    }

    let test_cases = vec![
        TestCase {
            description: "Addition",
            input: "12 + 34",
            expected: Value::Number(46.0),
            should_err: false,
        },
        TestCase {
            description: "Comparison",
            input: "12 > 34",
            expected: Value::Boolean(false),
            should_err: false,
        },
        TestCase {
            description: "Nested expressions",
            input: "1 + (2 - 3) * 4",
            expected: Value::Number(-3.0),
            should_err: false,
        },
        TestCase {
            description: "String concatenation",
            input: "\"Hello\" + \" \" + \"World\"",
            expected: Value::String("Hello World".to_string()),
            should_err: false,
        },
        TestCase {
            description: "Invalid operation",
            input: "12 + \"Hello\"",
            expected: Value::Nil,
            should_err: true,
        },
        TestCase {
            description: "Invalid boolean arithmetic",
            input: "true + false",
            expected: Value::Nil,
            should_err: true,
        },
        TestCase {
            description: "Invalid boolean operation",
            input: "(1 == 2) + 3",
            expected: Value::Nil,
            should_err: true,
        },
    ];

    let interpreter = Interpreter {};
    for test in test_cases {
        let tokens = crate::lexer::lexer::Lexer::new(test.input).get_tokens();
        let mut parser = super::syntax_tree::SyntaxTree::new(tokens);
        let expr = parser.expression().unwrap();
        let result = expr.accept(&interpreter);

        match (result, test.should_err) {
            (Ok(val), false) => assert_eq!(val, test.expected),
            (Err(_), true) => {}
            (res, _) => panic!(
                "Test failed: {}\nExpected: {:?}\nGot: {:?}",
                test.description, test.expected, res
            ),
        }
    }
}
