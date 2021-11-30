use crate::environment::Environment;
use crate::expr;
use crate::scanner;
pub struct Interpreter {
    pub env: Environment,
}
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn visit_expr(&mut self, expr: expr::Expr) -> Result<Value, String> {
        match expr {
            expr::Expr::Literal(lit) => Ok(self.visit_literal_expr(lit)),
            expr::Expr::Grouping(e) => self.visit_expr(*e),
            expr::Expr::Unary(op, e) => self.visit_unary_expr(op.clone(), *e),
            expr::Expr::Binary(left, op, right) => self.visit_binary_expr(*left, op, *right),
            expr::Expr::Identifier(tok) => self.visit_identifier_expr(tok),
            expr::Expr::Assign(tok, e) => self.visit_assign_expr(tok, *e),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left: expr::Expr,
        op: scanner::Token,
        right: expr::Expr,
    ) -> Result<Value, String> {
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
                            _ => return Err(format!("Unsuppored binary expr")),
                        },
                        _ => return Err(format!("Binary expr needs f64")),
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
                            _ => return Err(format!("Unsuppored binary expr")),
                        },
                        _ => return Err(format!("Binary expr needs String")),
                    },
                    Err(msg) => return Err(msg),
                },

                _ => return Err(format!("Binary expr needs f64")),
            },
            Err(msg) => return Err(msg),
        }
    }

    pub fn visit_identifier_expr(&self, op: scanner::Token) -> Result<Value, String> {
        self.env.get(op)
    }

    pub fn visit_unary_expr(&mut self, op: scanner::Token, e: expr::Expr) -> Result<Value, String> {
        match self.visit_expr(e) {
            Ok(val) => match val {
                Value::Number(num) => match op.tok_type {
                    scanner::TokenType::Minus => Ok(Value::Number(-num)),
                    scanner::TokenType::Plus => Ok(Value::Number(num)),
                    scanner::TokenType::Bang => Ok(Value::Bool(false)),
                    _ => Err(format!("Op type is not minus or plus")),
                },
                _ => match op.tok_type {
                    scanner::TokenType::Bang => Ok(Value::Bool(false)),
                    _ => Err(format!("Right is not a number")),
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

    fn visit_print_stmt(&mut self, expr: expr::Expr) -> Result<(), Error> {
        let value = self.visit_expr(expr);
        match value {
            Ok(val) => match val {
                Value::String(string) => println!("{}", string),
                Value::Number(num) => println!("{}", num),
                Value::Bool(boolean) => println!("{}", boolean),
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "print visitor needs string not other value",
                    ))
                }
            },
            Err(msg) => return Err(Error::new(ErrorKind::Other, msg)),
        }
        Ok(())
    }
    fn visit_var_stmt(&mut self, name: String, expr: expr::Expr) -> Result<(), Error> {
        let value = match self.visit_expr(expr) {
            Ok(val) => val,
            Err(msg) => return Err(Error::new(ErrorKind::Other, msg)),
        };

        self.env.define(name, value);
        Ok(())
    }
    fn visit_assign_expr(&mut self, tok: scanner::Token, e: expr::Expr) -> Result<Value, String> {
        let value = match self.visit_expr(e) {
            Ok(val) => val,
            Err(msg) => return Err(msg),
        };
        self.env.assign(tok, value.clone())?;
        Ok(value.clone())
    }

    pub fn visit_stmt(&mut self, stmt: expr::Stmt) -> Result<(), Error> {
        match stmt {
            expr::Stmt::Print(expr) => self.visit_print_stmt(expr),
            expr::Stmt::Var(name, expr) => self.visit_var_stmt(name, expr),
            expr::Stmt::Expr(expr) => match expr {
                expr::Expr::Assign(tok, e) => {
                    match self.visit_assign_expr(tok, *e) {
                        Err(msg) => return Err(Error::new(ErrorKind::Other, msg)),
                        _ => (),
                    }
                    Ok(())
                }
                _ => Err(Error::new(ErrorKind::Other, "Unimplemented STMT")),
            },
        }
    }
}
