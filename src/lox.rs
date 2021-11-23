use crate::parser::Parser;
use crate::scanner::Scanner;
use log::error;
use std::fs::read;
use std::io;
use std::io::prelude::*;
use std::io::Result;
pub struct Lox;

impl Lox {
    pub fn new() -> Lox {
        Lox
    }

    pub fn run(&self, source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
        match scanner.scan_tokens() {
            Err(err_msg) => {
                error!("{}", err_msg);
                return Ok(());
            }
            _ => (),
        }

        // info!("{:?}", scanner.tokens);

        let mut parser = Parser::new(scanner.tokens);
        parser.parse_tokens().unwrap();
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
}
