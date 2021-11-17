use crate::scanner::Scanner;
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
            Err(()) => return Ok(()),
            _ => (),
        }
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
