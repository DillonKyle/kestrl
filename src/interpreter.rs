use crate::expressions::{self, Visitor};
use crate::scanner::Literal;
use crate::token_types::TokenType;

pub struct Interpreter;

pub struct RuntimeError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Visitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &expressions::BinaryExpr) -> Result<Value, RuntimeError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::PLUS => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => Err(RuntimeError {
                    message: "Operands must be two numbers or two strings.".to_string(),
                }),
            },
            TokenType::MINUS => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::STAR => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::SLASH => match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r == 0.0 {
                        Err(RuntimeError {
                            message: "Division by zero.".to_string(),
                        })
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::GREATER => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::GREATER_EQUAL => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::LESS => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::LESS_EQUAL => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                _ => Err(RuntimeError {
                    message: "Operands must be numbers.".to_string(),
                }),
            },
            TokenType::EQUAL_EQUAL => Ok(Value::Boolean(self.is_equal(&left, &right))),
            TokenType::BANG_EQUAL => Ok(Value::Boolean(!self.is_equal(&left, &right))),
            _ => Err(RuntimeError {
                message: "Unknown binary operator.".to_string(),
            }),
        }
    }

    fn visit_unary_expr(&mut self, expr: &expressions::UnaryExpr) -> Result<Value, RuntimeError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => match right {
                Value::Number(r) => Ok(Value::Number(-r)),
                _ => Err(RuntimeError {
                    message: "Operand must be a number.".to_string(),
                }),
            },
            TokenType::BANG => Ok(Value::Boolean(self.is_truthy(&right))),
            _ => Err(RuntimeError {
                message: "Unknown unary operator.".to_string(),
            }),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &expressions::GroupingExpr,
    ) -> Result<Value, RuntimeError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(
        &mut self,
        expr: &expressions::LiteralExpr,
    ) -> Result<Value, RuntimeError> {
        match &expr.value {
            Literal::Nil => Ok(Value::Nil),
            Literal::Bool(b) => Ok(Value::Boolean(*b)),
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::Str(s) => Ok(Value::String(s.clone())),
            Literal::Unknown(u) => Err(RuntimeError {
                message: format!("Unknown literal: {u}."),
            }),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn evaluate(&mut self, expr: &expressions::Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    pub fn is_truthy(&mut self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn is_equal(&mut self, a: &Value, b: &Value) -> bool {
        if a == &Value::Nil && b == &Value::Nil {
            return true;
        }
        if a == &Value::Nil {
            return false;
        }
        a == b
    }

    pub fn interpret(&mut self, expr: &expressions::Expr) {
        match self.evaluate(expr) {
            Ok(value) => println!("Result: {value:?}"),
            Err(e) => eprintln!("Runtime error: {}", e.message),
        }
    }
}
