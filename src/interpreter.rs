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
            _ => Err("Not supported visit"),
        }
    }

    pub fn visit_unary_expr(&self, op: scanner::Token, e: expr::Expr) -> Result<Value, &str> {
        let right = match self.visit_expr(e) {
            Ok(val) => {
                match val {
                    Value::Number(num) => num,
                    _ => return Err("Right is not a number"),
                }
            },
            Err(err) => return Err(err),
        };

        match op.tok_type {
            scanner::TokenType::Minus => Ok(Value::Number(-right)),
            scanner::TokenType::Plus => Ok(Value::Number(right)),
            _ => Err("Op type is not minus or plus"),
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
