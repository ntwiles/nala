#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub test); // synthesized by LALRPOP

use std::env;
use std::error::Error;
use std::fs;

mod ast;
mod lexer;
mod parser;
mod token;

use crate::ast::Stmt;
use parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let code = fs::read_to_string(path).unwrap();

    println!("Parsing code: {}", code);

    let test: Stmt = Parser::parse_code(code);

    if let Stmt::Print(message) = test {
        println!("{}", message.to_string());
    }

    Ok(())
}
