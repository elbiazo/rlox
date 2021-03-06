use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use log::{error, info};
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

        // info!("\n\ttokens: {:?}", scanner.tokens);

        let mut parser = Parser::new(scanner.tokens);
        let stmts = match parser.parse() {
            Ok(stmts) => stmts,
            Err(err_msg) => {
                error!("{}", err_msg);
                return Ok(());
            }
        };
        // info!("\n\tstmts: {:?}", stmts);

        let mut interp = Interpreter::new();
        for stmt in stmts {
            match interp.execute(stmt) {
                Err(err_msg) => {
                    error!("{}", err_msg)
                }
                _ => (),
            }
        }
        info!("\n\tenv: {:?}", interp.env);

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
