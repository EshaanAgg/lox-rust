use crate::lexer::token::Token;
use crate::lexer::types::TokenType;

pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Token),
    Grouping(Box<Expr>),
}

pub trait Visitor<R> {
    fn visit_unary_expr(&self, op: &Token, expr: &Box<Expr>) -> R;
    fn visit_binary_expr(&self, expr1: &Box<Expr>, op: &Token, expr2: &Box<Expr>) -> R;
    fn visit_literal_expr(&self, token: &Token) -> R;
    fn visit_grouping_expr(&self, expr: &Box<Expr>) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
            Expr::Unary(op, expr) => visitor.visit_unary_expr(op, expr),
            Expr::Binary(expr1, op, expr2) => visitor.visit_binary_expr(expr1, op, expr2),
            Expr::Literal(token) => visitor.visit_literal_expr(token),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
        }
    }

    /// Creates a new unary expression with the given operator and expression.
    pub fn new_string_literal(value: &str) -> Expr {
        Expr::Literal(Token::new_default(
            TokenType::String(value.to_string()),
            value,
        ))
    }

    /// Creates a new unary expression with the given operator and expression.
    pub fn new_number_literal(value: f32) -> Expr {
        Expr::Literal(Token::new_default(
            TokenType::Number(value),
            &value.to_string(),
        ))
    }

    /// Creates a new unary expression with the given operator and expression.
    pub fn new_binary_expr(expr1: Expr, op: Token, expr2: Expr) -> Expr {
        Expr::Binary(Box::new(expr1), op, Box::new(expr2))
    }

    /// Creates a new unary expression with the given operator and expression.
    pub fn new_unary_expr(op: Token, expr: Expr) -> Expr {
        Expr::Unary(op, Box::new(expr))
    }

    /// Creates a new unary expression with the given operator and expression.
    pub fn new_grouping_expr(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }
}
