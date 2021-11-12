use std::fs::read;
use std::io;
use std::io::prelude::*;
use std::io::Result;
mod logger;
mod scanner;
pub struct Lox {
    had_err: bool,
}

impl Lox {
    pub fn new() -> Lox {
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
        let error = logger::Error { line, err, msg };
        error.report();
    }
}
