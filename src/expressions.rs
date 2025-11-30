use crate::scanner::Literal;
use crate::scanner::Token;

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(LiteralExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: Literal,
}
