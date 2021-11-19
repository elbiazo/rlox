use crate::expr;
use crate::scanner;
use std::io;
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
    fn peek(&self) -> Result<scanner::Token, io::Error> {
        match self.tokens.get(self.current) {
            Some(token) => Ok(*token),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unexpected end of input",
            )),
        }
    }

    fn is_at_end(&self) -> Result<bool, io::Error> {
        match self.peek() {
            Ok(token) => Ok(matches!(token.tok_type, scanner::TokenType::Eof)),
            Err(err_msg) => Err(err_msg),
        }
    }

    fn check(&self, token: scanner::TokenType) -> Result<bool, io::Error> {
        self.is_at_end()?;

        match self.peek() {
            Ok(cur_token) => Ok(matches!(cur_token.tok_type, token)),
            Err(err_msg) => Err(err_msg),
        }
    }

    fn previous(&self) -> Result<scanner::Token, io::Error> {
        match self.tokens.get(self.current - 1) {
            Some(token) => Ok(*token),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unexpected end of input",
            )),
        }
    }
    fn advance(&mut self) -> Result<scanner::Token, io::Error> {
        match self.is_at_end() {
            Ok(is_end) => {
                if !is_end {
                    self.current += 1;
                    Ok(self.previous()?)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Unexpected end of input",
                    ))
                }
            }
            Err(err_msg) => Err(err_msg),
        }
    }

    fn match_one_of(&self, token_types: Vec<scanner::TokenType>) -> Result<bool, io::Error> {
        for token_type in token_types {
            match self.check(token_type) {
                Ok(is_match) => {
                    if is_match {
                        return Ok(true);
                    } 
                }
                Err(err_msg) => return Err(err_msg),
            }
        }

        Ok(false)
    }

    fn equality(&mut self) -> Result<expr::Expr, Error> {
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

    fn expression(&self) -> Result<expr::Expr, Error> {
        self.equality()
    }

    pub fn parse_tokens(&self) -> Result<(), io::Error> {
        Ok(())
    }
}
