use crate::scanner;
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Unary(scanner::Token, Box<Expr>),
    Binary(Box<Expr>, scanner::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Identifier(scanner::Token),
    Assign(scanner::Token, Box<Expr>),
    Logical(Box<Expr>, scanner::Token, Box<Expr>),
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
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
}
