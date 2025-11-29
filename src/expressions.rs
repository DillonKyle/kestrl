use crate::scanner::Literal;
use crate::scanner::Token;

pub enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(LiteralExpr),
}

pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token, // You can replace String with a more specific type if needed
    pub right: Expr,
}

pub struct UnaryExpr {
    pub operator: Token, // You can replace String with a more specific type if needed
    pub right: Expr,
}

pub struct GroupingExpr {
    pub expression: Expr,
}

pub struct LiteralExpr {
    pub value: Literal,
}
