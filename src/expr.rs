use crate::scanner;
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Unary(scanner::Token, Box<Expr>),
    Binary(Box<Expr>, scanner::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Identifier(scanner::Token)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Var(String, Expr),
}
