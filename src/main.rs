use std::env;
use std::error::Error;
use std::fs;

mod ast;
mod interpreter;
#[allow(dead_code)]
mod lexer;
mod parser;
mod token;

use crate::ast::*;
use interpreter::*;
use parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let code = fs::read_to_string(path).unwrap();

    println!("Parsing code: {}", code);
    let parsed: Stmt = Parser::parse_code(code);

    println!("Interpreting.");
    interpret(parsed);

    Ok(())
}
