extern crate clap;
use clap::{Arg, App};
use std::fs::read;
use std::io;
use std::io::Result;
use std::io::prelude::*;

fn run(bytes: Vec<u8>) -> Result<()> {
    println!("{:?}", bytes);
    Ok(())
}
fn run_file(path: &str) -> Result<()>{
    let bytes = read(path)?;
    run(bytes)
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        run(line.unwrap().as_bytes().to_vec()).unwrap();
    }
}
fn main() {
    let matches = App::new("rlox")
    .version("1.0")
    .about("Interpreter for rlox language")
    .arg(Arg::with_name("SCRIPT")
         .help("Script for interpreter"))
    .get_matches();

    if matches.is_present("SCRIPT") {
        let path = matches.value_of("SCRIPT").unwrap();
        run_file(path).unwrap();
    } else {
        run_prompt();
    }
}
