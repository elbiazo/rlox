extern crate clap;
use clap::{App, Arg};

mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod environment;

fn main() {
    env_logger::init();
    let matches = App::new("rlox")
        .version("1.0")
        .about("Interpreter for rlox language")
        .arg(Arg::with_name("SCRIPT").help("Script for interpreter"))
        .get_matches();

    let mut lox = lox::Lox::new();
    if matches.is_present("SCRIPT") {
        let path = matches.value_of("SCRIPT").unwrap();
        lox.run_file(path).unwrap();
    } else {
        lox.run_prompt();
    }
}
