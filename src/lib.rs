use std::error::Error;
use std::fs;

mod ast;
pub mod interpreter;
#[allow(dead_code)]
mod lexer;
pub mod parser;
pub mod scope;
#[allow(dead_code)]
mod token;

use interpreter::*;
use parser::*;
use scope::Scope;

pub fn main(path: &str) -> Result<(), Box<dyn Error>> {
    let code = fs::read_to_string(path).unwrap();
    let parsed = parse_code(code);
    interpret_tree(parsed, &mut Scope::new(None));
    Ok(())
}
