use crate::{
    expressions::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    scanner::{Literal, Token},
    token_types::TokenType,
};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Expr {
        self.expr()
    }

    pub fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(Box::new(UnaryExpr { operator, right }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[TokenType::FALSE]) {
            return Expr::Literal(LiteralExpr {
                value: Literal::Bool(false),
            });
        }
        if self.match_token(&[TokenType::TRUE]) {
            return Expr::Literal(LiteralExpr {
                value: Literal::Bool(true),
            });
        }
        if self.match_token(&[TokenType::NIL]) {
            return Expr::Literal(LiteralExpr {
                value: Literal::Nil,
            });
        }
        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            });
        }
        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expr();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(GroupingExpr { expression: expr }));
        }
        panic!("Expect expression.");
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(&token_type) {
            return self.advance();
        }
        panic!("{}", message);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }

            self.advance();
        }
    }
}
