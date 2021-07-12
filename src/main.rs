use std::env;
use std::error::Error;
use std::fs;

mod ast;
mod interpreter;
#[allow(dead_code)]
mod lexer;
mod parser;
mod scope;
#[allow(dead_code)]
mod token;

use crate::ast::*;
use interpreter::*;
use parser::*;
use scope::Scope;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let code = fs::read_to_string(path).unwrap();

    let parsed: Program = parse_code(code);

    interpret_tree(parsed, &mut Scope::new(None));

    Ok(())
}
