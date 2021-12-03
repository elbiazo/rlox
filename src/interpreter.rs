use crate::environment::Environment;
use crate::expr;
use crate::scanner;
use log::info;
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
            env: Environment::new(None),
        }
    }

    pub fn evaluate(&mut self, expr: expr::Expr) -> Result<Value, Error> {
        match expr {
            expr::Expr::Literal(lit) => Ok(self.visit_literal_expr(lit)),
            expr::Expr::Grouping(e) => self.evaluate(*e),
            expr::Expr::Unary(op, e) => self.visit_unary_expr(op.clone(), *e),
            expr::Expr::Binary(left, op, right) => self.visit_binary_expr(*left, op, *right),
            expr::Expr::Identifier(tok) => self.visit_identifier_expr(tok),
            expr::Expr::Assign(tok, e) => self.visit_assign_expr(tok, *e),
            expr::Expr::Logical(left, op, right) => self.visit_logical_expr(*left, op, *right),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left: expr::Expr,
        op: scanner::Token,
        right: expr::Expr,
    ) -> Result<Value, Error> {
        match self.evaluate(left) {
            Ok(val) => match val {
                // Checking Number op Number
                Value::Number(left_val) => match self.evaluate(right) {
                    Ok(val) => match val {
                        Value::Number(right_val) => match op.tok_type {
                            scanner::TokenType::Plus => Ok(Value::Number(left_val + right_val)),
                            scanner::TokenType::Minus => Ok(Value::Number(left_val - right_val)),
                            scanner::TokenType::Slash => Ok(Value::Number(left_val / right_val)),
                            scanner::TokenType::Star => Ok(Value::Number(left_val * right_val)),
                            scanner::TokenType::Modulo => Ok(Value::Number(left_val % right_val)),
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
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unsuppored binary expr"),
                                ))
                            }
                        },
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Binary expr needs f64"),
                            ))
                        }
                    },
                    Err(msg) => return Err(msg),
                },

                // Checking String + String
                Value::String(left_val) => match self.evaluate(right) {
                    Ok(val) => match val {
                        Value::String(right_val) => match op.tok_type {
                            scanner::TokenType::Plus => {
                                Ok(Value::String(format!("{}{}", left_val, right_val)))
                            }
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unsuppored binary expr"),
                                ))
                            }
                        },
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Binary expr needs String"),
                            ))
                        }
                    },
                    Err(msg) => return Err(msg),
                },

                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Binary expr needs f64"),
                    ))
                }
            },
            Err(msg) => return Err(msg),
        }
    }

    pub fn visit_identifier_expr(&self, op: scanner::Token) -> Result<Value, Error> {
        info!("\n\tenv: {:?}", self.env);
        self.env.get(op)
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Bool(boolean) => {
                if boolean {
                    return true;
                } else {
                    return false;
                }
            }
            Value::Nil => return false,
            _ => return true,
        }
    }
    pub fn visit_if_stmt(
        &mut self,
        condition: expr::Expr,
        then_branch: expr::Stmt,
        else_branch: Option<Box<expr::Stmt>>,
    ) -> Result<(), Error> {
        let cond_val = match self.evaluate(condition) {
            Ok(val) => val,
            Err(msg) => return Err(Error::new(ErrorKind::Other, msg)),
        };

        if self.is_truthy(cond_val) {
            self.execute(then_branch)?;
        } else {
            match else_branch {
                Some(else_stmt) => {
                    self.execute(*else_stmt)?;
                    return Ok(());
                }
                None => return Ok(()),
            }
        }
        Ok(())
    }

    pub fn visit_unary_expr(&mut self, op: scanner::Token, e: expr::Expr) -> Result<Value, Error> {
        match self.evaluate(e) {
            Ok(val) => match val {
                Value::Number(num) => match op.tok_type {
                    scanner::TokenType::Minus => Ok(Value::Number(-num)),
                    scanner::TokenType::Plus => Ok(Value::Number(num)),
                    scanner::TokenType::Bang => Ok(Value::Bool(false)),
                    _ => Err(Error::new(
                        ErrorKind::Other,
                        format!("Op type is not minus or plus"),
                    )),
                },
                _ => match op.tok_type {
                    scanner::TokenType::Bang => Ok(Value::Bool(false)),
                    _ => Err(Error::new(
                        ErrorKind::Other,
                        format!("Right is not a number"),
                    )),
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
        let value = self.evaluate(expr);
        match value {
            Ok(val) => match val {
                Value::String(string) => println!("{}", string),
                Value::Number(num) => println!("{}", num),
                Value::Bool(boolean) => println!("{}", boolean),
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("print visitor needs string not other value. recv: {val:?}"),
                    ))
                }
            },
            Err(msg) => return Err(Error::new(ErrorKind::Other, msg)),
        }
        Ok(())
    }
    fn visit_var_stmt(&mut self, name: String, expr: expr::Expr) -> Result<(), Error> {
        let value = match self.evaluate(expr) {
            Ok(val) => val,
            Err(msg) => return Err(Error::new(ErrorKind::Other, msg)),
        };

        self.env.define(name, value);
        Ok(())
    }
    fn visit_assign_expr(&mut self, tok: scanner::Token, e: expr::Expr) -> Result<Value, Error> {
        let value = match self.evaluate(e) {
            Ok(val) => val,
            Err(msg) => return Err(msg),
        };
        self.env.assign(tok, value.clone())?;
        Ok(value.clone())
    }
    fn visit_block_stmt(&mut self, statements: Vec<expr::Stmt>) -> Result<(), Error> {
        let prev_env = self.env.clone();
        self.env = Environment::new(Some(Box::new(prev_env.clone())));
        for stmt in statements {
            match self.execute(stmt) {
                Err(err) => {
                    self.env = prev_env;
                    return Err(err);
                }
                _ => (),
            }
        }
        match self.env.enclosing.clone() {
            Some(enc) => self.env = *enc,
            None => (),
        }
        Ok(())
    }

    fn visit_logical_expr(
        &mut self,
        left: expr::Expr,
        op: scanner::Token,
        right: expr::Expr,
    ) -> Result<Value, Error> {
        let left = self.evaluate(left)?;

        if op.tok_type == scanner::TokenType::Or {
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(left.clone()) {
                return Ok(left);
            }
        }
        return Ok(self.evaluate(right)?);
    }

    fn visit_while(&mut self, condition: expr::Expr, body: expr::Stmt) -> Result<(), Error> {
        let mut cond = self.evaluate(condition.clone())?;
        while self.is_truthy(cond.clone()) {
            self.execute(body.clone())?;
            cond = self.evaluate(condition.clone())?;
        }
        Ok(())
    }
    pub fn execute(&mut self, stmt: expr::Stmt) -> Result<(), Error> {
        match stmt {
            expr::Stmt::Print(expr) => self.visit_print_stmt(expr),
            expr::Stmt::Var(name, expr) => self.visit_var_stmt(name, expr),
            expr::Stmt::Block(exprs) => self.visit_block_stmt(exprs),
            expr::Stmt::If(condition, then_branch, else_branch) => {
                self.visit_if_stmt(condition, *then_branch, else_branch)
            }
            expr::Stmt::While(condition, body) => self.visit_while(condition, *body),
            expr::Stmt::Expr(expr) => match expr {
                expr::Expr::Assign(tok, e) => {
                    match self.visit_assign_expr(tok, *e) {
                        Err(msg) => return Err(msg),
                        _ => (),
                    }
                    Ok(())
                }
                _ => Err(Error::new(ErrorKind::Other, "Unimplemented STMT")),
            },
        }
    }
}
