use crate::scanner::Token;
use std::io;

pub struct Parser {
    tokens: Vec<Token>,
}

/*
Recursive descent using the following grammar

program     → declaration* EOF ;

declaration → classDecl
            | funDecl
            | varDecl
            | statement ;

classDecl → "class" IDENTIFIER ( "<" IDENTIFIER )?
            "{" function* "}" ;

funDecl  → "fun" function ;
function → IDENTIFIER "(" parameters? ")" block ;
parameters  → IDENTIFIER ( "," IDENTIFIER )* ;

statement → exprStmt
          | forStmt
          | ifStmt
          | printStmt
          | returnStmt
          | whileStmt
          | block ;

returnStmt → "return" expression? ";" ;

forStmt   → "for" "(" ( varDecl | exprStmt | ";" )
                      expression? ";"
                      expression? ")" statement ;

whileStmt → "while" "(" expression ")" statement ;

ifStmt    → "if" "(" expression ")" statement ( "else" statement )? ;

block     → "{" declaration* "}" ;

varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;

exprStmt  → expression ";" ;
printStmt → "print" expression ";" ;

expression → assignment ;
assignment → ( call "." )? IDENTIFIER "=" assignment
           | logic_or;
logic_or   → logic_and ( "or" logic_and )* ;
logic_and  → equality ( "and" equality )* ;

equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
multiplication → unary ( ( "/" | "*" ) unary )* ;
unary → ( "!" | "-" ) unary | call ;
call → primary ( "(" arguments? ")" | "." IDENTIFIER | "[" expression "]" )* ;
arguments → expression ( "," expression )* ;

primary → "true" | "false" | "nil" | "this"
        | NUMBER | STRING | IDENTIFIER | "(" expression ")"
        | "super" "." IDENTIFIER
        | "[" arguments? "]" ;

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
