use crate::lexer::token::Token;
use crate::lexer::types::TokenType;

pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
}

pub trait Visitor<R> {
    fn visit_unary_expr(&self, op: &Token, expr: &Box<Expr>) -> R;
    fn visit_binary_expr(&self, left: &Box<Expr>, op: &Token, right: &Box<Expr>) -> R;
    fn visit_grouping_expr(&self, expr: &Box<Expr>) -> R;
    fn visit_literal_expr(&self, value: &Token) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
            Expr::Unary(op, expr) => visitor.visit_unary_expr(op, expr),
            Expr::Binary(left, op, right) => visitor.visit_binary_expr(left, op, right),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(value) => visitor.visit_literal_expr(value),
        }
    }
}

// Custom implementations for the Expr enum.
impl Expr {
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
