use crate::scanner;
use crate::expr;
use std::io::Error;
pub struct Parser {
    current: usize,
    tokens: Vec<scanner::Token>,
}
/*
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
*/
impl Parser {
    pub fn new(tokens: Vec<scanner::Token>) -> Parser {
        Parser {
            current: 0,
            tokens: tokens,
        }
    }

    fn equality(&mut self) -> Result<expr::Expr, Error>{
        let mut expr = self.comparison()?;

        while self.match_one_of(vec![
            scanner::TokenType::BangEqual,
            scanner::TokenType::EqualEqual,
        ]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.comparison()?);

            let binop_maybe = self.op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let lef = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn expression(&self) {
        self.equality()
    }

    pub fn parse_tokens(&self) -> Result<(), io::Error> {
        Ok(())
    }
}
