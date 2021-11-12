use std::fs::read;
use std::io;
use std::io::prelude::*;
use std::io::Result;
mod logger;
mod scanner;
pub struct Lox {
    had_err: bool,
}

pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF,
}
// pub struct Token {
//     TokenType: 
// }

impl Lox {
    pub fn new() -> Self {
        Lox { had_err: false }
    }

    pub fn run(&self, source: String) -> Result<()> {
        let scanner = scanner::Scanner::new(source);
        scanner.scan_tokens();
        Ok(())
    }
    pub fn run_file(&self, path: &str) -> Result<()> {
        let source = String::from_utf8(read(path)?).expect("Found invalid UTF-8");
        self.run(source)
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            self.run(line.unwrap()).unwrap();
        }
    }

    fn error(&mut self, line: u32, err: String, msg: String) {
        self.had_err = true;
        let error = logger::Error {
            line: line,
            err: err,
            msg: msg,
        };
        error.report();
    }
}
