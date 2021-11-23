use crate::expr;
use crate::scanner;
use log::{error, info};
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

    fn peek(&self) -> &scanner::Token {
        self.tokens.get(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().tok_type, scanner::TokenType::Eof)
    }

    fn check(&self, _ty: scanner::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        matches!(self.peek().tok_type, _ty)
    }

    fn previous(&self) -> &scanner::Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn advance(&mut self) -> &scanner::Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    fn matches(&mut self, ty: scanner::TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            return true;
        }
        false
    }

    fn match_one_of(&mut self, types: Vec<scanner::TokenType>) -> bool {
        for ty in types {
            if self.matches(ty) {
                error!("match_one_of: {:?}", ty);
                return true;
            }
        }
        false
    }

    fn consume(&mut self, ty: scanner::TokenType, msg: &str) {
        if self.check(ty) {
            self.advance();
        } else {
            error!("{}", msg);
        }
    }

    fn primary(&mut self) -> Result<expr::Expr, io::Error> {
        if self.match_one_of(vec![scanner::TokenType::False]) {
            return Ok(expr::Expr::Literal(expr::Literal::False));
        }
        if self.match_one_of(vec![scanner::TokenType::True]) {
            return Ok(expr::Expr::Literal(expr::Literal::True));
        }

        if self.match_one_of(vec![scanner::TokenType::Nil]) {
            return Ok(expr::Expr::Literal(expr::Literal::Nil));
        }

        if self.match_one_of(vec![scanner::TokenType::Number, scanner::TokenType::String]) {
            error!("TESTSE");
            return Ok(expr::Expr::Literal(self.previous().literal.clone()));
        }
        if self.match_one_of(vec![scanner::TokenType::LeftParen]) {
            let expr = Box::new(self.expression()?);
            self.consume(
                scanner::TokenType::RightParen,
                "Expect ')' after expression.",
            );
            return Ok(expr::Expr::Grouping(expr));
        } else {
            return Ok(expr::Expr::Literal(expr::Literal::Nil));
        }
    }

    fn unary(&mut self) -> Result<expr::Expr, io::Error> {
        println!("peek {:?}", self.peek().tok_type);
        if self.match_one_of(vec![scanner::TokenType::Minus, scanner::TokenType::Bang]) {
            let op = self.previous().clone();
            let right = Box::new(self.unary()?);

            return Ok(expr::Expr::Unary(op, right));
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<expr::Expr, io::Error> {
        let mut expr = self.unary()?;

        while self.match_one_of(vec![scanner::TokenType::Slash, scanner::TokenType::Star]) {
            let op = self.previous().clone();
            let right = Box::new(self.unary()?);
            let left = Box::new(expr);
            expr = expr::Expr::Binary(left, op, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<expr::Expr, io::Error> {
        let mut expr = self.factor()?;

        while self.match_one_of(vec![scanner::TokenType::Minus, scanner::TokenType::Plus]) {
            let op = self.previous().clone();
            let right = Box::new(self.factor()?);
            let left = Box::new(expr);
            expr = expr::Expr::Binary(left, op, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<expr::Expr, io::Error> {
        // let mut expr = self.addition()?;
        let mut expr = self.term()?;

        while self.match_one_of(vec![
            scanner::TokenType::Greater,
            scanner::TokenType::GreaterEqual,
            scanner::TokenType::Less,
            scanner::TokenType::LessEqual,
        ]) {
            let operator_token = self.previous().clone();
            // let right = Box::new(self.addition()?);
            let right = Box::new(self.term()?);

            let left = Box::new(expr.clone());
            expr = expr::Expr::Binary(left, operator_token, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<expr::Expr, io::Error> {
        let mut expr = self.comparison()?;
        while self.match_one_of(vec![
            scanner::TokenType::EqualEqual,
            scanner::TokenType::BangEqual,
        ]) {
            let op = self.previous().clone();
            let right = Box::new(self.comparison()?);
            let left = Box::new(expr);
            expr = expr::Expr::Binary(left, op, right);
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<expr::Expr, io::Error> {
        self.equality()
    }

    pub fn parse_tokens(&mut self) -> Result<(), io::Error> {
        info!("{:?}", self.expression()?);
        Ok(())
    }
}
