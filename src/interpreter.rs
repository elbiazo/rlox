use crate::expr;
use crate::scanner;

pub struct Interpreter;
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn visit_expr(&self, expr: expr::Expr) -> Result<Value, &str> {
        match expr {
            expr::Expr::Literal(lit) => Ok(self.visit_literal_expr(lit)),
            expr::Expr::Grouping(e) => self.visit_expr(*e),
            expr::Expr::Unary(op, e) => self.visit_unary_expr(op.clone(), *e),
            expr::Expr::Binary(left, op, right) => self.visit_binary_expr(*left, op, *right),
        }
    }

    fn visit_binary_expr(
        &self,
        left: expr::Expr,
        op: scanner::Token,
        right: expr::Expr,
    ) -> Result<Value, &str> {
        match self.visit_expr(left) {
            Ok(val) => match val {
                // Checking Number op Number
                Value::Number(left_val) => match self.visit_expr(right) {
                    Ok(val) => match val {
                        Value::Number(right_val) => match op.tok_type {
                            scanner::TokenType::Plus => Ok(Value::Number(left_val + right_val)),
                            scanner::TokenType::Minus => Ok(Value::Number(left_val - right_val)),
                            scanner::TokenType::Slash => Ok(Value::Number(left_val / right_val)),
                            scanner::TokenType::Star => Ok(Value::Number(left_val * right_val)),
                            // Comparison Operator
                            scanner::TokenType::Greater => Ok(Value::Bool(left_val > right_val)),
                            scanner::TokenType::GreaterEqual => {
                                Ok(Value::Bool(left_val >= right_val))
                            }
                            scanner::TokenType::Less => Ok(Value::Bool(left_val < right_val)),
                            scanner::TokenType::LessEqual => Ok(Value::Bool(left_val <= right_val)),
                            scanner::TokenType::BangEqual => Ok(Value::Bool(left_val != right_val)),
                            scanner::TokenType::EqualEqual => {
                                Ok(Value::Bool(left_val == right_val))
                            }
                            _ => return Err("Unsuppored binary expr"),
                        },
                        _ => return Err("Binary expr needs f64"),
                    },
                    Err(msg) => return Err(msg),
                },

                // Checking String + String
                Value::String(left_val) => match self.visit_expr(right) {
                    Ok(val) => match val {
                        Value::String(right_val) => match op.tok_type {
                            scanner::TokenType::Plus => {
                                Ok(Value::String(format!("{}{}", left_val, right_val)))
                            }
                            _ => return Err("Unsuppored binary expr"),
                        },
                        _ => return Err("Binary expr needs String"),
                    },
                    Err(msg) => return Err(msg),
                },

                _ => return Err("Binary expr needs f64"),
            },
            Err(msg) => return Err(msg),
        }
    }

    pub fn visit_unary_expr(&self, op: scanner::Token, e: expr::Expr) -> Result<Value, &str> {
        match self.visit_expr(e) {
            Ok(val) => match val {
                Value::Number(num) => match op.tok_type {
                    scanner::TokenType::Minus => Ok(Value::Number(-num)),
                    scanner::TokenType::Plus => Ok(Value::Number(num)),
                    scanner::TokenType::Bang => Ok(Value::Bool(false)),
                    _ => Err("Op type is not minus or plus"),
                },
                _ => match op.tok_type {
                    scanner::TokenType::Bang => Ok(Value::Bool(false)),
                    _ => Err("Right is not a number"),
                },
            },
            Err(err) => return Err(err),
        }
    }
    pub fn visit_literal_expr(&self, lit: expr::Literal) -> Value {
        match lit {
            expr::Literal::Number(num) => Value::Number(num),
            expr::Literal::String(str_val) => Value::String(str_val),
            expr::Literal::True => Value::Bool(true),
            expr::Literal::False => Value::Bool(false),
            _ => Value::Nil,
        }
    }
}