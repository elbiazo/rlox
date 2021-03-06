use crate::expr;

use crate::scanner;

use std::io;
pub struct Parser {
    current: usize,
    tokens: Vec<scanner::Token>,
    pub exprs: Vec<expr::Expr>,
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
            exprs: Vec::new(),
        }
    }

    fn peek(&self) -> &scanner::Token {
        self.tokens.get(self.current).unwrap()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().tok_type == scanner::TokenType::Eof
    }

    fn check(&self, _ty: scanner::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().tok_type == _ty
    }

    fn previous(&self) -> scanner::Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn advance(&mut self) -> scanner::Token {
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
                return true;
            }
        }
        false
    }

    fn consume(&mut self, ty: scanner::TokenType, msg: &str) -> Result<scanner::Token, io::Error> {
        if self.check(ty) {
            return Ok(self.advance());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}. received {:?}", msg, ty),
            ));
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
            return Ok(expr::Expr::Literal(self.previous().literal.clone()));
        }
        if self.match_one_of(vec![scanner::TokenType::Identifier]) {
            return Ok(expr::Expr::Identifier(self.previous()));
        }
        if self.match_one_of(vec![scanner::TokenType::LeftParen]) {
            let expr = Box::new(self.expression()?);
            self.consume(
                scanner::TokenType::RightParen,
                "Expect ')' after expression.",
            )?;
            return Ok(expr::Expr::Grouping(expr));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid expression: {:?}", self.peek().tok_type),
            ));
        }
    }

    fn unary(&mut self) -> Result<expr::Expr, io::Error> {
        if self.match_one_of(vec![scanner::TokenType::Minus, scanner::TokenType::Bang]) {
            let op = self.previous().clone();
            let right = Box::new(self.unary()?);

            return Ok(expr::Expr::Unary(op, right));
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<expr::Expr, io::Error> {
        let mut expr = self.unary()?;

        while self.match_one_of(vec![
            scanner::TokenType::Slash,
            scanner::TokenType::Star,
            scanner::TokenType::Modulo,
        ]) {
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
    fn or(&mut self) -> Result<expr::Expr, io::Error> {
        let mut expr = self.and()?;
        while self.match_one_of(vec![scanner::TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = expr::Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }
    fn and(&mut self) -> Result<expr::Expr, io::Error> {
        let mut expr = self.equality()?;

        while self.match_one_of(vec![scanner::TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = expr::Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<expr::Expr, io::Error> {
        let expr = self.or()?;
        if self.match_one_of(vec![scanner::TokenType::Equal]) {
            let value = self.assignment()?;

            match expr {
                expr::Expr::Identifier(tok) => return Ok(expr::Expr::Assign(tok, Box::new(value))),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to do assignment it is not identifier",
                    ))
                }
            }
        } else {
            return Ok(expr);
        }
    }
    fn expression(&mut self) -> Result<expr::Expr, io::Error> {
        self.assignment()
    }
    fn expression_statement(&mut self) -> Result<expr::Stmt, io::Error> {
        let expr = self.expression()?;
        self.consume(scanner::TokenType::SemiColon, "Expected ; after value")?;
        Ok(expr::Stmt::Expr(expr))
    }
    fn print_statement(&mut self) -> Result<expr::Stmt, io::Error> {
        let expr = self.expression()?;
        self.consume(scanner::TokenType::SemiColon, "Expected ; after value")?;
        Ok(expr::Stmt::Print(expr))
    }

    fn block_stmt(&mut self) -> Result<expr::Stmt, io::Error> {
        let mut statements = Vec::new();

        while !self.check(scanner::TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(scanner::TokenType::RightBrace, "Expected } after block")?;
        Ok(expr::Stmt::Block(statements))
    }
    fn if_stmt(&mut self) -> Result<expr::Stmt, io::Error> {
        self.consume(scanner::TokenType::LeftParen, "Expected '(' after if")?;
        let condition = self.expression()?;
        self.consume(scanner::TokenType::RightParen, "exprected ')' after if")?;

        let then_branch = Box::new(self.statement()?);

        let mut else_branch = None;
        if self.match_one_of(vec![scanner::TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?))
        }

        Ok(expr::Stmt::If(condition, then_branch, else_branch))
    }

    fn while_stmt(&mut self) -> Result<expr::Stmt, io::Error> {
        self.consume(scanner::TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(scanner::TokenType::RightParen, "Expect ')' after 'while'.")?;

        let body = self.statement()?;

        Ok(expr::Stmt::While(condition, Box::new(body)))
    }

    fn for_stmt(&mut self) -> Result<expr::Stmt, io::Error> {
        self.consume(scanner::TokenType::LeftParen, "Expected ( after for.")?;

        let mut maybe_initializer: Option<expr::Stmt> = None;
        if self.matches(scanner::TokenType::SemiColon) {
        } else if self.matches(scanner::TokenType::Var) {
            maybe_initializer = Some(self.var_declaration()?)
        } else {
            maybe_initializer = Some(self.expression_statement()?)
        }
        let maybe_initializer = maybe_initializer;

        let mut maybe_condition: Option<expr::Expr> = None;
        if !self.check(scanner::TokenType::SemiColon) {
            maybe_condition = Some(self.expression()?)
        }
        let maybe_condition = maybe_condition;

        self.consume(
            scanner::TokenType::SemiColon,
            "Expected ; after loop condition",
        )?;

        let maybe_increment = if !self.check(scanner::TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            scanner::TokenType::RightParen,
            "Expected ) after for clauses",
        )?;

        let mut body = self.statement()?;

        if let Some(increment) = maybe_increment {
            body = expr::Stmt::Block(vec![body, expr::Stmt::Expr(increment)])
        }

        let condition = match maybe_condition {
            Some(cond) => cond,
            None => expr::Expr::Literal(expr::Literal::True),
        };
        body = expr::Stmt::While(condition, Box::new(body));

        if let Some(initializer) = maybe_initializer {
            body = expr::Stmt::Block(vec![initializer, body])
        }
        let body = body;

        Ok(body)
    }

    fn statement(&mut self) -> Result<expr::Stmt, io::Error> {
        if self.match_one_of(vec![scanner::TokenType::Print]) {
            return self.print_statement();
        } else if self.match_one_of(vec![scanner::TokenType::Var]) {
            return self.var_declaration();
        } else if self.match_one_of(vec![scanner::TokenType::LeftBrace]) {
            return self.block_stmt();
        } else if self.match_one_of(vec![scanner::TokenType::If]) {
            return self.if_stmt();
        } else if self.match_one_of(vec![scanner::TokenType::While]) {
            return self.while_stmt();
        } else if self.match_one_of(vec![scanner::TokenType::For]) {
            return self.for_stmt();
        }

        self.expression_statement()
    }
    fn var_declaration(&mut self) -> Result<expr::Stmt, io::Error> {
        let name = self.consume(
            scanner::TokenType::Identifier,
            "Expected Identifier in var decl",
        )?;
        let initializer = match self.match_one_of(vec![scanner::TokenType::Equal]) {
            true => self.expression()?,
            false => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Var decl requires it to match to equals. Got {name:?}"),
                ))
            }
        };

        self.consume(scanner::TokenType::SemiColon, "Expected ; after var decl")?;

        Ok(expr::Stmt::Var(name.lexme, initializer))
    }

    pub fn parse(&mut self) -> Result<Vec<expr::Stmt>, io::Error> {
        let mut stmts = Vec::<expr::Stmt>::new();
        while !self.is_at_end() {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }
}
