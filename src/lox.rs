use std::fs::read;
use std::io;
use std::io::prelude::*;
use std::io::Result;
use crate::logger::Error;
use crate::scanner::Scanner;
pub struct Lox {
    had_err: Error,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_err: Error {
            line: 0,
            msg: String::new()
        } }
    }

    pub fn run(&self, source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
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

}
