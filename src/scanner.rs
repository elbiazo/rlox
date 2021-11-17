use crate::logger::{Error, Report};
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::SemiColon => write!(f, "SemiColon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),

            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),

            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),

            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::For => write!(f, "For"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),

            TokenType::Eof => write!(f, "Eof"),
        }
    }
}

pub struct Token {
    tok_type: TokenType,
    lexme: String,
    literal: Literal,
    line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "type: {:?}\tlexme: {}\tliteral: {:?}\tline: {}",
            self.tok_type, self.lexme, self.literal, self.line
        )
    }
}
impl Token {
    pub fn new(tok_type: TokenType, lexme: String, literal: Literal, line: usize) -> Token {
        Token {
            tok_type,
            lexme,
            literal,
            line,
        }
    }
}
pub enum Literal {
    Null(Option<()>),
    Str(String),
    Int(f64),
}
impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Null(_) => write!(f, "None"),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Int(i) => write!(f, "{}", i),
        }
    }
}

impl Report for Scanner {
    fn report(&self, err: &Error) {
        println!("[{}] Error: {}", err.line, err.msg);
    }
}

pub struct Scanner {
    pub source: String,
    current: usize,
    start: usize,
    line: usize,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    err: Option<Error>,
}



impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
            err: None,
        }
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("Index does not exist for source");
        self.current += 1;
        c
    }

    fn add_token(&mut self, tok_type: TokenType) {
        self.add_token_lit(tok_type, Literal::Null(None));
    }

    fn add_token_lit(&mut self, tok_type: TokenType, lit: Literal) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens.push(Token::new(tok_type, text, lit, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),

            // Operators
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            // Operator
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.err = Some(Error {
                        line: self.line,
                        msg: "Unimplemented token".to_string(),
                    });

                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = String::from(&self.source[self.start..self.current]);
        let tok_type: TokenType;
        match self.keywords.get(text.as_str()) {
            Some(t_type) => tok_type = *t_type,
            None => tok_type = TokenType::Identifier,
        }
        self.add_token(tok_type);
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token_lit(
            TokenType::Number,
            Literal::Int(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .expect("Not a number"),
            ),
        );
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.err = Some(Error{
                line: self.line,
                msg: "Unterminated string".to_string(),
            });
        }
        self.advance();

        let value: Literal =
            Literal::Str(String::from(&self.source[self.start + 1..self.current - 1]));
        self.add_token_lit(TokenType::String, value);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }
    pub fn scan_tokens(&mut self) -> Result<(),()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        match &self.err {
            Some(err) => {
                self.report(err);
                return Err(());
            }
            None => {
                self.tokens.push(Token::new(
                    TokenType::Eof,
                    String::from(""),
                    Literal::Null(None),
                    self.line,
                ));
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }
}
