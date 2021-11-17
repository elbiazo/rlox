use crate::expr;
use crate::scanner::Token;
use std::io;
pub struct Parser {
    tokens: Vec<Token>,
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
    pub fn new(tokens: Vec<Token>) -> Parser {
        println!("{:?}", tokens);
        Parser { tokens: tokens }
    }

    pub fn parse_tokens(&self) -> Result<(), io::Error> {
        Ok(())
    }
}
